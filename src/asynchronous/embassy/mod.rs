//! This module contains a high-level API uses embassy synchronization types
use core::iter::zip;
use core::sync::atomic::AtomicBool;

use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::{Mutex, MutexGuard};
use embassy_sync::signal::Signal;
use embassy_time::{with_timeout, Delay, Duration};
use embedded_hal::digital::InputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::asynchronous::controller::PdController;
use embedded_usb_pd::{Error, PdError, PortId};

use super::interrupt::{self, InterruptController};
use crate::asynchronous::internal;
use crate::command::*;
use crate::registers::field_sets::IntEventBus1;
use crate::registers::{self};
use crate::{error, Mode, MAX_SUPPORTED_PORTS};

pub mod fw_update;
pub mod task;

pub mod controller {
    use super::*;
    use crate::{TPS66993_NUM_PORTS, TPS66994_NUM_PORTS};

    /// Controller struct. This struct is meant to be created and then immediately broken into its parts
    pub struct Controller<M: RawMutex, B: I2c> {
        /// Low-level TPS6699x driver
        pub(super) inner: Mutex<M, internal::Tps6699x<B>>,
        /// Signal for awaiting an interrupt
        pub(super) interrupt_waker: Signal<M, [IntEventBus1; MAX_SUPPORTED_PORTS]>,
        /// Current interrupt state
        pub(super) interrupts_enabled: [AtomicBool; MAX_SUPPORTED_PORTS],
        /// Number of active ports
        pub(super) num_ports: usize,
    }

    impl<M: RawMutex, B: I2c> Controller<M, B> {
        /// Private constructor
        pub fn new(bus: B, addr: [u8; MAX_SUPPORTED_PORTS], num_ports: usize) -> Result<Self, Error<B::Error>> {
            Ok(Self {
                inner: Mutex::new(internal::Tps6699x::new(bus, addr, num_ports)),
                interrupt_waker: Signal::new(),
                interrupts_enabled: [const { AtomicBool::new(true) }; MAX_SUPPORTED_PORTS],
                num_ports,
            })
        }

        /// Create a new controller for the TPS66993
        pub fn new_tps66993(bus: B, addr: u8) -> Result<Self, Error<B::Error>> {
            Self::new(bus, [addr, 0], TPS66993_NUM_PORTS)
        }

        /// Create a new controller for the TPS66994
        pub fn new_tps66994(bus: B, addr: [u8; TPS66994_NUM_PORTS]) -> Result<Self, Error<B::Error>> {
            Self::new(bus, addr, TPS66994_NUM_PORTS)
        }

        /// Breaks the controller into its parts
        pub fn make_parts(&mut self) -> (Tps6699x<'_, M, B>, Interrupt<'_, M, B>) {
            let tps = Tps6699x { controller: self };
            let interrupt = Interrupt { controller: self };
            (tps, interrupt)
        }

        /// Enable or disable interrupts for the given ports
        pub(super) fn enable_interrupts(&self, enabled: [bool; MAX_SUPPORTED_PORTS]) {
            for (enabled, s) in zip(enabled.iter(), self.interrupts_enabled.iter()) {
                s.store(*enabled, core::sync::atomic::Ordering::SeqCst);
            }
        }

        /// Returns current interrupt state
        pub(super) fn interrupts_enabled(&self) -> [bool; MAX_SUPPORTED_PORTS] {
            let mut interrupts_enabled = [false; MAX_SUPPORTED_PORTS];
            for (copy, enabled) in zip(interrupts_enabled.iter_mut(), self.interrupts_enabled.iter()) {
                *copy = enabled.load(core::sync::atomic::Ordering::SeqCst);
            }

            interrupts_enabled
        }
    }
}

