use defmt::{debug, error, info};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::{Error, PdError};

use crate::command::*;

use super::Tps6699x;

/// This struct manages the firmware update process and allows updating multiple controllers simultaneously.
// TODO: Make this generic over the controller implementation
// TODO: Create an image struct instead of directly using a byte-slice
pub struct FwUpdater<'a, M: RawMutex, B: I2c, const N: usize> {
    controllers: [&'a mut Tps6699x<'a, M, B>; N],
}

impl<'a, M: RawMutex, B: I2c, const N: usize> FwUpdater<'a, M, B, N> {
    pub fn new(controllers: [&'a mut Tps6699x<'a, M, B>; N]) -> Self {
        Self { controllers }
    }

    async fn enter_fw_update_mode(&mut self) -> Result<(), Error<B::Error>> {
        for (i, controller) in self.controllers.iter_mut().enumerate() {
            info!("Controller {}: Entering FW update mode", i);
            if let Err(e) = controller.fw_update_mode_enter().await {
                info!("Controller {}: Failed to enter FW update mode", i);

                self.exit_fw_update_mode().await?;
                return Err(e);
            }
        }

        Ok(())
    }

    async fn exit_fw_update_mode(&mut self) -> Result<(), Error<B::Error>> {
        for (i, controller) in self.controllers.iter_mut().enumerate() {
            info!("Controller {}: Exiting FW update mode", i);
            if let Err(_) = controller.fw_update_mode_exit().await {
                info!("Controller {}: Failed to exit FW update mode", i);
            }
        }

        Ok(())
    }

    async fn fw_update_init(&mut self, image: &[u8]) -> Result<TfuiArgs, Error<B::Error>> {
        let mut buf = [0u8; PD_FW_HEADER_METADATA_LENGTH as usize];

        // Get TFUi args from image
        if PD_FW_HEADER_METADATA_OFFSET + PD_FW_HEADER_METADATA_LENGTH > image.len() {
            return Err(PdError::InvalidParams.into());
        }

        buf.copy_from_slice(
            &image[PD_FW_HEADER_METADATA_OFFSET as usize
                ..(PD_FW_HEADER_METADATA_OFFSET + PD_FW_HEADER_METADATA_LENGTH) as usize],
        );
        let tfui_args = TfuiArgs::decode_from_slice(&buf).map_err(Error::Pd)?;

        for (i, controller) in self.controllers.iter_mut().enumerate() {
            info!("Controller {}: Initializing FW update", i);

            let result = controller.fw_update_init(&tfui_args).await;
            if result.is_err() || result.unwrap() != ReturnValue::Success {
                info!("Controller {}: Failed to initialize FW update", i);

                self.exit_fw_update_mode().await?;
                return PdError::Failed.into();
            }
        }

        debug!("Transfering header block");
        let header_block = &image
            [PD_FW_HEADER_BLOCK_OFFSET as usize..(PD_FW_HEADER_BLOCK_OFFSET + PD_FW_HEADER_BLOCK_LENGTH) as usize];
        debug!(
            "Header block offset: {:?}, size {}",
            PD_FW_HEADER_BLOCK_OFFSET,
            header_block.len()
        );

        self.burst_write(tfui_args.broadcast_u16_address as u8, header_block)
            .await?;

        Timer::after_millis(TFUI_BURST_WRITE_DELAY_MS).await;

        for (i, controller) in self.controllers.iter_mut().enumerate() {
            info!("Controller {}: Validating header block", i);
            let abort = match controller.fw_update_validate_stream(PD_FW_HEADER_BLOCK_INDEX).await {
                Ok(TfuqBlockStatus::HeaderValidAndAuthentic) => false,
                Ok(r) => {
                    error!("Controller {}: Header block validation failed, result {}", i, r);
                    true
                }
                Err(_) => {
                    error!("Controller {}: Header block validation failed", i);
                    true
                }
            };

            if abort {
                self.exit_fw_update_mode().await?;
                return PdError::Failed.into();
            }
        }

        Ok(tfui_args)
    }

    const fn data_block_metadata_offset(block: usize) -> usize {
        PD_FW_HEADER_BLOCK_OFFSET
            + PD_FW_HEADER_BLOCK_LENGTH
            + (block * (PD_FW_DATA_BLOCK_SIZE + PD_FW_DATA_BLOCK_METADATA_SIZE))
    }

    const fn block_offset(metadata_offset: usize) -> usize {
        metadata_offset + PD_FW_DATA_BLOCK_METADATA_SIZE
    }

    const fn app_config_block_metadata_offset(num_data_blocks: usize, app_size: usize) -> usize {
        app_size
            + PD_FW_IMAGE_ID_LENGTH
            + PD_FW_HEADER_METADATA_LENGTH
            + PD_FW_HEADER_BLOCK_LENGTH
            + num_data_blocks * PD_FW_DATA_BLOCK_METADATA_SIZE
    }

