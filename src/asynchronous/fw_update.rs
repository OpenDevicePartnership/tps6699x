//! Streaming FW update implementation
use core::future::Future;
use core::iter::zip;
use core::marker::PhantomData;

use bincode::config;
use embedded_hal_async::delay::DelayNs;
use embedded_usb_pd::{Error, PdError};

use super::interrupt::InterruptController;
use crate::command::{ReturnValue, TfudArgs, TfuiArgs, TfuqBlockStatus};
use crate::fw_update::{
    State, UpdateConfig, APP_CONFIG_BLOCK_INDEX, DATA_BLOCK_LEN, DATA_BLOCK_METADATA_LEN, DATA_BLOCK_START_INDEX,
    HEADER_BLOCK_INDEX, HEADER_BLOCK_LEN, HEADER_BLOCK_OFFSET, HEADER_METADATA_LEN, HEADER_METADATA_OFFSET,
    IMAGE_ID_LEN, MAX_METADATA_LEN, TFUD_BURST_WRITE_DELAY_MS, TFUI_BURST_WRITE_DELAY_MS, UPDATE_CHUNK_LENGTH,
};
use crate::stream::*;
use crate::{debug, error, info, trace, warn, PORT0};

/// Size of args_buffer used for reading various metadata
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

/// Abort the FW update process
async fn abort_fw_update<T: UpdateTarget>(controllers: &mut [&mut T], delay: &mut impl DelayNs) {
    for (i, controller) in controllers.iter_mut().enumerate() {
        debug!("Controller {}: Exiting FW update mode", i);
        if controller.fw_update_mode_exit(delay).await.is_err() {
            debug!("Controller {}: Failed to exit FW update mode", i);
            // Don't return to allow the other controllers to exit FW update mode
        }
    }
}

/// Initializes the FW update process
///
/// All supplied controllers are updated in parallel using burst writes
pub struct BorrowedUpdater<T: UpdateTarget> {
    /// Phantom target
    _target: PhantomData<T>,
    /// Update configuration
    config: UpdateConfig,
}

impl<T: UpdateTarget> Default for BorrowedUpdater<T> {
    fn default() -> Self {
        Self {
            _target: PhantomData,
            config: UpdateConfig::default(),
        }
    }
}

impl<T: UpdateTarget> BorrowedUpdater<T> {
    /// Create a new updater
    pub fn with_config(config: UpdateConfig) -> Self {
        Self {
            _target: PhantomData,
            config,
        }
    }
}

impl<T: UpdateTarget> BorrowedUpdater<T> {
    /// Enter FW update mode on all controllers
    pub async fn start_fw_update(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
    ) -> Result<BorrowedUpdaterInProgress<T>, Error<T::BusError>> {
        if controllers.is_empty() {
            return Err(PdError::InvalidParams.into());
        }

        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Entering FW update mode", i);
            if let Err(e) = controller.fw_update_mode_enter(delay).await {
                debug!("Controller {}: Failed to enter FW update mode", i);
                abort_fw_update(controllers, delay).await;
                return Err(e);
            }
        }

        Ok(BorrowedUpdaterInProgress::new(self.config.clone()))
    }
}

/// Handles writing FW and completing the update process
///
/// All supplied controllers are updated in parallel using burst writes
pub struct BorrowedUpdaterInProgress<T: UpdateTarget> {
    /// Stream
    stream: Stream,
    /// Current state of the updater
    state: State,
    /// Buffer used to store TfudArgs and TfuiArgs bytes before they are deserialized
    args_buffer: [u8; BUFFER_LENGTH],
    /// Update args
    update_args: Option<TfuiArgs>,
    /// Image size
    image_size: usize,
    /// Block args
    block_args: Option<TfudArgs>,
    /// Phantom target
    _target: PhantomData<T>,
    /// Update configuration
    config: UpdateConfig,
}

impl<T: UpdateTarget> BorrowedUpdaterInProgress<T> {
    fn new(config: UpdateConfig) -> Self {
        Self {
            stream: Stream::Seeking(SeekingStream::new(0, HEADER_METADATA_OFFSET)),
            state: State::UpdateArgs,
            args_buffer: [0; BUFFER_LENGTH],
            update_args: None,
            image_size: 0,
            block_args: None,
            _target: PhantomData,
            config,
        }
    }