/// Struct for controlling a TP6699x device
pub struct Tps6699x<'a, M: RawMutex, B: I2c> {
    controller: &'a controller::Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> Tps6699x<'a, M, B> {
    /// Locks the inner device
    async fn lock_inner(&mut self) -> MutexGuard<'_, M, internal::Tps6699x<B>> {
        self.controller.inner.lock().await
    }

    /// Wrapper for `get_port_status``
    pub async fn get_port_status(&mut self, port: PortId) -> Result<registers::field_sets::Status, Error<B::Error>> {
        self.lock_inner().await.get_port_status(port).await
    }

    /// Wrapper for `get_active_pdo_contract`
    pub async fn get_active_pdo_contract(
        &mut self,
        port: PortId,
    ) -> Result<registers::field_sets::ActivePdoContract, Error<B::Error>> {
        self.lock_inner().await.get_active_pdo_contract(port).await
    }

    /// Wrapper for `get_active_rdo_contract`
    pub async fn get_active_rdo_contract(
        &mut self,
        port: PortId,
    ) -> Result<registers::field_sets::ActiveRdoContract, Error<B::Error>> {
        self.lock_inner().await.get_active_rdo_contract(port).await
    }

    /// Wrapper for `get_mode`
    pub async fn get_mode(&mut self) -> Result<Mode, Error<B::Error>> {
        self.lock_inner().await.get_mode().await
    }

    /// Wrapper for `get_fw_version`
    pub async fn get_fw_version(&mut self) -> Result<u32, Error<B::Error>> {
        self.lock_inner().await.get_fw_version().await
    }

    /// Wrapper for `get_customer_use`
    pub async fn get_customer_use(&mut self) -> Result<u64, Error<B::Error>> {
        self.lock_inner().await.get_customer_use().await
    }

    /// Wrapper for `get_power_path_status`
    pub async fn get_power_path_status(
        &mut self,
        port: PortId,
    ) -> Result<registers::field_sets::PowerPathStatus, Error<B::Error>> {
        self.lock_inner().await.get_power_path_status(port).await
    }

    /// Wrapper for `get_pd_status`
    pub async fn get_pd_status(&mut self, port: PortId) -> Result<registers::field_sets::PdStatus, Error<B::Error>> {
        self.lock_inner().await.get_pd_status(port).await
    }

    /// Wrapper for `get_port_control`
    pub async fn get_port_control(
        &mut self,
        port: PortId,
    ) -> Result<registers::field_sets::PortControl, Error<B::Error>> {
        self.lock_inner().await.get_port_control(port).await
    }

    /// Wrapper for `set_port_control`
    pub async fn set_port_control(
        &mut self,
        port: PortId,
        control: registers::field_sets::PortControl,
    ) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.set_port_control(port, control).await
    }

    /// Wrapper for `get_system_config`
    pub async fn get_system_config(&mut self) -> Result<registers::field_sets::SystemConfig, Error<B::Error>> {
        self.lock_inner().await.get_system_config().await
    }

    /// Wrapper for `set_system_config`
    pub async fn set_system_config(
        &mut self,
        config: registers::field_sets::SystemConfig,
    ) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.set_system_config(config).await
    }

    /// Wrapper for `enable_source`
    pub async fn enable_source(&mut self, port: PortId, enable: bool) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.enable_source(port, enable).await
    }

    /// Returns the number of ports
    pub fn num_ports(&self) -> usize {
        self.controller.num_ports
    }

    /// Wait for an interrupt to occur that satisfies the given predicate
    pub async fn wait_interrupt(
        &mut self,
        clear_current: bool,
        f: impl Fn(PortId, IntEventBus1) -> bool,
    ) -> [IntEventBus1; MAX_SUPPORTED_PORTS] {
        if clear_current {
            self.controller.interrupt_waker.reset();
        }

        loop {
            let flags = self.controller.interrupt_waker.wait().await;
            for (port, flag) in flags.iter().enumerate() {
                if f(PortId(port as u8), *flag) {
                    return flags;
                }
            }
        }
    }

    /// Execute the given command with no timeout
    async fn execute_command_no_timeout(
        &mut self,
        port: PortId,
        cmd: Command,
        indata: Option<&[u8]>,
        outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        {
            let mut inner = self.lock_inner().await;
            let mut delay = Delay;
            inner.send_command(&mut delay, port, cmd, indata).await?;
        }

        self.wait_interrupt(false, |p, flags| p == port && flags.cmd_1_completed())
            .await;
        {
            let mut inner = self.lock_inner().await;
            inner.read_command_result(port, outdata).await
        }
    }

    /// Execute the given command with a timeout
    async fn execute_command(
        &mut self,
        port: PortId,
        cmd: Command,
        timeout_ms: u32,
        indata: Option<&[u8]>,
        outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        let result = with_timeout(
            Duration::from_millis(timeout_ms.into()),
            self.execute_command_no_timeout(port, cmd, indata, outdata),
        )
        .await;
        if result.is_err() {
            error!("Command {:#?} timed out", cmd);
            return PdError::Timeout.into();
        }

        result.unwrap()
    }

    async fn execute_srdy(&mut self, port: PortId, switch: SrdySwitch) -> Result<ReturnValue, Error<B::Error>> {
        let arg_bytes = [switch.into()];
        self.execute_command(port, Command::Srdy, SRDY_TIMEOUT_MS, Some(&arg_bytes), None)
            .await
    }

    async fn execute_sryr(&mut self, port: PortId) -> Result<ReturnValue, Error<B::Error>> {
        self.execute_command(port, Command::Sryr, SRYR_TIMEOUT_MS, None, None)
            .await
    }

    /// Enable or disable the given power path
    pub async fn enable_sink_path(&mut self, port: PortId, enable: bool) -> Result<(), Error<B::Error>> {
        if enable {
            self.execute_srdy(
                port,
                match port.0 {
                    0 => Ok(SrdySwitch::PpExt1),
                    1 => Ok(SrdySwitch::PpExt2),
                    _ => PdError::InvalidPort.into(),
                }?,
            )
            .await?;
        } else {
            self.execute_sryr(port).await?;
        }

        Ok(())
    }
}

