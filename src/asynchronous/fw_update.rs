//! Streaming FW update implementation
use core::future::Future;
use core::iter::zip;

use bincode::config;
use embedded_hal_async::delay::DelayNs;
use embedded_usb_pd::{Error, PdError};

use super::interrupt::InterruptController;
use crate::command::{ReturnValue, TfudArgs, TfuiArgs, TfuqBlockStatus};
use crate::fw_update::{
    APP_CONFIG_BLOCK_INDEX, APP_CONFIG_METADATA_LEN, APP_IMAGE_SIZE_LEN, APP_IMAGE_SIZE_OFFSET, DATA_BLOCK_LEN,
    DATA_BLOCK_METADATA_LEN, DATA_BLOCK_START_INDEX, HEADER_BLOCK_INDEX, HEADER_BLOCK_LEN, HEADER_BLOCK_OFFSET,
    HEADER_METADATA_LEN, HEADER_METADATA_OFFSET, IMAGE_ID_LEN, MAX_METADATA_LEN, TFUD_BURST_WRITE_DELAY_MS,
    TFUI_BURST_WRITE_DELAY_MS,
};
use crate::stream::Stream;
use crate::{debug, error, trace, warn, PORT0};

/// Size of buffer used for reading various metadata
const BUFFER_LENGTH: usize = MAX_METADATA_LEN;

