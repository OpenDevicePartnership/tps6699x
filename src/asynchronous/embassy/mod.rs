//! High-level API that uses embassy_sync wakers
// This code holds refcells across await points but this is controlled within the code using scope

use defmt::debug;
use embassy_sync::blocking_mutex::raw::{NoopRawMutex, RawMutex};
use embassy_sync::mutex::{Mutex, MutexGuard};
use embassy_sync::signal::Signal;
use embassy_time::{with_timeout, Duration, Timer};
use embedded_hal::digital::InputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::asynchronous::controller::PdController;
use embedded_usb_pd::{Error, PdError, PortId};

use crate::asynchronous::internal;
use crate::command::{
    Command, ResetArgs, ReturnValue, TfudArgs, TfuiArgs, TfuqArgs, TfuqBlockStatus, TfuqCommandType, TfuqReturnValue,
    TfuqStatusQuery, BURST_WRITE_SIZE, PD_FW_APP_CONFIG_METADATA_SIZE, PD_FW_APP_IMAGE_SIZE_OFFSET,
    PD_FW_DATA_BLOCK_METADATA_SIZE, PD_FW_DATA_BLOCK_SIZE, PD_FW_HEADER_BLOCK_LENGTH, PD_FW_HEADER_BLOCK_OFFSET,
    PD_FW_HEADER_METADATA_LENGTH, PD_FW_HEADER_METADATA_OFFSET, PD_FW_IMAGE_ID_LENGTH, RESET_ARGS_LEN,
    RESET_FEATURE_ENABLE, TFUQ_RETURN_LEN,
};
use crate::registers::field_sets::IntEventBus1;
use crate::{registers, Mode, TFUE_TIMEOUT_MS, TFUS_TIMEOUT_MS};

pub struct Controller<M: RawMutex, B: I2c> {
    inner: Mutex<M, internal::Tps6699x<B>>,
    interrupt_waker: Signal<NoopRawMutex, (IntEventBus1, IntEventBus1)>,
}

impl<M: RawMutex, B: I2c> Controller<M, B> {
    pub fn new(bus: B, addr: [u8; 2]) -> Result<Self, Error<B::Error>> {
        Ok(Self {
            inner: Mutex::new(internal::Tps6699x::new(bus, addr)),
            interrupt_waker: Signal::new(),
        })
    }

    pub fn make_parts(&mut self) -> (Tps6699x<'_, M, B>, Interrupt<'_, M, B>) {
        let tps = Tps6699x { controller: self };
        let interrupt = Interrupt { controller: self };
        (tps, interrupt)
    }
}

