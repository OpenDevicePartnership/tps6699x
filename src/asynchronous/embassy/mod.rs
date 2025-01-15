//! High-level API that uses embassy_sync wakers
// This code holds refcells across await points but this is controlled within the code using scope

use core::sync::atomic::AtomicBool;

use defmt::error;
use embassy_sync::blocking_mutex::raw::{NoopRawMutex, RawMutex};
use embassy_sync::mutex::{Mutex, MutexGuard};
use embassy_sync::signal::Signal;
use embassy_time::{with_timeout, Duration};
use embedded_hal::digital::InputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::asynchronous::controller::PdController;
use embedded_usb_pd::{Error, PdError, PortId};

use crate::asynchronous::internal;
use crate::command::*;
use crate::registers::field_sets::IntEventBus1;
use crate::{registers, Mode};

pub mod fw_update;
pub mod task;

pub const NUM_PORTS: usize = 2;
pub const PORT0: PortId = PortId(0);
pub const PORT1: PortId = PortId(1);

pub struct Controller<M: RawMutex, B: I2c> {
    inner: Mutex<M, internal::Tps6699x<B>>,
    interrupt_waker: Signal<NoopRawMutex, (IntEventBus1, IntEventBus1)>,
    interrupts_enabled: [AtomicBool; NUM_PORTS],
}

