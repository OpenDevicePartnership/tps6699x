use defmt::{debug, error, info};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::{Error, PdError};

use crate::command::*;

use super::Tps6699x;

// TODO: Make this generic over the controller implementation
// TODO: Create an image struct instead of directly using a byte-slice

async fn enter_fw_update_mode<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
) -> Result<(), Error<B::Error>> {
    for (i, controller) in controllers.iter_mut().enumerate() {
        info!("Controller {}: Entering FW update mode", i);
        if let Err(e) = controller.fw_update_mode_enter().await {
            info!("Controller {}: Failed to enter FW update mode", i);

            exit_fw_update_mode(controllers).await?;
            return Err(e);
        }
    }

    Ok(())
}

async fn exit_fw_update_mode<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
) -> Result<(), Error<B::Error>> {
    for (i, controller) in controllers.iter_mut().enumerate() {
        info!("Controller {}: Exiting FW update mode", i);
        if let Err(e) = controller.fw_update_mode_exit().await {
            info!("Controller {}: Failed to exit FW update mode", i);
            return Err(e);
        }
    }

    Ok(())
}

async fn fw_update_init<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
    image: &[u8],
) -> Result<TfuiArgs, Error<B::Error>> {
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

    for (i, controller) in controllers.iter_mut().enumerate() {
        info!("Controller {}: Initializing FW update", i);

        let result = controller.fw_update_init(&tfui_args).await;
        if result.is_err() || result.unwrap() != ReturnValue::Success {
            info!("Controller {}: Failed to initialize FW update", i);

            exit_fw_update_mode(controllers).await?;
            return PdError::Failed.into();
        }
    }

    let header_block =
        &image[PD_FW_HEADER_BLOCK_OFFSET as usize..(PD_FW_HEADER_BLOCK_OFFSET + PD_FW_HEADER_BLOCK_LENGTH) as usize];
    debug!(
        "Header block offset: {:?}, size {}",
        PD_FW_HEADER_BLOCK_OFFSET,
        header_block.len()
    );

    info!("Broadcasting header block");
    burst_write(controllers, tfui_args.broadcast_u16_address as u8, header_block).await?;

    Timer::after_millis(TFUI_BURST_WRITE_DELAY_MS).await;

    for (i, controller) in controllers.iter_mut().enumerate() {
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
            exit_fw_update_mode(controllers).await?;
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

fn get_image_size(image: &[u8]) -> Result<u32, PdError> {
    if image.len() < PD_FW_APP_IMAGE_SIZE_OFFSET + 4 {
        return Err(PdError::InvalidParams);
    }

    let image_size_data = &image[PD_FW_APP_IMAGE_SIZE_OFFSET..PD_FW_APP_IMAGE_SIZE_OFFSET + 4];
    Ok(u32::from_le_bytes([
        image_size_data[0],
        image_size_data[1],
        image_size_data[2],
        image_size_data[3],
    ]))
}

// Data block indices start at 1
fn data_block_index_to_block_index(block_index: usize) -> usize {
    block_index + PD_FW_DATA_BLOCK_START_INDEX
}

async fn fw_update_stream_data<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
    image: &[u8],
    block_index: usize,
    metadata_offset: usize,
    metadata_size: usize,
) -> Result<(), Error<B::Error>> {
    debug!(
        "Metadata offset: {}, offset: {}",
        metadata_offset,
        block_offset(metadata_offset)
    );
    let arg_bytes = &image[metadata_offset..metadata_offset + metadata_size];

    let args = TfudArgs::decode_from_slice(arg_bytes).map_err(Error::Pd)?;
    debug!("TFUd args: {:?}", args);

    for (i, controller) in controllers.iter_mut().enumerate() {
        info!("Controller {}: Streaming data block", i);
        if let Err(_) = controller.fw_update_stream_data(&args).await {
            error!("Controller {}: Failed to stream data block", i);
            exit_fw_update_mode(controllers).await?;
            return PdError::Failed.into();
        }
    }

    let data_len = args.data_len as usize;
    let block = &image[block_offset(metadata_offset)..block_offset(metadata_offset) + data_len];
    burst_write(controllers, args.broadcast_u16_address as u8, block).await?;
    Timer::after_millis(TFUD_BURST_WRITE_DELAY_MS).await;

    for (i, controller) in controllers.iter_mut().enumerate() {
        info!("Controller {}: Validating block {}", i, block_index);
        let abort = match controller.fw_update_validate_stream(block_index).await {
            Ok(TfuqBlockStatus::DataValidAndAuthentic)
            | Ok(TfuqBlockStatus::DataValidButRepeated)
            | Ok(TfuqBlockStatus::HeaderValidAndAuthentic) => false,
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
            exit_fw_update_mode(controllers).await?;
            return PdError::Failed.into();
        }
    }
    Ok(())
}

async fn fw_update_load_app_image<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
    image: &[u8],
    num_data_blocks: usize,
) -> Result<(), Error<B::Error>> {
    for i in 0..num_data_blocks {
        info!("Broadcasting data block {}", i);
        fw_update_stream_data(
            controllers,
            image,
            data_block_index_to_block_index(i),
            data_block_metadata_offset(i),
            PD_FW_DATA_BLOCK_METADATA_SIZE,
        )
        .await?;
    }

    Ok(())
}

async fn fw_update_load_app_config<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
    image: &[u8],
    num_data_blocks: usize,
) -> Result<(), Error<B::Error>> {
    let app_size = get_image_size(image).map_err(Error::Pd)? as usize;
    debug!("App size: {}", app_size);
    let metadata_offset = app_config_block_metadata_offset(num_data_blocks, app_size);
    info!("Broadcasting app config block");
    fw_update_stream_data(
        controllers,
        image,
        PD_FW_APP_CONFIG_BLOCK_INDEX,
        metadata_offset,
        PD_FW_APP_CONFIG_METADATA_SIZE,
    )
    .await
}

async fn fw_update_complete<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
) -> Result<(), Error<B::Error>> {
    for (i, controller) in controllers.iter_mut().enumerate() {
        info!("Controller {}: Completing FW update", i);
        if let Err(_) = controller.fw_update_complete().await {
            error!("Controller {}: Failed to complete FW update", i);
            let _ = controller.fw_update_mode_exit().await;
        }
    }

    Ok(())
}

async fn burst_write<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
    address: u8,
    data: &[u8],
) -> Result<(), Error<B::Error>> {
    let mut inner = controllers[0].lock_inner().await;
    for chunk in data.chunks(BURST_WRITE_SIZE) {
        inner.bus.write(address, chunk).await.map_err(Error::Bus)?;
    }

    Ok(())
}

pub async fn perform_fw_update<'a, M: RawMutex, B: I2c>(
    controllers: &mut [&mut Tps6699x<'a, M, B>],
    image: &[u8],
) -> Result<(), Error<B::Error>> {
    info!("Starting FW update");
    enter_fw_update_mode(controllers).await?;
    let tfui_args = fw_update_init(controllers, image).await?;
    fw_update_load_app_image(controllers, image, tfui_args.num_data_blocks_tx as usize).await?;
    fw_update_load_app_config(controllers, image, tfui_args.num_data_blocks_tx as usize).await?;
    fw_update_complete(controllers).await?;

    info!("FW update complete");

    Ok(())
}