pub struct Tps6699x<'a, M: RawMutex, B: I2c> {
    controller: &'a Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> Tps6699x<'a, M, B> {
    pub async fn lock_inner(&mut self) -> MutexGuard<'_, M, internal::Tps6699x<B>> {
        self.controller.inner.lock().await
    }

    pub async fn get_port_status(&mut self, port: PortId) -> Result<registers::field_sets::Status, Error<B::Error>> {
        self.lock_inner().await.get_port_status(port).await
    }

    pub async fn get_active_pdo_contract(
        &mut self,
        port: PortId,
    ) -> Result<registers::field_sets::ActivePdoContract, Error<B::Error>> {
        self.lock_inner().await.get_active_pdo_contract(port).await
    }

    pub async fn get_active_rdo_contract(
        &mut self,
        port: PortId,
    ) -> Result<registers::field_sets::ActiveRdoContract, Error<B::Error>> {
        self.lock_inner().await.get_active_rdo_contract(port).await
    }

    pub async fn get_mode(&mut self) -> Result<Mode, Error<B::Error>> {
        self.lock_inner().await.get_mode().await
    }

    pub async fn get_fw_version(&mut self) -> Result<u32, Error<B::Error>> {
        self.lock_inner().await.get_fw_version().await
    }

    /// Execute the given command
    pub async fn execute_command_int_type(
        &mut self,
        port: PortId,
        cmd: Command,
        indata: Option<&[u8]>,
        outdata: Option<&mut [u8]>,
        f: impl Fn(&IntEventBus1) -> bool,
    ) -> Result<ReturnValue, Error<B::Error>> {
        {
            let mut inner = self.lock_inner().await;
            inner.send_raw_command_unchecked(port, cmd, indata).await?;
        }

        self.wait_interrupt(true, f).await;
        {
            let mut inner = self.lock_inner().await;
            inner.read_command_result(port, outdata).await
        }
    }

    pub async fn execute_command(
        &mut self,
        port: PortId,
        cmd: Command,
        indata: Option<&[u8]>,
        outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        self.execute_command_int_type(port, cmd, indata, outdata, |flags| flags.cmd_1_completed())
            .await
    }

    pub async fn wait_interrupt(
        &mut self,
        clear_current: bool,
        f: impl Fn(&IntEventBus1) -> bool,
    ) -> (IntEventBus1, IntEventBus1) {
        if clear_current {
            self.controller.interrupt_waker.reset();
        }

        loop {
            let (p0_flags, p1_flags) = self.controller.interrupt_waker.wait().await;
            if f(&p0_flags) || f(&p1_flags) {
                return (p0_flags, p1_flags);
            }
        }
    }

    pub async fn fw_update_mode_enter(&mut self) -> Result<(), Error<B::Error>> {
        let result = with_timeout(
            Duration::from_millis(TFUS_TIMEOUT_MS),
            self.execute_command_int_type(PortId(0), Command::Tfus, None, None, |flags| {
                defmt::info!("Checking command interrupt");
                flags.boot_error()
            }),
        )
        .await
        .map_err(|_| PdError::Timeout.into())??;

        defmt::debug!("Write command: {:?}", result);
        if result == ReturnValue::Abort {
            return Err(PdError::Busy.into());
        } else if result != ReturnValue::Success {
            return Err(PdError::Failed.into());
        }

        // Confirm we're in the correct mode
        let mode = self.get_mode().await?;
        defmt::info!("Mode: {}", mode);
        if mode != Mode::F211 {
            return Err(PdError::InvalidMode.into());
        }
        Ok(())
    }

    async fn burst_write(&mut self, address: u8, data: &[u8]) -> Result<(), Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        for chunk in data.chunks(BURST_WRITE_SIZE) {
            inner.bus.write(address, chunk).await.map_err(Error::Bus)?;
        }

        Ok(())
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

    // TODO: make this use Seek and Read traits
    pub async fn fw_update_init(&mut self, image: &[u8]) -> Result<(), Error<B::Error>> {
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
        defmt::info!("Tfui args: {:?}", tfui_args);

        self.execute_command(PortId(0), Command::Tfui, Some(&buf), None).await?;

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

        Timer::after_millis(250).await;
        debug!("Validing header");
        self.fw_update_validate_stream().await?;

        Ok(())
    }

    pub async fn fw_update_mode_exit(&mut self) -> Result<(), Error<B::Error>> {
        defmt::info!("Exiting firmware update mode");
        let result = with_timeout(
            Duration::from_millis(TFUE_TIMEOUT_MS),
            self.execute_command(PortId(0), Command::Tfue, None, None),
        )
        .await;

        // Reset the controller if we failed to exit fw update mode
        if result.is_err() || result.unwrap()? != ReturnValue::Success {
            defmt::info!("Failed to exit, attempting to reset");
            let mut delay = embassy_time::Delay;
            self.reset(&mut delay).await?;
            return Err(PdError::Failed.into());
        }

        defmt::info!("Exit firmware update mode complete");
        Ok(())
    }

    pub async fn fw_update_validate_stream(&mut self) -> Result<TfuqBlockStatus, Error<B::Error>> {
        let args = TfuqArgs {
            command: TfuqCommandType::QueryTfuStatus,
            status_query: TfuqStatusQuery::StatusInProgress,
        };

        let mut arg_bytes = [0u8; 2];
        let mut return_bytes = [0u8; TFUQ_RETURN_LEN];

        args.encode_into_slice(&mut arg_bytes).map_err(Error::Pd)?;

        let result = with_timeout(
            Duration::from_millis(TFUE_TIMEOUT_MS),
            self.execute_command(PortId(0), Command::Tfuq, Some(&arg_bytes), Some(&mut return_bytes)),
        )
        .await;

        if result.is_err() {
            debug!("Validate stream timeout");
            return PdError::Timeout.into();
        }

        let data = TfuqReturnValue::decode_from_slice(&return_bytes).map_err(Error::Pd)?;
        debug!("Validate stream result: {:?}", data);

        Ok(TfuqBlockStatus::Success)
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

        let result = with_timeout(
            Duration::from_millis(TFUE_TIMEOUT_MS),
            self.execute_command(PortId(0), Command::Tfud, Some(&arg_bytes), None),
        )
        .await;

        if result.is_err() {
            debug!("Validate stream timeout");
            return PdError::Timeout.into();
        }

        let data_len = args.data_len as usize;
        let block = &image[Self::block_offset(metadata_offset)..Self::block_offset(metadata_offset) + data_len];
        self.burst_write(args.broadcast_u16_address as u8, block).await?;

        let status = self.fw_update_validate_stream().await?;
        debug!("Data block status: {:?}", status);

        Timer::after_millis(150).await;

        Ok(())
    }

    pub async fn fw_update_load_app_image(
        &mut self,
        image: &[u8],
        num_data_blocks: usize,
    ) -> Result<(), Error<B::Error>> {
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

    pub async fn fw_update_load_app_config(
        &mut self,
        image: &[u8],
        num_data_blocks: usize,
    ) -> Result<(), Error<B::Error>> {
        let app_size = Self::get_image_size(image)? as usize;
        debug!("App size: {}", app_size);
        let metadata_offset = Self::app_config_block_metadata_offset(num_data_blocks, app_size);
        self.fw_update_stream_data(image, metadata_offset, PD_FW_APP_CONFIG_METADATA_SIZE)
            .await
    }

    pub async fn fw_update_complete(&mut self) -> Result<(), Error<B::Error>> {
        let mut arg_bytes = [0u8; RESET_ARGS_LEN];

        let args = ResetArgs {
            switch_banks: 0,
            copy_bank: RESET_FEATURE_ENABLE,
        };

        args.encode_into_slice(&mut arg_bytes).map_err(Error::Pd)?;

        self.execute_command(PortId(0), Command::Tfuc, Some(&arg_bytes), None)
            .await?;
        Ok(())
    }
}