impl<M: RawMutex, B: I2c> Controller<M, B> {
    pub fn new(bus: B, addr: [u8; NUM_PORTS]) -> Result<Self, Error<B::Error>> {
        Ok(Self {
            inner: Mutex::new(internal::Tps6699x::new(bus, addr)),
            interrupt_waker: Signal::new(),
            interrupts_enabled: [AtomicBool::new(true), AtomicBool::new(true)],
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

    pub(crate) async fn fw_update_mode_enter(&mut self) -> Result<(), Error<B::Error>> {
        self.enable_interrupts(false);
        let result = {
            let mut inner = self.lock_inner().await;
            with_timeout(Duration::from_millis(TFUS_TIMEOUT_MS.into()), inner.execute_tfus()).await
        };

        if result.is_err() {
            error!("Enter FW mode timeout");
            self.enable_interrupts(true);
            return PdError::Timeout.into();
        }

        if let Err(e) = result.unwrap() {
            self.enable_interrupts(true);
            return Err(e);
        };

        // PORT0 is always a valid port
        self.enable_interrupt(PORT0, true).unwrap();

        Ok(())
    }

    pub(crate) async fn fw_update_init(&mut self, args: &TfuiArgs) -> Result<ReturnValue, Error<B::Error>> {
        let mut args_buf = [0u8; PD_FW_HEADER_METADATA_LENGTH as usize];

        args.encode_into_slice(&mut args_buf).map_err(Error::Pd)?;
        self.execute_command(PortId(0), Command::Tfui, Some(&args_buf), None)
            .await
    }

    pub async fn fw_update_mode_exit(&mut self) -> Result<(), Error<B::Error>> {
        let result = with_timeout(
            Duration::from_millis(RESET_TIMEOUT_MS.into()),
            self.execute_command(PortId(0), Command::Tfue, None, None),
        )
        .await;

        // Reset the controller if we failed to exit fw update mode
        if result.is_err() {
            error!("FW update exit timeout, attempting to reset");
            let mut delay = embassy_time::Delay;
            self.reset(&mut delay).await?;
            return Ok(());
        }

        if result.unwrap()? != ReturnValue::Success {
            error!("FW update exit command error, attempting to reset");
            let mut delay = embassy_time::Delay;
            self.reset(&mut delay).await?;
            return Ok(());
        }

        Ok(())
    }

    pub(crate) async fn fw_update_validate_stream(
        &mut self,
        block_index: usize,
    ) -> Result<TfuqBlockStatus, Error<B::Error>> {
        let args = TfuqArgs {
            command: TfuqCommandType::QueryTfuStatus,
            status_query: TfuqStatusQuery::StatusInProgress,
        };

        let mut arg_bytes = [0u8; 2];
        let mut return_bytes = [0u8; TFUQ_RETURN_LEN];

        args.encode_into_slice(&mut arg_bytes).map_err(Error::Pd)?;

        let result = with_timeout(
            Duration::from_millis(TFUE_TIMEOUT_MS.into()),
            self.execute_command(PortId(0), Command::Tfuq, Some(&arg_bytes), Some(&mut return_bytes)),
        )
        .await;

        if result.is_err() {
            error!("Validate stream timeout");
            return PdError::Timeout.into();
        }

        if result.unwrap()? != ReturnValue::Success {
            error!("Validate stream failed");
            return PdError::Failed.into();
        }

        let data = TfuqReturnValue::decode_from_slice(&return_bytes).map_err(Error::Pd)?;
        TfuqBlockStatus::try_from(data.block_status[block_index]).map_err(Error::Pd)
    }

    pub(crate) async fn fw_update_stream_data(&mut self, args: &TfudArgs) -> Result<(), Error<B::Error>> {
        let mut arg_bytes = [0u8; TFUD_ARGS_LEN];

        TfudArgs::encode_into_slice(args, &mut arg_bytes).map_err(Error::Pd)?;

        let result = with_timeout(
            Duration::from_millis(TFUE_TIMEOUT_MS.into()),
            self.execute_command(PortId(0), Command::Tfud, Some(&arg_bytes), None),
        )
        .await;

        if result.is_err() {
            error!("Stream data timeout");
            return PdError::Timeout.into();
        }

        if result.unwrap()? != ReturnValue::Success {
            error!("Stream data failed");
            return PdError::Failed.into();
        }

        Ok(())
    }

    pub(crate) async fn fw_update_complete(&mut self) -> Result<(), Error<B::Error>> {
        self.enable_interrupts(false);
        let result = {
            let mut inner = self.lock_inner().await;
            with_timeout(Duration::from_millis(RESET_TIMEOUT_MS.into()), inner.execute_tfuc()).await
        };
        self.enable_interrupts(true);

        if result.is_err() {
            error!("Complete timeout");
            return PdError::Timeout.into();
        }

        result.unwrap()
    }

    pub fn enable_interrupt(&mut self, port: PortId, enabled: bool) -> Result<(), Error<B::Error>> {
        if port.0 > 1 {
            return PdError::InvalidParams.into();
        }

        self.controller.interrupts_enabled[port.0 as usize].store(enabled, core::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    pub fn enable_interrupts(&mut self, enabled: bool) {
        for port in 0..2 {
            self.enable_interrupt(PortId(port), enabled).unwrap();
        }
    }
}

impl<'a, M: RawMutex, B: I2c> PdController<B::Error> for Tps6699x<'a, M, B> {
    async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        self.enable_interrupts(false);
        let result = {
            let mut inner = self.lock_inner().await;
            with_timeout(Duration::from_millis(RESET_TIMEOUT_MS.into()), inner.reset(delay)).await
        };
        self.enable_interrupts(true);

        if result.is_err() {
            error!("Reset timeout");
            return PdError::Timeout.into();
        }

        result.unwrap()
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
            let interrupts_enabled: [bool; 2] = [
                self.controller.interrupts_enabled[0].load(core::sync::atomic::Ordering::SeqCst),
                self.controller.interrupts_enabled[1].load(core::sync::atomic::Ordering::SeqCst),
            ];

            let mut inner = self.lock_inner().await;
            for port in 0..NUM_PORTS {
                let port_id = PortId(port as u8);

                if !interrupts_enabled[port] {
                    continue;
                }

                // Early exit if checking the last port cleared the interrupt
                // TODO: better error handling
                let result = int.is_high();
                if result.is_err() || !result.unwrap() {
                    flags[port as usize] = IntEventBus1::new_zero();
                }

                let result = inner.clear_interrupt(port_id).await?;
                flags[port as usize] = result;
            }
        }

        let flags = (flags[0], flags[1]);
        self.controller.interrupt_waker.signal(flags);
        Ok(flags)
    }
}