    /// Initialize FW update on all controllers
    async fn fw_update_init(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<T::BusError>> {
        if controllers.is_empty() {
            return Err(PdError::InvalidParams.into());
        }

        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Initializing FW update", i);

            let update_args = self.update_args.ok_or(Error::Pd(PdError::InvalidParams))?;
            match controller.fw_update_init(delay, &update_args).await {
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
    ///
    /// Since all controllers are listening on the same burst write address,
    /// we only use the first supplied controller to actually do the write.
    async fn fw_update_burst_write(
        &mut self,
        controllers: &mut [&mut T],
        data: &[u8],
    ) -> Result<(), Error<T::BusError>> {
        if controllers.is_empty() {
            return Err(PdError::InvalidParams.into());
        }

        trace!("Controllers: Sending burst write");
        let update_args = self.update_args.ok_or(Error::Pd(PdError::InvalidParams))?;
        if let Err(e) = controllers[0]
            .fw_update_burst_write(update_args.broadcast_u16_address as u8, data)
            .await
        {
            debug!("Controllers: Failed to send burst write");
            return Err(e);
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
        if controllers.is_empty() {
            return Err(PdError::InvalidParams.into());
        }

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
        if controllers.is_empty() {
            return Err(PdError::InvalidParams.into());
        }

        for (i, controller) in controllers.iter_mut().enumerate() {
            debug!("Controller {}: Streaming data block", i);
            if controller.fw_update_stream_data(delay, args).await.is_err() {
                error!("Controller {}: Failed to stream data block", i);
                return Err(Error::Pd(PdError::Failed));
            }
        }

        Ok(())
    }

    /// Supply update contents to the updater
    ///
    /// Returns Ok(true) if the update is complete
    pub async fn write_bytes(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        data: &[u8],
    ) -> Result<bool, Error<T::BusError>> {
        let mut data = data;

        while !data.is_empty() && self.state != State::Complete {
            match self.stream {
                Stream::Seeking(stream) => {
                    data = self.handle_seek(data, stream).await?;
                }
                Stream::Reading(stream) => {
                    data = self.handle_read(controllers, delay, data, stream).await?;
                }
            }
        }

        Ok(self.state == State::Complete)
    }

    /// Handle seeking in the stream
    async fn handle_seek<'c>(
        &mut self,
        data: &'c [u8],
        mut stream: SeekingStream,
    ) -> Result<&'c [u8], Error<T::BusError>> {
        let data = stream.seek_bytes(data);
        if data.is_empty() {
            // Still waiting for more bytes, continue the current operation
            self.stream = Stream::Seeking(stream);
            return Ok(data);
        }

        let read_op = match self.state {
            State::DataBlock(_) | State::ConfigBlock => {
                let block_args = self.block_args.ok_or(Error::Pd(PdError::InvalidParams))?;
                stream.start_read(self.state.next_read_block_args(&block_args).map_err(Error::Pd)?)
            }
            _ => stream.start_read(self.state.next_read().map_err(Error::Pd)?),
        };

        self.stream = Stream::Reading(read_op);

        Ok(data)
    }

    /// Handle reading data from the stream
    async fn handle_read<'c>(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        data: &'c [u8],
        mut stream: ReadingStream,
    ) -> Result<&'c [u8], Error<T::BusError>> {
        let read_result = stream.read_bytes(data);

        let op = match self.state {
            State::UpdateArgs => {
                trace!("Reading update args");
                self.read_update_args(controllers, delay, &read_result).await
            }
            State::HeaderBlockStart => {
                trace!("Reading header start");
                self.process_header_start(controllers, &read_result).await
            }
            State::ImageSize => {
                trace!("Reading image size");
                self.read_image_size(controllers, &read_result).await
            }
            State::HeaderBlockRest => {
                trace!("Reading header rest");
                self.process_header_rest(controllers, delay, &read_result).await
            }
            State::DataBlockHeader(block_index) => {
                trace!("Reading data block header: {}", block_index);
                self.read_data_block_header(controllers, delay, &read_result).await
            }
            State::DataBlock(block_index) => {
                trace!("Reading data block: {}", block_index);
                self.read_data_block(controllers, delay, &read_result, block_index)
                    .await
            }
            State::ConfigHeader => {
                trace!("Reading config header");
                self.read_config_block_header(controllers, delay, &read_result).await
            }
            State::ConfigBlock => {
                trace!("Reading config block");
                self.read_config_block(controllers, delay, &read_result).await
            }
            State::Complete => {
                trace!("Read other: {:#?}", self.state);
                Err(PdError::InvalidMode.into())
            }
        }?;

        if let Some(op) = op {
            // Start the next operation
            self.stream = Stream::Seeking(stream.start_seek(op)?);
        } else {
            // Continue the current operation
            self.stream = Stream::Reading(stream);
        }

        Ok(read_result.remaining_data)
    }