impl<'a, M: RawMutex, B: I2c> PdController<B::Error> for Tps6699x<'a, M, B> {
    async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.reset(delay).await
    }
}

pub struct Interrupt<'a, M: RawMutex, B: I2c> {
    controller: &'a Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> Interrupt<'a, M, B> {
    async fn lock_inner(&mut self) -> MutexGuard<'_, M, internal::Tps6699x<B>> {
        self.controller.inner.lock().await
    }

    pub async fn process_interrupt(
        &mut self,
        int: &mut impl InputPin,
    ) -> Result<(IntEventBus1, IntEventBus1), Error<B::Error>> {
        let mut flags = [IntEventBus1::new_zero(); 2];

        {
            let mut inner = self.lock_inner().await;
            for port in 0..2 {
                let port_id = PortId(port);

                // Early exit if checking the last port cleared the interrupt
                // TODO: better error handling
                let result = int.is_high();
                if result.is_err() || !result.unwrap() {
                    flags[port as usize] = IntEventBus1::new_zero();
                }

                let result = inner.clear_interrupt(port_id).await;
                if let Err(e) = result {
                    match e {
                        Error::Pd(PdError::Busy) => {
                            // Under certain conditions the controller will not respond to reads while processing a command
                            // This is a normal condition and should be ignored
                            continue;
                        }
                        _ => {
                            defmt::error!("Error processing interrupt on port {}", port);
                            return Err(e);
                        }
                    }
                }

                flags[port as usize] = result.unwrap_or(IntEventBus1::new_zero());
            }
        }

        let flags = (flags[0], flags[1]);
        self.controller.interrupt_waker.signal(flags);
        Ok(flags)
    }
}