    fn get_image_size(image: &[u8]) -> Result<u32, Error<B::Error>> {
        if image.len() < PD_FW_APP_IMAGE_SIZE_OFFSET + 4 {
            return PdError::InvalidParams.into();
        }

        let image_size_data = &image[PD_FW_APP_IMAGE_SIZE_OFFSET..PD_FW_APP_IMAGE_SIZE_OFFSET + 4];
        Ok(u32::from_le_bytes([
            image_size_data[0],
            image_size_data[1],
            image_size_data[2],
            image_size_data[3],
        ]))
    }

    async fn fw_update_stream_data(
        &mut self,
        image: &[u8],
        metadata_offset: usize,
        metadata_size: usize,
    ) -> Result<(), Error<B::Error>> {
        debug!(
            "Metadata offset: {}, offset: {}",
            metadata_offset,
            Self::block_offset(metadata_offset)
        );
        let arg_bytes = &image[metadata_offset..metadata_offset + metadata_size];

        let args = TfudArgs::decode_from_slice(arg_bytes).map_err(Error::Pd)?;
        debug!("TFUd args: {:?}", args);

        for (i, controller) in self.controllers.iter_mut().enumerate() {
            info!("Controller {}: Streaming data block", i);
            if let Err(_) = controller.fw_update_stream_data(&args).await {
                error!("Controller {}: Failed to stream data block", i);
                self.exit_fw_update_mode().await?;
                return PdError::Failed.into();
            }
        }

        let data_len = args.data_len as usize;
        let block = &image[Self::block_offset(metadata_offset)..Self::block_offset(metadata_offset) + data_len];
        self.burst_write(args.broadcast_u16_address as u8, block).await?;
        Timer::after_millis(TFUD_BURST_WRITE_DELAY_MS).await;

        for (i, controller) in self.controllers.iter_mut().enumerate() {
            info!("Controller {}: Validating header block", i);
            let abort = match controller.fw_update_validate_stream(PD_FW_HEADER_BLOCK_INDEX).await {
                Ok(TfuqBlockStatus::DataValidAndAuthentic) | Ok(TfuqBlockStatus::DataValidButRepeated) => false,
                Ok(r) => {
                    error!("Controller {}: Block validation failed, result {}", i, r);
                    true
                }
                Err(_) => {
                    error!("Controller {}: Block validation failed", i);
                    true
                }
            };

            if abort {
                self.exit_fw_update_mode().await?;
                return PdError::Failed.into();
            }
        }
        Ok(())
    }

    async fn fw_update_load_app_image(&mut self, image: &[u8], num_data_blocks: usize) -> Result<(), Error<B::Error>> {
        for i in 0..num_data_blocks {
            self.fw_update_stream_data(
                image,
                Self::data_block_metadata_offset(i),
                PD_FW_DATA_BLOCK_METADATA_SIZE,
            )
            .await?;
        }

        Ok(())
    }

    async fn fw_update_load_app_config(&mut self, image: &[u8], num_data_blocks: usize) -> Result<(), Error<B::Error>> {
        let app_size = Self::get_image_size(image)? as usize;
        debug!("App size: {}", app_size);
        let metadata_offset = Self::app_config_block_metadata_offset(num_data_blocks, app_size);
        self.fw_update_stream_data(image, metadata_offset, PD_FW_APP_CONFIG_METADATA_SIZE)
            .await
    }

    async fn fw_update_complete(&mut self) -> Result<(), Error<B::Error>> {
        for (i, controller) in self.controllers.iter_mut().enumerate() {
            info!("Controller {}: Completing FW update", i);
            if let Err(_) = controller.fw_update_complete().await {
                error!("Controller {}: Failed to complete FW update", i);
                self.exit_fw_update_mode().await?;
                return PdError::Failed.into();
            }
        }

        Ok(())
    }

    async fn burst_write(&mut self, address: u8, data: &[u8]) -> Result<(), Error<B::Error>> {
        // Controllers are on the same bus, just use the first one
        let mut inner = self.controllers[0].lock_inner().await;
        for chunk in data.chunks(BURST_WRITE_SIZE) {
            inner.bus.write(address, chunk).await.map_err(Error::Bus)?;
        }

        Ok(())
    }

    pub async fn perform_fw_update(&mut self, image: &[u8]) -> Result<(), Error<B::Error>> {
        self.enter_fw_update_mode().await?;
        let tfui_args = self.fw_update_init(image).await?;
        self.fw_update_load_app_image(image, tfui_args.num_data_blocks_tx as usize)
            .await?;
        self.fw_update_load_app_config(image, tfui_args.num_data_blocks_tx as usize)
            .await?;
        self.fw_update_complete().await?;
        self.exit_fw_update_mode().await?;

        Ok(())
    }
}