impl<'a, M: RawMutex, B: I2c> interrupt::InterruptController for Tps6699x<'a, M, B> {
    type Guard = InterruptGuard<'a, M, B>;
    type BusError = B::Error;

    async fn interrupts_enabled(&self) -> Result<[bool; MAX_SUPPORTED_PORTS], Error<Self::BusError>> {
        Ok(self.controller.interrupts_enabled())
    }

    async fn enable_interrupts_guarded(
        &mut self,
        enabled: [bool; MAX_SUPPORTED_PORTS],
    ) -> Result<Self::Guard, Error<Self::BusError>> {
        Ok(InterruptGuard::new(self.controller, enabled))
    }
}

impl<M: RawMutex, B: I2c> PdController for Tps6699x<'_, M, B> {
    type BusError = B::Error;

    async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<Self::BusError>> {
        let _guard = self.disable_all_interrupts_guarded().await;
        let mut inner = self.lock_inner().await;
        inner.reset(delay, &Default::default()).await
    }
}

pub struct Interrupt<'a, M: RawMutex, B: I2c> {
    controller: &'a controller::Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> Interrupt<'a, M, B> {
    async fn lock_inner(&mut self) -> MutexGuard<'_, M, internal::Tps6699x<B>> {
        self.controller.inner.lock().await
    }

    /// Process interrupts
    pub async fn process_interrupt(
        &mut self,
        int: &mut impl InputPin,
    ) -> Result<[IntEventBus1; MAX_SUPPORTED_PORTS], Error<B::Error>> {
        let mut flags = [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS];

        {
            let interrupts_enabled = self.controller.interrupts_enabled();
            let mut inner = self.lock_inner().await;
            for port in 0..inner.num_ports() {
                let port_id = PortId(port as u8);

                if !interrupts_enabled[port] {
                    continue;
                }

                let result = int.is_high();
                if result.is_err() {
                    error!("Failed to read interrupt line");
                    return PdError::Failed.into();
                }

                // Early exit if checking the last port cleared the interrupt
                if result.unwrap() {
                    continue;
                }

                flags[port] = inner.clear_interrupt(port_id).await?;
            }
        }

        self.controller.interrupt_waker.signal(flags);
        Ok(flags)
    }
}

/// Restores the original interrupt state when dropped
pub struct InterruptGuard<'a, M: RawMutex, B: I2c> {
    target_state: [bool; MAX_SUPPORTED_PORTS],
    controller: &'a controller::Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> InterruptGuard<'a, M, B> {
    fn new(controller: &'a controller::Controller<M, B>, enabled: [bool; MAX_SUPPORTED_PORTS]) -> Self {
        let target_state = controller.interrupts_enabled();
        controller.enable_interrupts(enabled);
        Self {
            target_state,
            controller,
        }
    }
}

impl<M: RawMutex, B: I2c> Drop for InterruptGuard<'_, M, B> {
    fn drop(&mut self) {
        self.controller.enable_interrupts(self.target_state);
    }
}

impl<M: RawMutex, B: I2c> interrupt::InterruptGuard for InterruptGuard<'_, M, B> {}