/// Trait for updating the firmware of a target device
pub trait UpdateTarget: InterruptController {
    /// Enter FW update mode
    fn fw_update_mode_enter(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> impl Future<Output = Result<(), Error<Self::BusError>>>;

    /// Start FW update
    fn fw_update_init(
        &mut self,
        delay: &mut impl DelayNs,
        args: &TfuiArgs,
    ) -> impl Future<Output = Result<ReturnValue, Error<Self::BusError>>>;

    /// Abort FW update
    fn fw_update_mode_exit(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> impl Future<Output = Result<(), Error<Self::BusError>>>;

    /// Validate the most recent block supplied to the device
    fn fw_update_validate_stream(
        &mut self,
        delay: &mut impl DelayNs,
        block_index: usize,
    ) -> impl Future<Output = Result<TfuqBlockStatus, Error<Self::BusError>>>;

    /// Stream a block to the device
    fn fw_update_stream_data(
        &mut self,
        delay: &mut impl DelayNs,
        args: &TfudArgs,
    ) -> impl Future<Output = Result<(), Error<Self::BusError>>>;

    /// Complete the FW update process
    fn fw_update_complete(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> impl Future<Output = Result<(), Error<Self::BusError>>>;

    /// Write data to all supplied devices
    fn fw_update_burst_write(
        &mut self,
        address: u8,
        data: &[u8],
    ) -> impl Future<Output = Result<(), Error<Self::BusError>>>;
}

/// Computes the offset of a data block's metadata
pub const fn data_block_metadata_offset(block: usize) -> usize {
    HEADER_BLOCK_OFFSET + HEADER_BLOCK_LEN + (block * (DATA_BLOCK_LEN + DATA_BLOCK_METADATA_LEN))
}

/// Computes the offset of a data block's data
pub const fn block_offset(metadata_offset: usize) -> usize {
    metadata_offset + DATA_BLOCK_METADATA_LEN
}

/// Computes the offset of the app config block's metadata
pub const fn app_config_block_metadata_offset(num_data_blocks: usize, app_size: usize) -> usize {
    app_size + IMAGE_ID_LEN + HEADER_METADATA_LEN + HEADER_BLOCK_LEN + num_data_blocks * DATA_BLOCK_METADATA_LEN
}

/// Converts a data block into a block index
pub const fn data_block_index_to_block_index(block_index: usize) -> usize {
    block_index + DATA_BLOCK_START_INDEX
}

/// Current update state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum State {
    /// Receiving update args
    UpdateArgs,
    /// Receiving header block
    HeaderBlockStart,
    /// Receiving image size
    ImageSize,
    // Receiver rest of header block
    HeaderBlockRest,
    /// Receiving data block N header
    DataBlockHeader(usize),
    /// Receiving data block N
    DataBlock(usize),
    /// Receiving app config block header
    ConfigHeader,
    /// Receiving app config block
    ConfigBlock,
    /// Complete
    Complete,
}

/// Updater struct
pub struct Updater<const N: usize, T: UpdateTarget> {
    /// Stream
    stream: Stream,
    /// Current state of the updater
    state: State,
    /// Buffer for various metadata
    buffer: [u8; BUFFER_LENGTH],
    /// Update args
    update_args: TfuiArgs,
    /// Image size
    image_size: usize,
    /// Interrupt guards
    guards: [Option<T::Guard>; N],
}

impl<const N: usize, T: UpdateTarget> Updater<N, T> {
    /// Enter FW update mode on all controllers
    async fn fw_update_mode_enter(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<T::BusError>> {
        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Entering FW update mode", i);
            if let Err(e) = controller.fw_update_mode_enter(delay).await {
                debug!("Controller {}: Failed to enter FW update mode", i);
                return Err(e);
            }
        }

        Ok(())
    }

    /// Initialize FW update on all controllers
    async fn fw_update_init(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<T::BusError>> {
        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Initializing FW update", i);

            match controller.fw_update_init(delay, &self.update_args).await {
                Ok(ReturnValue::Success) => (),
                Ok(r) => {
                    debug!("Controller {}: Failed to initialize FW update, result {:#?}", i, r);
                    return Err(Error::Pd(PdError::Failed));
                }
                Err(e) => {
                    debug!("Controller {}: Failed to initialize FW update", i);
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    /// Send data to all controllers on the burst write address
    async fn fw_update_burst_write(
        &mut self,
        controllers: &mut [&mut T],
        data: &[u8],
    ) -> Result<(), Error<T::BusError>> {
        for (i, controller) in controllers.iter_mut().enumerate() {
            trace!("Controller {}: Sending burst write", i);
            if let Err(e) = controller
                .fw_update_burst_write(self.update_args.broadcast_u16_address as u8, data)
                .await
            {
                debug!("Controller {}: Failed to send burst write", i);
                return Err(e);
            }
        }

        Ok(())
    }

    /// Validate the most recent block supplied to the device
    async fn fw_update_validate_stream(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        block_index: usize,
    ) -> Result<(), Error<T::BusError>> {
        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Validating stream", i);
            match controller.fw_update_validate_stream(delay, block_index).await {
                Ok(TfuqBlockStatus::HeaderValidAndAuthentic)
                | Ok(TfuqBlockStatus::DataValidAndAuthentic)
                | Ok(TfuqBlockStatus::DataValidButRepeated) => (),
                Ok(r) => {
                    error!("Controller {}: Header block validation failed, result {:#?}", i, r);
                    return Err(Error::Pd(PdError::Failed));
                }
                Err(_) => {
                    error!("Controller {}: Header block validation failed", i);
                    return Err(Error::Pd(PdError::Failed));
                }
            }
        }

        Ok(())
    }

    /// Stream block data to all controllers
    async fn fw_update_stream_data(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        args: &TfudArgs,
    ) -> Result<(), Error<T::BusError>> {
        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Streaming data block", i);
            if controller.fw_update_stream_data(delay, args).await.is_err() {
                error!("Controller {}: Failed to stream data block", i);
                return Err(Error::Pd(PdError::Failed));
            }
        }

        Ok(())
    }

    /// Start the FW update process
    pub async fn start_fw_update(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<T::BusError>> {
        // Disable all interrupts during the reset into FW update mode
        for (guard, controller) in zip(self.guards.iter_mut(), controllers.iter_mut()) {
            *guard = Some(controller.disable_all_interrupts_guarded().await?);
        }

        self.fw_update_mode_enter(controllers, delay).await?;

        // Re-enable interrupts on port 0 only, other ports don't respond to interrupts in FW update mode
        for (guard, controller) in zip(self.guards.iter_mut(), controllers.iter_mut()) {
            *guard = Some(controller.enable_interrupt_guarded(PORT0, true).await?);
        }

        Ok(())
    }

    /// Supply update contents to the updater
    pub async fn write_bytes(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        data: &[u8],
    ) -> Result<bool, Error<T::BusError>> {
        let mut data = data;

        while !data.is_empty() {
            if self.state == State::Complete {
                break;
            }

            if self.stream.is_seeking() {
                data = self.handle_seek(data).await?;
            } else if self.stream.is_reading() {
                data = self.handle_read(controllers, delay, data).await?;
            }
        }

        Ok(self.state == State::Complete)
    }

    /// Handle seeking in the stream
    async fn handle_seek<'c>(&mut self, data: &'c [u8]) -> Result<&'c [u8], Error<T::BusError>> {
        let (data, bytes_needed) = self.stream.seek_bytes(data)?;
        if bytes_needed > 0 {
            // Still waiting for more bytes
            return Ok(data);
        }

        // Switch to needs bytes state
        match self.state {
            State::UpdateArgs => {
                trace!("Reading update args");
                self.stream.start_read(HEADER_METADATA_LEN);
            }
            State::HeaderBlockStart => {
                trace!("Read header start");
                // Read the first part of the header block, up to the image size
                self.stream.start_read(APP_IMAGE_SIZE_OFFSET - HEADER_BLOCK_OFFSET);
            }
            State::ImageSize => {
                trace!("Read app image size");
                self.stream.start_read(APP_IMAGE_SIZE_LEN);
            }
            State::HeaderBlockRest => {
                trace!("Read rest of  header block");
                self.stream
                    .start_read(HEADER_BLOCK_LEN - APP_IMAGE_SIZE_LEN - APP_IMAGE_SIZE_OFFSET - HEADER_BLOCK_OFFSET);
            }
            State::DataBlockHeader(i) => {
                trace!("Read data block {}", i);
                self.stream.start_read(DATA_BLOCK_METADATA_LEN);
            }
            State::ConfigHeader => {
                trace!("Seek config header");
                self.stream.start_read(APP_CONFIG_METADATA_LEN);
            }
            _ => {
                trace!("Seek other: {:#?}", self.state);
                return Err(PdError::InvalidMode.into());
            }
        }

        Ok(data)
    }

    /// Handle reading data from the stream
    async fn handle_read<'c>(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        data: &'c [u8],
    ) -> Result<&'c [u8], Error<T::BusError>> {
        let (data, to_take, read_state) = self.stream.read_bytes(data)?;
        let remaining = read_state.total - (read_state.current + to_take.len());
        let current = read_state.current;

        match self.state {
            State::UpdateArgs => {
                trace!("Reading update args");
                self.read_update_args(controllers, delay, to_take, current, remaining)
                    .await?;
            }
            State::HeaderBlockStart => {
                trace!("Reading header start");
                self.process_header_start(controllers, to_take, remaining).await?
            }
            State::ImageSize => {
                trace!("Reading image size");
                self.read_image_size(controllers, to_take, current, remaining).await?;
            }
            State::HeaderBlockRest => {
                trace!("Reading header rest");
                self.process_header_rest(controllers, delay, to_take, remaining).await?;
            }
            State::DataBlockHeader(block_index) => {
                trace!("Reading data block header: {}", block_index);
                self.read_data_block_header(controllers, delay, to_take, current, remaining, block_index)
                    .await?;
            }
            State::DataBlock(block_index) => {
                trace!("Reading data block: {}", block_index);
                self.read_data_block(controllers, delay, to_take, remaining, block_index)
                    .await?;
            }
            State::ConfigHeader => {
                trace!("Reading config header");
                self.read_config_block_header(controllers, delay, to_take, current, remaining)
                    .await?;
            }
            State::ConfigBlock => {
                trace!("Reading config block");
                self.read_config_block(controllers, delay, to_take, remaining).await?;
            }
            State::Complete => {
                trace!("Read other: {:#?}", self.state);
                return Err(PdError::InvalidMode.into());
            }
        }

        Ok(data)
    }

    /// Read update args and proceed to the next state
    async fn read_update_args(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        to_take: &[u8],
        current: usize,
        remaining: usize,
    ) -> Result<(), Error<T::BusError>> {
        trace!(
            "Read args: {} bytes to take, remaining: {}, current: {}, data: {:#?}",
            to_take.len(),
            remaining,
            current,
            to_take
        );
        self.buffer[current..current + to_take.len()].copy_from_slice(to_take);

        if remaining == 0 {
            // We have the full header metadata
            let (args, _) = bincode::decode_from_slice(&self.buffer, config::standard().with_fixed_int_encoding())
                .map_err(|_| PdError::Serialize)?;
            self.update_args = args;
            self.fw_update_init(controllers, delay).await?;
            trace!("Got update args: {:#?}", self.update_args);

            // Proceed to the next state
            self.state = State::HeaderBlockStart;
            self.stream.start_seek(HEADER_BLOCK_OFFSET)?;
        }

        Ok(())
    }

    /// Process the first part of the update header block
    async fn process_header_start(
        &mut self,
        controllers: &mut [&mut T],
        to_take: &[u8],
        remaining: usize,
    ) -> Result<(), Error<T::BusError>> {
        self.fw_update_burst_write(controllers, to_take).await?;
        if remaining == 0 {
            self.state = State::ImageSize;
            self.stream.start_seek(APP_IMAGE_SIZE_OFFSET)?;
        }
        Ok(())
    }

    /// Process the rest of the update header block
    async fn process_header_rest(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        to_take: &[u8],
        remaining: usize,
    ) -> Result<(), Error<T::BusError>> {
        self.fw_update_burst_write(controllers, to_take).await?;
        if remaining == 0 {
            // Full header has been written
            // Validate and proceed to the next state
            delay.delay_ms(TFUI_BURST_WRITE_DELAY_MS).await;
            trace!("Validing header");
            self.fw_update_validate_stream(controllers, delay, HEADER_BLOCK_INDEX)
                .await?;
            trace!("Header validated");
            self.state = State::DataBlockHeader(0);
            self.stream.start_seek(data_block_metadata_offset(0))?
        }
        Ok(())
    }

    /// Read update arguments for the current block
    async fn read_block_args(
        &mut self,
        to_take: &[u8],
        current: usize,
        remaining: usize,
    ) -> Result<Option<TfudArgs>, Error<T::BusError>> {
        self.buffer[current..current + to_take.len()].copy_from_slice(to_take);

        if remaining == 0 {
            // We have the full header metadata
            let (args, _) = bincode::decode_from_slice(&self.buffer, config::standard().with_fixed_int_encoding())
                .map_err(|_| PdError::Serialize)?;
            Ok(Some(args))
        } else {
            // Still waiting for the rest of the header
            Ok(None)
        }
    }

    /// Read data block header and proceed to the next state
    async fn read_data_block_header(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        to_take: &[u8],
        current: usize,
        remaining: usize,
        block_index: usize,
    ) -> Result<(), Error<T::BusError>> {
        if let Some(args) = self.read_block_args(to_take, current, remaining).await? {
            // We have the full header metadata
            self.fw_update_stream_data(controllers, delay, &args).await?;

            // Proceed to the next state
            self.state = State::DataBlock(block_index);
            self.stream.start_read(args.data_len.into());
        }

        Ok(())
    }

    /// Read data block and proceed to the next state
    async fn read_data_block(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        to_take: &[u8],
        remaining: usize,
        block_index: usize,
    ) -> Result<(), Error<T::BusError>> {
        self.fw_update_burst_write(controllers, to_take).await?;

        if remaining == 0 {
            // Full block has been written
            // Validate and proceed to the next state
            delay.delay_ms(TFUD_BURST_WRITE_DELAY_MS).await;
            self.fw_update_validate_stream(controllers, delay, data_block_index_to_block_index(block_index))
                .await?;

            let next_block = block_index + 1;
            if next_block < self.update_args.num_data_blocks_tx.into() {
                // Proceed to the next block
                self.state = State::DataBlockHeader(next_block);
                self.stream.start_seek(data_block_metadata_offset(next_block))?
            } else {
                self.state = State::ConfigHeader;
                self.stream.start_seek(app_config_block_metadata_offset(
                    self.update_args.num_data_blocks_tx.into(),
                    self.image_size,
                ))?
            }
        }
        Ok(())
    }

    /// Read the image size from the header block and proceed to the next state
    async fn read_image_size(
        &mut self,
        controllers: &mut [&mut T],
        to_take: &[u8],
        current: usize,
        remaining: usize,
    ) -> Result<(), Error<T::BusError>> {
        self.buffer[current..current + to_take.len()].copy_from_slice(to_take);
        self.fw_update_burst_write(controllers, to_take).await?;
        if remaining == 0 {
            // We have the full image size
            let (image_size, _): (u32, _) =
                bincode::decode_from_slice(&self.buffer, config::standard().with_fixed_int_encoding())
                    .map_err(|_| PdError::Serialize)?;

            self.image_size = image_size as usize;
            self.state = State::HeaderBlockRest;
            self.stream
                .start_read(HEADER_BLOCK_LEN - APP_IMAGE_SIZE_LEN - APP_IMAGE_SIZE_OFFSET + HEADER_BLOCK_OFFSET);
        }
        Ok(())
    }

    /// Read app config block header and proceed to the next state
    async fn read_config_block_header(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        to_take: &[u8],
        current: usize,
        remaining: usize,
    ) -> Result<(), Error<T::BusError>> {
        if let Some(args) = self.read_block_args(to_take, current, remaining).await? {
            // We have the full header metadata
            self.fw_update_stream_data(controllers, delay, &args).await?;

            // Proceed to the next state
            self.state = State::ConfigBlock;
            self.stream.start_read(args.data_len.into());
        }

        Ok(())
    }

    /// Read config block and proceed to the next state
    async fn read_config_block(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        to_take: &[u8],
        remaining: usize,
    ) -> Result<(), Error<T::BusError>> {
        self.fw_update_burst_write(controllers, to_take).await?;

        if remaining == 0 {
            // Full block has been written
            // Validate and proceed to the next state
            delay.delay_ms(TFUD_BURST_WRITE_DELAY_MS).await;
            self.fw_update_validate_stream(controllers, delay, APP_CONFIG_BLOCK_INDEX)
                .await?;

            // Proceed to the next state
            trace!("FW update complete");
            self.state = State::Complete;
        }
        Ok(())
    }

    /// Exit the FW update process early
    pub async fn exit_fw_update_mode(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<T::BusError>> {
        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Exiting FW update mode", i);
            if controller.fw_update_mode_exit(delay).await.is_err() {
                debug!("Controller {}: Failed to exit FW update mode", i);
                // Don't return to allow the other controllers to exit FW update mode
            }
        }

        Ok(())
    }

    /// Complete the FW update process
    pub async fn complete_fw_update(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<T::BusError>> {
        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Completing FW update", i);
            if controller.fw_update_complete(delay).await.is_err() {
                warn!("Controller {}: Failed to complete FW update, attempting to exit", i);
                controller.fw_update_mode_exit(delay).await?;
                // Don't return to allow the other controllers to exit FW update mode
            }
        }

        Ok(())
    }
}

impl<const N: usize, T: UpdateTarget> Default for Updater<N, T> {
    fn default() -> Self {
        Updater {
            stream: Stream::new(HEADER_METADATA_OFFSET),
            state: State::UpdateArgs,
            buffer: [0; BUFFER_LENGTH],
            update_args: Default::default(),
            image_size: 0,
            guards: [const { None }; N],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::asynchronous::interrupt::InterruptGuard;
    use crate::test::Delay;
    use crate::MAX_SUPPORTED_PORTS;
    extern crate std;

    /// Simple mock update target for testing that does nothing
    #[derive(Debug)]
    struct UpdateTargetNoop {}

    struct UpdateInterruptGuard;

    impl Drop for UpdateInterruptGuard {
        fn drop(&mut self) {}
    }
    impl InterruptGuard for UpdateInterruptGuard {}

    impl InterruptController for UpdateTargetNoop {
        type Guard = UpdateInterruptGuard;
        type BusError = ();

        async fn interrupts_enabled(
            &self,
        ) -> Result<[bool; MAX_SUPPORTED_PORTS], embedded_usb_pd::Error<Self::BusError>> {
            Ok([true; MAX_SUPPORTED_PORTS])
        }

        async fn enable_interrupts_guarded(
            &mut self,
            _enabled: [bool; MAX_SUPPORTED_PORTS],
        ) -> Result<Self::Guard, embedded_usb_pd::Error<Self::BusError>> {
            Ok(UpdateInterruptGuard)
        }
    }

    impl UpdateTarget for UpdateTargetNoop {
        async fn fw_update_mode_enter(&mut self, _delay: &mut impl DelayNs) -> Result<(), Error<Self::BusError>> {
            Ok(())
        }

        async fn fw_update_init(
            &mut self,
            _delay: &mut impl DelayNs,
            _args: &TfuiArgs,
        ) -> Result<ReturnValue, Error<Self::BusError>> {
            Ok(ReturnValue::Success)
        }

        async fn fw_update_mode_exit(&mut self, _delay: &mut impl DelayNs) -> Result<(), Error<Self::BusError>> {
            Ok(())
        }

        async fn fw_update_validate_stream(
            &mut self,
            _delay: &mut impl DelayNs,
            block_index: usize,
        ) -> Result<TfuqBlockStatus, Error<Self::BusError>> {
            match block_index {
                HEADER_BLOCK_INDEX => Ok(TfuqBlockStatus::HeaderValidAndAuthentic),
                _ => Ok(TfuqBlockStatus::DataValidAndAuthentic),
            }
        }

        async fn fw_update_stream_data(
            &mut self,
            _delay: &mut impl DelayNs,
            _args: &TfudArgs,
        ) -> Result<(), Error<Self::BusError>> {
            Ok(())
        }

        async fn fw_update_complete(&mut self, _delay: &mut impl DelayNs) -> Result<(), Error<Self::BusError>> {
            Ok(())
        }

        async fn fw_update_burst_write(&mut self, _address: u8, _data: &[u8]) -> Result<(), Error<Self::BusError>> {
            Ok(())
        }
    }

    /// Test that the fw update only seeks forward
    #[tokio::test]
    async fn test_fw_update_seek_forward() {
        let mut delay = Delay {};
        let mut target = UpdateTargetNoop {};
        let mut controllers = [&mut target];
        let mut updater: Updater<1, _> = Updater::default();

        updater.start_fw_update(&mut controllers, &mut delay).await.unwrap();

        let fw_mock = include_bytes!("../../fw_mock.bin").as_slice();
        // Use 51 as it's not a typical even number
        for chunk in fw_mock.chunks(51) {
            updater.write_bytes(&mut controllers, &mut delay, chunk).await.unwrap();
        }

        updater.complete_fw_update(&mut controllers, &mut delay).await.unwrap();
    }
}