    /// Read update args and proceed to the next state
    /// Returns the next stream operation or None if the current operation is not complete
    async fn read_update_args(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        read_result: &ReadResult<'_>,
    ) -> Result<Option<SeekOperation>, Error<T::BusError>> {
        let current = read_result.read_state.current;
        let read_len = read_result.read_data.len();

        trace!(
            "Read args: {} bytes to take, remaining: {}, current: {}, data: {:#?}",
            read_len,
            read_result.remaining(),
            current,
            read_result.read_data,
        );

        self.args_buffer[current..current + read_len].copy_from_slice(read_result.read_data);

        if read_result.is_complete() {
            // We have the full header metadata
            let (mut args, _): (TfuiArgs, _) =
                bincode::decode_from_slice(&self.args_buffer, config::standard().with_fixed_int_encoding())
                    .map_err(|_| PdError::Serialize)?;

            // Override broadcast address if specified
            args.broadcast_u16_address = self.config.broadcast_addr.unwrap_or(args.broadcast_u16_address);
            self.update_args = Some(args);
            self.fw_update_init(controllers, delay).await?;
            trace!("Got update args: {:#?}", self.update_args);

            // Proceed to the next state
            Ok(Some(self.state.next_seek().map_err(Error::Pd)?))
        } else {
            Ok(None)
        }
    }

    /// Process the first part of the update header block
    /// Returns the next stream operation or None if the current operation is not complete
    async fn process_header_start(
        &mut self,
        controllers: &mut [&mut T],
        read_result: &ReadResult<'_>,
    ) -> Result<Option<SeekOperation>, Error<T::BusError>> {
        self.fw_update_burst_write(controllers, read_result.read_data).await?;
        if read_result.is_complete() {
            Ok(Some(self.state.next_seek().map_err(Error::Pd)?))
        } else {
            Ok(None)
        }
    }

    /// Process the rest of the update header block
    /// Returns the next stream operation or None if the current operation is not complete
    async fn process_header_rest(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        read_result: &ReadResult<'_>,
    ) -> Result<Option<SeekOperation>, Error<T::BusError>> {
        self.fw_update_burst_write(controllers, read_result.read_data).await?;
        if read_result.is_complete() {
            // Full header has been written
            // Validate and proceed to the next state
            delay.delay_ms(TFUI_BURST_WRITE_DELAY_MS).await;
            trace!("Validing header");
            self.fw_update_validate_stream(controllers, delay, HEADER_BLOCK_INDEX)
                .await?;
            trace!("Header validated");
            Ok(Some(self.state.next_seek().map_err(Error::Pd)?))
        } else {
            Ok(None)
        }
    }

    /// Read update arguments for the current block
    async fn read_block_args(&mut self, read_result: &ReadResult<'_>) -> Result<Option<TfudArgs>, Error<T::BusError>> {
        self.block_args = None;
        let current = read_result.read_state.current;
        let read_len = read_result.read_data.len();
        self.args_buffer[current..current + read_len].copy_from_slice(read_result.read_data);

        if read_result.is_complete() {
            // We have the full header metadata
            let (mut args, _): (TfudArgs, _) =
                bincode::decode_from_slice(&self.args_buffer, config::standard().with_fixed_int_encoding())
                    .map_err(|_| PdError::Serialize)?;

            // Override broadcast address if specified
            args.broadcast_u16_address = self.config.broadcast_addr.unwrap_or(args.broadcast_u16_address);

            self.block_args = Some(args);
            Ok(self.block_args)
        } else {
            // Still waiting for the rest of the header
            Ok(None)
        }
    }

    /// Read data block header and proceed to the next state
    /// Returns the next stream operation or None if the current operation is not complete
    async fn read_data_block_header(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        read_result: &ReadResult<'_>,
    ) -> Result<Option<SeekOperation>, Error<T::BusError>> {
        if let Some(args) = self.read_block_args(read_result).await? {
            // We have the full header metadata
            self.fw_update_stream_data(controllers, delay, &args).await?;

            Ok(Some(self.state.next_seek_nop(read_result.position)?))
        } else {
            Ok(None)
        }
    }

    /// Read data block and proceed to the next state
    /// Returns the next stream operation or None if the current operation is not complete
    async fn read_data_block(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        read_result: &ReadResult<'_>,
        block_index: usize,
    ) -> Result<Option<SeekOperation>, Error<T::BusError>> {
        self.fw_update_burst_write(controllers, read_result.read_data).await?;

        if read_result.is_complete() {
            // Full block has been written
            // Validate and proceed to the next state
            delay.delay_ms(TFUD_BURST_WRITE_DELAY_MS).await;
            self.fw_update_validate_stream(controllers, delay, data_block_index_to_block_index(block_index))
                .await?;

            let update_args = self.update_args.ok_or(Error::Pd(PdError::InvalidParams))?;
            Ok(Some(
                self.state
                    .next_seek_block_args(&update_args, self.image_size)
                    .map_err(Error::Pd)?,
            ))
        } else {
            Ok(None)
        }
    }

    /// Read the image size from the header block and proceed to the next state
    /// Returns the next stream operation or None if the current operation is not complete
    async fn read_image_size(
        &mut self,
        controllers: &mut [&mut T],
        read_result: &ReadResult<'_>,
    ) -> Result<Option<SeekOperation>, Error<T::BusError>> {
        let current = read_result.read_state.current;
        let read_len = read_result.read_data.len();
        self.args_buffer[current..current + read_len].copy_from_slice(read_result.read_data);
        self.fw_update_burst_write(controllers, read_result.read_data).await?;
        if read_result.is_complete() {
            // We have the full image size
            let (image_size, _): (u32, _) =
                bincode::decode_from_slice(&self.args_buffer, config::standard().with_fixed_int_encoding())
                    .map_err(|_| PdError::Serialize)?;

            self.image_size = image_size as usize;
            Ok(Some(self.state.next_seek_nop(read_result.position).map_err(Error::Pd)?))
        } else {
            Ok(None)
        }
    }

    /// Read app config block header and proceed to the next state
    /// Returns the next stream operation or None if the current operation is not complete
    async fn read_config_block_header(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        read_result: &ReadResult<'_>,
    ) -> Result<Option<SeekOperation>, Error<T::BusError>> {
        if let Some(args) = self.read_block_args(read_result).await? {
            // We have the full header metadata
            self.fw_update_stream_data(controllers, delay, &args).await?;

            Ok(Some(self.state.next_seek_nop(read_result.position)?))
        } else {
            Ok(None)
        }
    }

    /// Read config block and proceed to the next state
    /// Returns the next stream operation or None if the current operation is not complete
    async fn read_config_block(
        &mut self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
        read_result: &ReadResult<'_>,
    ) -> Result<Option<SeekOperation>, Error<T::BusError>> {
        self.fw_update_burst_write(controllers, read_result.read_data).await?;

        if read_result.is_complete() {
            // Full block has been written
            // Validate and proceed to the next state
            delay.delay_ms(TFUD_BURST_WRITE_DELAY_MS).await;
            self.fw_update_validate_stream(controllers, delay, APP_CONFIG_BLOCK_INDEX)
                .await?;

            // Proceed to the next state
            trace!("FW update complete");
            Ok(Some(self.state.next_seek_nop(read_result.position).map_err(Error::Pd)?))
        } else {
            Ok(None)
        }
    }

    /// Abort the FW update process
    pub async fn abort_fw_update(self, controllers: &mut [&mut T], delay: &mut impl DelayNs) {
        abort_fw_update(controllers, delay).await
    }

    /// Complete the FW update process
    pub async fn complete_fw_update(
        self,
        controllers: &mut [&mut T],
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<T::BusError>> {
        // The update blob wasn't completely supplied
        if self.state != State::Complete {
            return Err(PdError::Failed.into());
        }

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

/// Disable all interrupts during the reset into FW update mode
pub async fn disable_all_interrupts<T: UpdateTarget>(
    controllers: &mut [&mut T],
    out_guards: &mut [Option<T::Guard>],
) -> Result<(), Error<T::BusError>> {
    for (guard, controller) in zip(out_guards.iter_mut(), controllers.iter_mut()) {
        *guard = Some(controller.disable_all_interrupts_guarded().await?);
    }
    Ok(())
}

/// Re-enable interrupts on port 0 only, other ports don't respond to interrupts in FW update mode
pub async fn enable_port0_interrupts<T: UpdateTarget>(
    controllers: &mut [&mut T],
    out_guards: &mut [Option<T::Guard>],
) -> Result<(), Error<T::BusError>> {
    for (guard, controller) in zip(out_guards.iter_mut(), controllers.iter_mut()) {
        *guard = Some(controller.enable_interrupt_guarded(PORT0, true).await?);
    }
    Ok(())
}

/// General FW update function
///
/// interrupt_guards have a length twice of that of controllers
pub async fn perform_fw_update_borrowed<T: UpdateTarget>(
    controllers: &mut [&mut T],
    interrupt_guards: &mut [Option<T::Guard>],
    delay: &mut impl DelayNs,
    config: UpdateConfig,
    pd_fw_bytes: &[u8],
) -> Result<(), Error<T::BusError>> {
    // Need two sets of interrupt guards for each controller
    if interrupt_guards.len() != 2 * controllers.len() {
        return Err(PdError::InvalidParams.into());
    }

    let mut updater = BorrowedUpdater::with_config(config);
    let half = interrupt_guards.len() / 2;

    // Disable all interrupts while we're entering FW update mode
    // These go in the second half of the interrupt_guards array so they get dropped last
    disable_all_interrupts(controllers, &mut interrupt_guards[half..]).await?;
    info!("Starting update");
    let result = updater.start_fw_update(controllers, delay).await;
    info!("Update started");

    // Re-enable interrupts on port 0 only
    // These go in the first half of the interrupt_guards array so they get dropped first
    enable_port0_interrupts(controllers, &mut interrupt_guards[0..half]).await?;

    match result {
        Err(e) => {
            error!("Failed to enter FW update mode");
            Err(e)
        }
        Ok(mut updater) => {
            info!("Sending chunks");
            for chunk in pd_fw_bytes.chunks(UPDATE_CHUNK_LENGTH) {
                match updater.write_bytes(controllers, delay, chunk).await {
                    Err(e) => {
                        error!("Failed to write chunk");
                        updater.abort_fw_update(controllers, delay).await;
                        return Err(e);
                    }
                    Ok(true) => {
                        info!("Update contents written");
                        break;
                    }
                    _ => {}
                }
            }

            updater.complete_fw_update(controllers, delay).await
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::asynchronous::interrupt::InterruptGuard;
    use crate::test::{generate_mock_fw, Delay};
    use crate::MAX_SUPPORTED_PORTS;
    extern crate std;

    /// Simple mock update target for testing that validates the length of the data written
    #[derive(Debug)]
    struct UpdateTargetNoop {
        pub expected_write_len: Option<usize>,
        pub write_len: usize,
    }

    impl UpdateTargetNoop {
        pub fn new() -> Self {
            Self {
                expected_write_len: None,
                write_len: 0,
            }
        }
    }

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
            args: &TfuiArgs,
        ) -> Result<ReturnValue, Error<Self::BusError>> {
            if let Some(expected_len) = self.expected_write_len {
                if expected_len != self.write_len {
                    panic!("Expected length {} but got {}", expected_len, self.write_len);
                }
            }
            self.write_len = 0;
            self.expected_write_len = Some(args.data_len as usize);
            Ok(ReturnValue::Success)
        }

        async fn fw_update_mode_exit(&mut self, _delay: &mut impl DelayNs) -> Result<(), Error<Self::BusError>> {
            Ok(())
        }

        async fn fw_update_validate_stream(
            &mut self,
            _delay: &mut impl DelayNs,
            _block_index: usize,
        ) -> Result<TfuqBlockStatus, Error<Self::BusError>> {
            Ok(TfuqBlockStatus::DataValidAndAuthentic)
        }

        async fn fw_update_stream_data(
            &mut self,
            _delay: &mut impl DelayNs,
            args: &TfudArgs,
        ) -> Result<(), Error<Self::BusError>> {
            if let Some(expected_len) = self.expected_write_len {
                if expected_len != self.write_len {
                    panic!("Expected length {} but got {}", expected_len, self.write_len);
                }
            }
            self.write_len = 0;
            self.expected_write_len = Some(args.data_len as usize);
            Ok(())
        }

        async fn fw_update_complete(&mut self, _delay: &mut impl DelayNs) -> Result<(), Error<Self::BusError>> {
            Ok(())
        }

        async fn fw_update_burst_write(&mut self, _address: u8, data: &[u8]) -> Result<(), Error<Self::BusError>> {
            self.write_len += data.len();
            Ok(())
        }
    }

    /// Test complete FW update flow
    #[tokio::test]
    async fn test_fw_update() {
        let mut delay = Delay {};
        let mut target = UpdateTargetNoop::new();
        let mut controllers = [&mut target];
        let mut guards = [const { None }; 2];
        let fw_mock = &generate_mock_fw();

        perform_fw_update_borrowed(
            &mut controllers,
            &mut guards,
            &mut delay,
            UpdateConfig::default(),
            fw_mock,
        )
        .await
        .unwrap();
    }

    /// Test return value of write_bytes when the update is complete
    #[tokio::test]
    async fn test_fw_update_completion_write_bytes() {
        let mut delay = Delay {};
        let mut target = UpdateTargetNoop::new();
        let mut controllers = [&mut target];
        let fw_mock = &generate_mock_fw();

        // Don't need to do anything with interrupts since this isn't actual hardware
        let mut updater = BorrowedUpdater::default();
        let mut updater = updater.start_fw_update(&mut controllers, &mut delay).await.unwrap();

        for chunk in fw_mock.chunks(UPDATE_CHUNK_LENGTH) {
            if updater.write_bytes(&mut controllers, &mut delay, chunk).await.unwrap() {
                // Update contents have been written
                break;
            }
        }

        // Validate that write_bytes returns Ok(true) when the update is complete
        assert_eq!(
            updater
                .write_bytes(&mut controllers, &mut delay, &[0u8; UPDATE_CHUNK_LENGTH])
                .await,
            Ok(true)
        );
        assert_eq!(updater.state, State::Complete);

        // Validate that write_bytes returns Ok(true) every time after the update is complete
        assert_eq!(
            updater
                .write_bytes(&mut controllers, &mut delay, &[0u8; UPDATE_CHUNK_LENGTH])
                .await,
            Ok(true)
        );
        assert_eq!(updater.state, State::Complete);
    }

    /// Test error checking around number of interrupt guards and controllers
    #[tokio::test]
    async fn test_fw_update_guards_count() {
        let mut delay = Delay {};
        let mut target = UpdateTargetNoop::new();
        let mut controllers = [&mut target];
        let mut guards = [const { None }; 1];
        let fw_mock = &generate_mock_fw();

        assert_eq!(
            perform_fw_update_borrowed(
                &mut controllers,
                &mut guards,
                &mut delay,
                UpdateConfig::default(),
                fw_mock,
            )
            .await,
            Err(Error::Pd(PdError::InvalidParams))
        );
    }
}
