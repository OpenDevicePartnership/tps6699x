//! This module contains a high-level API uses embassy synchronization types
use core::array::from_fn;
use core::cell::RefCell;
use core::future::Future;
use core::iter::zip;
use core::sync::atomic::AtomicBool;

use bincode::config;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::blocking_mutex::Mutex as BlockingMutex;
use embassy_sync::mutex::{Mutex, MutexGuard};
use embassy_sync::signal::Signal;
use embassy_time::{with_timeout, Duration, Timer};
use embedded_hal::digital::InputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::ado::{self, Ado};
use embedded_usb_pd::pdinfo::AltMode;
use embedded_usb_pd::{pdo, Error, LocalPortId, PdError};
use itertools::izip;

use super::interrupt::{self, InterruptController};
use crate::asynchronous::internal;
use crate::command::{gcdm, muxr, trig, vdms, Command, ReturnValue, SrdySwitch};
use crate::registers::autonegotiate_sink::AutoComputeSinkMaxVoltage;
use crate::registers::field_sets::IntEventBus1;
use crate::{error, registers, trace, warn, DeviceError, Mode, MAX_SUPPORTED_PORTS};

pub mod fw_update;
pub mod rx_caps;
pub mod task;
pub mod ucsi;

pub mod controller {
    use super::*;
    use crate::{TPS66993_NUM_PORTS, TPS66994_NUM_PORTS};

    /// Controller struct. This struct is meant to be created and then immediately broken into its parts
    pub struct Controller<M: RawMutex, B: I2c> {
        /// Low-level TPS6699x driver
        pub(super) inner: Mutex<M, internal::Tps6699x<B>>,
        /// Signal for awaiting an interrupt
        pub(super) interrupt_waker: Signal<M, [IntEventBus1; MAX_SUPPORTED_PORTS]>,
        /// Interrupts read from hardware that still need a confirmed W1C write.
        pending_interrupt_clears: BlockingMutex<M, RefCell<[IntEventBus1; MAX_SUPPORTED_PORTS]>>,
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
                pending_interrupt_clears: BlockingMutex::new(RefCell::new(
                    [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS],
                )),
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

        pub(super) fn commit_interrupts(&self, port: usize, flags: IntEventBus1) {
            if flags == IntEventBus1::new_zero() {
                return;
            }

            self.pending_interrupt_clears.lock(|pending| {
                if let Some(port_pending) = pending.borrow_mut().get_mut(port) {
                    *port_pending |= flags;
                }
            });

            let mut accumulated = self
                .interrupt_waker
                .try_take()
                .unwrap_or([IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS]);
            if let Some(port_flags) = accumulated.get_mut(port) {
                *port_flags |= flags;
            }
            self.interrupt_waker.signal(accumulated);
        }

        pub(super) fn pending_interrupt_clear(&self, port: usize) -> IntEventBus1 {
            self.pending_interrupt_clears
                .lock(|pending| pending.borrow().get(port).copied().unwrap_or(IntEventBus1::new_zero()))
        }

        pub(super) fn complete_interrupt_clear(&self, port: usize, cleared: IntEventBus1) {
            self.pending_interrupt_clears.lock(|pending| {
                if let Some(port_pending) = pending.borrow_mut().get_mut(port) {
                    *port_pending &= !cleared;
                }
            });
        }
    }
}

/// Struct for controlling a TP6699x device
pub struct Tps6699x<'a, M: RawMutex, B: I2c> {
    controller: &'a controller::Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> Tps6699x<'a, M, B> {
    /// Locks the inner device
    pub fn lock_inner(&mut self) -> impl Future<Output = MutexGuard<'_, M, internal::Tps6699x<B>>> {
        self.controller.inner.lock()
    }

    /// Wrapper for `modify_interrupt_mask`
    pub async fn modify_interrupt_mask(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::field_sets::IntEventBus1) -> registers::field_sets::IntEventBus1,
    ) -> Result<registers::field_sets::IntEventBus1, Error<B::Error>> {
        self.lock_inner().await.modify_interrupt_mask(port, f).await
    }

    /// Wrapper for `modify_interrupt_mask_all`
    pub async fn modify_interrupt_mask_all(
        &mut self,
        f: impl Fn(&mut registers::field_sets::IntEventBus1) -> registers::field_sets::IntEventBus1,
    ) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.modify_interrupt_mask_all(f).await
    }

    /// Wrapper for `get_port_status``
    pub async fn get_port_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::Status, Error<B::Error>> {
        self.lock_inner().await.get_port_status(port).await
    }

    /// Wrapper for `get_active_pdo_contract`
    pub async fn get_active_pdo_contract(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::ActivePdoContract, Error<B::Error>> {
        self.lock_inner().await.get_active_pdo_contract(port).await
    }

    /// Wrapper for `get_active_rdo_contract`
    pub async fn get_active_rdo_contract(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::ActiveRdoContract, Error<B::Error>> {
        self.lock_inner().await.get_active_rdo_contract(port).await
    }

    /// Get the Autonegotiate Sink register (`0x37`).
    pub async fn get_autonegotiate_sink(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::autonegotiate_sink::AutonegotiateSink, Error<B::Error>> {
        self.lock_inner().await.get_autonegotiate_sink(port).await
    }

    /// Set the Autonegotiate Sink register (`0x37`).
    pub async fn set_autonegotiate_sink(
        &mut self,
        port: LocalPortId,
        value: registers::autonegotiate_sink::AutonegotiateSink,
    ) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.set_autonegotiate_sink(port, value).await
    }

    /// Modify the Autonegotiate Sink register (`0x37`).
    pub async fn modify_autonegotiate_sink(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(
            &mut registers::autonegotiate_sink::AutonegotiateSink,
        ) -> registers::autonegotiate_sink::AutonegotiateSink,
    ) -> Result<registers::autonegotiate_sink::AutonegotiateSink, Error<B::Error>> {
        self.lock_inner().await.modify_autonegotiate_sink(port, f).await
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
        port: LocalPortId,
    ) -> Result<registers::field_sets::PowerPathStatus, Error<B::Error>> {
        self.lock_inner().await.get_power_path_status(port).await
    }

    /// Wrapper for `get_pd_status`
    pub async fn get_pd_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::PdStatus, Error<B::Error>> {
        self.lock_inner().await.get_pd_status(port).await
    }

    /// Wrapper for `get_port_control`
    pub async fn get_port_control(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::PortControl, Error<B::Error>> {
        self.lock_inner().await.get_port_control(port).await
    }

    /// Wrapper for `set_port_control`
    pub async fn set_port_control(
        &mut self,
        port: LocalPortId,
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
    pub async fn enable_source(&mut self, port: LocalPortId, enable: bool) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.enable_source(port, enable).await
    }

    /// Returns the number of ports
    pub fn num_ports(&self) -> usize {
        self.controller.num_ports
    }

    /// Wait for an interrupt to occur that matches any bits in the given mask.
    ///
    /// Drop safety: Safe, unhandled interrupts will be re-signaled.
    pub async fn wait_interrupt_any(
        &mut self,
        clear_current: bool,
        mask: [IntEventBus1; MAX_SUPPORTED_PORTS],
    ) -> [IntEventBus1; MAX_SUPPORTED_PORTS] {
        // No interrupts set, return immediately because there is nothing to wait for
        // Also log a warning because this likely isn't what the user intended
        if mask == [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS] {
            warn!("Interrupt masks are empty, returning immediately");
            return [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS];
        }

        if clear_current {
            self.controller.interrupt_waker.reset();
        }

        let mut accumulated_flags = AccumulatedFlagsAny::new(self.controller, mask);
        loop {
            let flags = self.controller.interrupt_waker.wait().await;
            if let Some(flags) = accumulated_flags.accumulate(flags) {
                return flags;
            }
        }
    }

    /// Execute the given command with no timeout
    async fn execute_command_no_timeout(
        &mut self,
        port: LocalPortId,
        cmd: Command,
        indata: Option<&[u8]>,
        outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        {
            let mut inner = self.lock_inner().await;
            inner.send_command(port, cmd, indata).await?;
        }

        let mut cmd_complete = IntEventBus1::new_zero();
        cmd_complete.set_cmd_1_completed(true);

        let _flags = self
            .wait_interrupt_any(
                false,
                from_fn(|i| {
                    if i == port.0 as usize {
                        cmd_complete
                    } else {
                        IntEventBus1::new_zero()
                    }
                }),
            )
            .await;
        {
            let mut inner = self.lock_inner().await;
            inner.read_command_result(port, outdata, cmd.has_return_value()).await
        }
    }

    /// Execute the given command with a timeout determined by [`Command::timeout`].
    async fn execute_command(
        &mut self,
        port: LocalPortId,
        cmd: Command,
        indata: Option<&[u8]>,
        mut outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        let timeout = cmd.timeout();
        let result = with_timeout(
            timeout,
            self.execute_command_no_timeout(port, cmd, indata, outdata.as_deref_mut()),
        )
        .await;
        if let Ok(result) = result {
            result
        } else {
            error!("Command {:#?} timed out", cmd);
            // See if there's a definite error we can read
            let mut inner = self.lock_inner().await;
            match inner.read_command_result(port, outdata, cmd.has_return_value()).await? {
                ReturnValue::Success => Ok(ReturnValue::Success),
                ReturnValue::Rejected => Err(PdError::Rejected.into()),
                _ => Err(PdError::Timeout.into()),
            }
        }
    }

    async fn execute_srdy(&mut self, port: LocalPortId, switch: SrdySwitch) -> Result<ReturnValue, Error<B::Error>> {
        let arg_bytes = [switch.into()];
        self.execute_command(port, Command::Srdy, Some(&arg_bytes), None).await
    }

    async fn execute_sryr(&mut self, port: LocalPortId) -> Result<ReturnValue, Error<B::Error>> {
        self.execute_command(port, Command::Sryr, None, None).await
    }

    /// Enable or disable the given power path
    pub async fn enable_sink_path(&mut self, port: LocalPortId, enable: bool) -> Result<(), Error<B::Error>> {
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

    /// Trigger an `ANeg` command to autonegotiate the sink contract.
    pub async fn autonegotiate_sink(&mut self, port: LocalPortId) -> Result<(), Error<B::Error>> {
        match self.execute_command(port, Command::Aneg, None, None).await? {
            ReturnValue::Success => Ok(()),
            ReturnValue::Rejected => PdError::Rejected.into(),
            _ => PdError::Failed.into(),
        }
    }

    /// Trigger virtual gpios
    async fn virtual_gpio_trigger(
        &mut self,
        port: LocalPortId,
        edge: trig::Edge,
        cmd: trig::Cmd,
    ) -> Result<ReturnValue, Error<B::Error>> {
        let args = trig::Args { edge, cmd };
        let mut args_buf = [0; trig::ARGS_LEN];

        bincode::encode_into_slice(args, &mut args_buf, config::standard().with_fixed_int_encoding())
            .map_err(|_| Error::Pd(PdError::InvalidParams))?;

        self.execute_command(port, Command::Trig, Some(&args_buf), None).await
    }

    /// Force retimer power on or off
    pub async fn retimer_force_pwr(&mut self, port: LocalPortId, enable: bool) -> Result<(), Error<B::Error>> {
        trace!("retimer_force_pwr: {}", enable);

        let edge = if enable {
            trig::Edge::Rising
        } else {
            trig::Edge::Falling
        };

        self.virtual_gpio_trigger(port, edge, trig::Cmd::RetimerForcePwr)
            .await?;

        Ok(())
    }

    /// Get retimer fw update state
    pub async fn get_rt_fw_update_status(&mut self, port: LocalPortId) -> Result<bool, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        let rt_fw_update_mode = inner.get_intel_vid_status(port).await?.forced_tbt_mode();
        trace!("rt_fw_update_mode: {}", rt_fw_update_mode);
        Ok(rt_fw_update_mode)
    }

    /// set retimer fw update state
    pub async fn set_rt_fw_update_state(&mut self, port: LocalPortId) -> Result<(), Error<B::Error>> {
        // Force RT Pwr On
        self.retimer_force_pwr(port, true).await?;

        let mut inner = self.lock_inner().await;
        let mut port_control = inner.get_port_control(port).await?;
        port_control.set_retimer_fw_update(true);
        inner.set_port_control(port, port_control).await?;
        Ok(())
    }

    /// clear retimer fw update state
    pub async fn clear_rt_fw_update_state(&mut self, port: LocalPortId) -> Result<(), Error<B::Error>> {
        {
            let mut inner = self.lock_inner().await;
            let mut port_control = inner.get_port_control(port).await?;
            port_control.set_retimer_fw_update(false);
            inner.set_port_control(port, port_control).await?;
        }

        // Force RT Pwr Off
        self.retimer_force_pwr(port, false).await?;

        Ok(())
    }

    /// set retimer compliance
    pub async fn set_rt_compliance(&mut self, port: LocalPortId) -> Result<(), Error<B::Error>> {
        {
            // Force RT Pwr On
            self.retimer_force_pwr(port, true).await?;

            let mut inner = self.lock_inner().await;
            let mut tbt_config = inner.get_tbt_config(port).await?;
            tbt_config.set_retimer_compliance_support(true);
            inner.set_tbt_config(port, tbt_config).await?;
        }

        Ok(())
    }

    /// Execute the [`Command::Dbfg`] command.
    pub async fn execute_dbfg(&mut self, port: LocalPortId) -> Result<ReturnValue, Error<B::Error>> {
        self.execute_command(port, Command::Dbfg, None, None).await
    }

    /// Execute the [`Command::Muxr`] command.
    pub async fn execute_muxr(
        &mut self,
        port: LocalPortId,
        input: muxr::Input,
    ) -> Result<ReturnValue, Error<B::Error>> {
        let indata = input.0.to_le_bytes();
        self.execute_command(port, Command::Muxr, Some(&indata), None).await
    }

    /// Execute the [`Command::VDMs`] command.
    pub async fn send_vdms(&mut self, port: LocalPortId, input: vdms::Input) -> Result<ReturnValue, Error<B::Error>> {
        let indata = input.as_bytes();
        self.execute_command(port, Command::VDMs, Some(indata), None).await
    }

    /// Reset the device.
    pub async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        let _guard = self.disable_all_interrupts_guarded().await;
        let mut inner = self.lock_inner().await;
        inner.reset(delay, &Default::default()).await
    }

    /// Execute the [`Command::DISC`] command to disconnect a port for a specified amount of time (in seconds).
    pub async fn execute_disc(
        &mut self,
        port: LocalPortId,
        disconnect_time_s: Option<u8>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        let buf = [disconnect_time_s.unwrap_or(0)];
        self.execute_command(port, Command::DISC, Some(&buf), None).await
    }

    /// Get boot flags
    pub async fn get_boot_flags(&mut self) -> Result<registers::boot_flags::BootFlags, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_boot_flags().await
    }

    /// Get DP status
    pub async fn get_dp_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::dp_status::DpStatus, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_dp_status(port).await
    }

    /// Get Intel VID status
    pub async fn get_intel_vid(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::IntelVidStatus, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_intel_vid_status(port).await
    }

    /// Get USB status
    pub async fn get_usb_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::UsbStatus, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_usb_status(port).await
    }

    /// Get user VID status
    pub async fn get_user_vid(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::UserVidStatus, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_user_vid_status(port).await
    }

    /// Get complete alt-mode status
    pub async fn get_alt_mode_status(&mut self, port: LocalPortId) -> Result<AltMode, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_alt_mode_status(port).await
    }

    /// Set unconstrained power on a port
    pub async fn set_unconstrained_power(&mut self, port: LocalPortId, enable: bool) -> Result<(), Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.set_unconstrained_power(port, enable).await
    }

    /// Get port config
    pub async fn get_port_config(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::port_config::PortConfig, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_port_config(port).await
    }

    /// Set port config
    pub async fn set_port_config(
        &mut self,
        port: LocalPortId,
        config: registers::port_config::PortConfig,
    ) -> Result<(), Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.set_port_config(port, config).await
    }

    /// Get Sx App Config register (`0x20`).
    ///
    /// This register contains the current system power state.
    pub async fn get_sx_app_config(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::SxAppConfig, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_sx_app_config(port).await
    }

    /// Set Sx App Config register (`0x20`).
    ///
    /// Write the current system power state to the PD controller. A change in power state
    /// triggers a new Application Configuration to be applied.
    pub async fn set_sx_app_config(
        &mut self,
        port: LocalPortId,
        state: registers::SystemPowerState,
    ) -> Result<(), Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.set_sx_app_config(port, state).await
    }

    /// Get the discovered SVIDs on a port returned from `Discover SVIDs REQ` messages.
    pub async fn get_discovered_svids(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::discovered_svids::DiscoveredSvids, Error<B::Error>> {
        self.lock_inner().await.get_discovered_svids(port).await
    }

    /// Get Rx ADO
    pub async fn get_rx_ado(
        &mut self,
        port: LocalPortId,
    ) -> Result<Option<Ado>, DeviceError<B::Error, ado::InvalidType>> {
        let mut inner = self.lock_inner().await;
        let ado_raw = inner.get_rx_ado(port).await.map_err(DeviceError::from)?;

        if ado_raw == registers::field_sets::RxAdo::new_zero() {
            // No ADO available
            Ok(None)
        } else {
            Ok(Some(ado_raw.ado().try_into().map_err(DeviceError::Other)?))
        }
    }

    /// Get Rx Attention Vdm
    pub async fn get_rx_attn_vdm(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::RxAttnVdm, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_rx_attn_vdm(port).await
    }

    /// Get Rx Other Vdm
    pub async fn get_rx_other_vdm(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::rx_other_vdm::RxOtherVdm, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_rx_other_vdm(port).await
    }

    /// Set autonegotiate sink max voltage. This may trigger a renegotiation
    pub async fn set_autonegotiate_sink_max_voltage(
        &mut self,
        port: LocalPortId,
        voltage_mv: Option<u16>,
    ) -> Result<(), Error<B::Error>> {
        self.modify_autonegotiate_sink(port, |settings| {
            if let Some(voltage) = voltage_mv {
                settings.set_auto_compute_sink_max_voltage(AutoComputeSinkMaxVoltage::ProvidedByHost);
                settings.set_auto_neg_max_voltage(voltage);
            } else {
                // Auto neg max voltage is ignored if this value is set
                settings.set_auto_compute_sink_max_voltage(AutoComputeSinkMaxVoltage::ComputedByPdController);
            }

            settings.clone()
        })
        .await?;

        // Trigger autonegotiate sink to apply the new max voltage
        // This will result in a rejection if the port is not a sink, but this is expected
        match self.autonegotiate_sink(port).await {
            Err(Error::Pd(PdError::Rejected)) => Ok(()),
            rest => rest,
        }
    }

    /// Get Rx source/sink Caps
    ///
    /// Returns (num_standard_pdos, num_epr_pdos).
    pub async fn get_rx_caps<T: pdo::RoleCommon>(
        &mut self,
        port: LocalPortId,
        register: u8,
    ) -> Result<rx_caps::RxCaps<T>, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        let mut out_spr_pdos = [T::default(); crate::registers::rx_caps::NUM_SPR_PDOS];
        let mut out_epr_pdos = [T::default(); crate::registers::rx_caps::NUM_EPR_PDOS];

        let (num_valid_spr, num_valid_epr) = inner
            .get_rx_caps(port, register, &mut out_spr_pdos, &mut out_epr_pdos)
            .await?;

        Ok(rx_caps::RxCaps {
            spr: heapless::Vec::from_iter(out_spr_pdos.into_iter().take(num_valid_spr)),
            epr: heapless::Vec::from_iter(out_epr_pdos.into_iter().take(num_valid_epr)),
        })
    }

    /// Get Rx Sink Caps
    ///
    /// Returns (num_standard_pdos, num_epr_pdos).
    pub async fn get_rx_snk_caps(&mut self, port: LocalPortId) -> Result<rx_caps::RxSnkCaps, Error<B::Error>> {
        self.get_rx_caps(port, registers::rx_caps::RX_SNK_ADDR).await
    }

    /// Get Rx source Caps
    ///
    /// Returns (num_standard_pdos, num_epr_pdos).
    pub async fn get_rx_src_caps(&mut self, port: LocalPortId) -> Result<rx_caps::RxSrcCaps, Error<B::Error>> {
        self.get_rx_caps(port, registers::rx_caps::RX_SRC_ADDR).await
    }

    /// Get Tx Identity
    pub async fn get_tx_identity(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::tx_identity::TxIdentity, Error<B::Error>> {
        self.lock_inner().await.get_tx_identity(port).await
    }

    /// Set Tx Identity
    pub async fn set_tx_identity(
        &mut self,
        port: LocalPortId,
        value: registers::tx_identity::TxIdentity,
    ) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.set_tx_identity(port, value).await
    }

    /// Modify the Tx Identity register (`0x47`).
    pub async fn modify_tx_identity(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::tx_identity::TxIdentity) -> registers::tx_identity::TxIdentity,
    ) -> Result<registers::tx_identity::TxIdentity, Error<B::Error>> {
        self.lock_inner().await.modify_tx_identity(port, f).await
    }

    /// Get the latest received SOP identity data
    pub async fn get_received_sop_identity_data(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::received_sop_identity_data::ReceivedSopIdentityData, Error<B::Error>> {
        self.lock_inner().await.get_received_sop_identity_data(port).await
    }

    /// Get the latest received SOP Prime identity data
    pub async fn get_received_sop_prime_identity_data(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::received_sop_prime_identity_data::ReceivedSopPrimeIdentityData, Error<B::Error>> {
        self.lock_inner().await.get_received_sop_prime_identity_data(port).await
    }

    /// Get DP config
    pub async fn get_dp_config(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::DpConfig, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.get_dp_config(port).await
    }

    /// Set DP config
    pub async fn set_dp_config(
        &mut self,
        port: LocalPortId,
        config: registers::field_sets::DpConfig,
    ) -> Result<(), Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.set_dp_config(port, config).await
    }

    /// Modify DP config settings
    pub async fn modify_dp_config(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::field_sets::DpConfig) -> registers::field_sets::DpConfig,
    ) -> Result<registers::field_sets::DpConfig, Error<B::Error>> {
        let mut inner = self.lock_inner().await;
        inner.modify_dp_config(port, f).await
    }

    /// Execute the [`Command::Drst`] command.
    pub async fn execute_drst(&mut self, port: LocalPortId) -> Result<ReturnValue, Error<B::Error>> {
        self.execute_command(port, Command::Drst, None, None).await
    }

    /// Execute the [`Command::HRST`] command.
    pub async fn execute_hrst(&mut self, port: LocalPortId) -> Result<ReturnValue, Error<B::Error>> {
        self.execute_command(port, Command::HRST, None, None).await
    }

    /// Get Rx discovered custom modes
    pub async fn execute_gcdm(
        &mut self,
        port: LocalPortId,
        input: gcdm::Input,
    ) -> Result<gcdm::DiscoveredModes, Error<B::Error>> {
        let mut input_data = [0u8; gcdm::INPUT_LEN];
        let mut output_data = [0u8; gcdm::OUTPUT_LEN];

        // Executing `GCdm` too soon after the discover modes interrupt can fail
        // Brief delay to work around this, value determined by trial and error
        Timer::after_millis(5).await;

        let _size = bincode::encode_into_slice(
            input,
            input_data.as_mut_slice(),
            bincode::config::standard().with_fixed_int_encoding(),
        )
        .map_err(|_| Error::Pd(PdError::Serialize))?;

        let ret: Result<(), PdError> = self
            .execute_command(
                port,
                Command::GCdm,
                Some(input_data.as_slice()),
                Some(output_data.as_mut_slice()),
            )
            .await?
            .into();
        ret?;

        let (modes, _): (gcdm::DiscoveredModes, _) =
            bincode::decode_from_slice(&output_data, bincode::config::standard().with_fixed_int_encoding())
                .map_err(|_| Error::Pd(PdError::Serialize))?;

        // Documentation says that this command doesn't have a standard return value.
        // But it actually can fail with a rejection error, however the output data is not shifted to accommodate this.
        // We have to handle this ourselves instead of relying on the standard command execution code.
        // Object positions for the discover modes command start at 1 so we can clearly distinguish between a rejection
        // and a VDO with a value that matches a return value
        if modes.alt_modes[0].position == 0 && modes.alt_modes[0].vdo != 0 {
            Err(Error::Pd(PdError::Rejected))
        } else {
            Ok(modes)
        }
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

pub struct Interrupt<'a, M: RawMutex, B: I2c> {
    controller: &'a controller::Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> Interrupt<'a, M, B> {
    /// Process interrupts
    pub async fn process_interrupt(
        &mut self,
        int: &mut impl InputPin,
    ) -> Result<[IntEventBus1; MAX_SUPPORTED_PORTS], Error<B::Error>> {
        let mut flags = [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS];

        {
            let interrupts_enabled = self.controller.interrupts_enabled();
            let mut inner = self.controller.inner.lock().await;

            // Read and publish every asserted event before starting any destructive W1C writes.
            // Note: `interrupts_enabled` and `flags` are both of size MAX_SUPPORTED_PORTS and so
            // will always have a 1:1 mapping. If `num_ports` ever returns a value larger than
            // MAX_SUPPORTED_PORTS, `port` will simply be capped at MAX_SUPPORTED_PORTS.
            for (port, (interrupt_enabled, flag)) in interrupts_enabled
                .iter()
                .zip(flags.iter_mut())
                .take(inner.num_ports())
                .enumerate()
            {
                let port_id = LocalPortId(port as u8);

                if !interrupt_enabled {
                    trace!("{:?}: Interrupt for disabled", port_id);
                    continue;
                }

                match int.is_high() {
                    Ok(true) => {
                        // Early exit if checking the last port cleared the interrupt
                        trace!("Interrupt line is high, exiting");
                        continue;
                    }
                    Err(_) => {
                        error!("Failed to read interrupt line");
                        return PdError::Failed.into();
                    }
                    _ => {}
                }

                match with_timeout(Duration::from_millis(100), inner.get_event_bus(port_id)).await {
                    Ok(res) => match res {
                        Ok(event) => {
                            *flag |= event;
                            self.controller.commit_interrupts(port, event);
                        }
                        Err(_e) => {
                            error!("{:?}: get_event_bus failed", port_id);
                            continue;
                        }
                    },
                    Err(_) => {
                        error!("{:?}: get_event_bus timeout", port_id);
                        continue;
                    }
                }
            }

            // Pending W1C bits remain recorded until the write completes. Retrying an already
            // completed W1C is harmless, so cancellation and ambiguous bus failures are safe.
            for (port, interrupt_enabled) in interrupts_enabled.iter().take(inner.num_ports()).enumerate() {
                if !interrupt_enabled {
                    continue;
                }

                let pending = self.controller.pending_interrupt_clear(port);
                if pending == IntEventBus1::new_zero() {
                    continue;
                }

                let port_id = LocalPortId(port as u8);
                match with_timeout(Duration::from_millis(100), inner.clear_interrupt(port_id, pending)).await {
                    Ok(Ok(())) => self.controller.complete_interrupt_clear(port, pending),
                    Ok(Err(_e)) => {
                        error!("{:?}: clear_interrupt failed", port_id);
                    }
                    Err(_) => {
                        error!("{:?}: clear_interrupt timeout", port_id);
                    }
                }
            }
        }

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

/// Struct to ensure drop-safety of [`Tps6699x::wait_interrupt_any`]
///
/// This struct re-signals any unhandled interrupts on drop.
struct AccumulatedFlagsAny<'a, M: RawMutex, B: I2c> {
    controller: &'a controller::Controller<M, B>,
    accumulated_flags: [IntEventBus1; MAX_SUPPORTED_PORTS],
    masks: [IntEventBus1; MAX_SUPPORTED_PORTS],
}

impl<'a, M: RawMutex, B: I2c> AccumulatedFlagsAny<'a, M, B> {
    fn new(controller: &'a controller::Controller<M, B>, masks: [IntEventBus1; MAX_SUPPORTED_PORTS]) -> Self {
        AccumulatedFlagsAny {
            controller,
            accumulated_flags: [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS],
            masks,
        }
    }

    fn accumulate(
        &mut self,
        flags: [IntEventBus1; MAX_SUPPORTED_PORTS],
    ) -> Option<[IntEventBus1; MAX_SUPPORTED_PORTS]> {
        let mut done = false;
        for (&flags, &mask, accumulated) in izip!(flags.iter(), self.masks.iter(), self.accumulated_flags.iter_mut(),) {
            *accumulated |= flags;
            let consumed_flags = flags & mask;
            if consumed_flags != IntEventBus1::new_zero() {
                done = true;
            }
        }

        if done {
            // Panic safety: the return type, `accumulated_flags`, and `mask` are all of size MAX_SUPPORTED_PORTS
            // so this will never index out of bounds
            #[allow(clippy::indexing_slicing)]
            let handled = from_fn(|i| self.accumulated_flags[i] & self.masks[i]);
            // Put unhandled flags back for signaling in `drop()`
            self.accumulated_flags = from_fn(|i| self.accumulated_flags[i] & !self.masks[i]);
            Some(handled)
        } else {
            None
        }
    }
}

impl<M: RawMutex, B: I2c> Drop for AccumulatedFlagsAny<'_, M, B> {
    fn drop(&mut self) {
        // Catch any flags that may have happened since the last accumulate.
        let new = self
            .controller
            .interrupt_waker
            .try_take()
            .unwrap_or([IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS]);
        // Panic safety: `unhandled`, `accumulated_flags`, and `mask` are all of size MAX_SUPPORTED_PORTS
        // so this will never index out of bounds
        #[allow(clippy::indexing_slicing)]
        let unhandled = from_fn(|i| self.accumulated_flags[i] | new[i]);

        // Put back any unhandled interrupt flags for future processing
        if unhandled.iter().any(|&f| f != IntEventBus1::new_zero()) {
            // If there are unhandled flags, signal them for future processing
            trace!("Signaling unhandled interrupt flags: {:?}", unhandled);
            self.controller.interrupt_waker.signal(unhandled);
        }
    }
}

#[cfg(test)]
mod test {
    use core::convert::Infallible;
    use core::future::pending;
    use core::time::Duration as CoreDuration;

    use embassy_sync::blocking_mutex::raw::NoopRawMutex;
    use embassy_time::{with_timeout, Duration, TimeoutError};
    use embedded_hal::digital::ErrorType as DigitalErrorType;
    use embedded_hal::i2c::{ErrorKind, ErrorType, Operation};
    use embedded_hal_async::i2c::I2c;
    use embedded_hal_mock::eh1::i2c::Mock;
    use static_cell::StaticCell;

    extern crate std;
    use std::sync::{Arc, Mutex as StdMutex};

    use super::*;
    use crate::asynchronous::embassy::controller::Controller;
    use crate::asynchronous::fw_update::UpdateTarget;
    use crate::command::{TfuqBlockStatus, TFUQ_RETURN_LEN};
    use crate::registers::{REG_DATA1, REG_DATA1_LEN};
    use crate::test::{create_register_read, create_register_write, Delay, PORT0_ADDR0};
    use crate::{ADDR0, PORT0};

    struct TestPin {
        high: bool,
    }

    impl DigitalErrorType for TestPin {
        type Error = Infallible;
    }

    impl InputPin for TestPin {
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(self.high)
        }

        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(!self.high)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum BusFault {
        None,
        EventReadPending(usize),
        EventReadError(usize),
        ClearPending(usize),
        ClearError(usize),
    }

    struct TestBusState {
        event: IntEventBus1,
        event_reads: usize,
        clear_submissions: usize,
        fault: BusFault,
    }

    #[derive(Clone)]
    struct TestBus {
        state: Arc<StdMutex<TestBusState>>,
    }

    type TestController = Controller<NoopRawMutex, TestBus>;

    impl TestBus {
        fn new(event: IntEventBus1, fault: BusFault) -> (Self, Arc<StdMutex<TestBusState>>) {
            let state = Arc::new(StdMutex::new(TestBusState {
                event,
                event_reads: 0,
                clear_submissions: 0,
                fault,
            }));
            (
                Self {
                    state: Arc::clone(&state),
                },
                state,
            )
        }
    }

    impl ErrorType for TestBus {
        type Error = ErrorKind;
    }

    impl I2c for TestBus {
        async fn transaction(&mut self, _address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
            let register = match operations.first() {
                Some(Operation::Write(data)) => data.first().copied(),
                _ => None,
            };

            if register == Some(0x14) {
                let (event, action) = {
                    let mut state = self.state.lock().unwrap();
                    state.event_reads += 1;
                    let action = match state.fault {
                        BusFault::EventReadPending(call) if call == state.event_reads => 1,
                        BusFault::EventReadError(call) if call == state.event_reads => 2,
                        _ => 0,
                    };
                    (state.event, action)
                };

                if action == 1 {
                    return pending().await;
                }
                if action == 2 {
                    return Err(ErrorKind::Other);
                }

                let response = operations.get_mut(1);
                if let Some(Operation::Read(data)) = response {
                    let event_bytes: [u8; 11] = event.into();
                    if let Some(length) = data.first_mut() {
                        *length = event_bytes.len() as u8;
                    }
                    if let Some(payload) = data.get_mut(1..) {
                        payload.copy_from_slice(&event_bytes);
                    }
                    return Ok(());
                }
            }

            if register == Some(0x18) {
                let action = {
                    let mut state = self.state.lock().unwrap();
                    state.clear_submissions += 1;
                    match state.fault {
                        BusFault::ClearPending(call) if call == state.clear_submissions => 1,
                        BusFault::ClearError(call) if call == state.clear_submissions => 2,
                        _ => 0,
                    }
                };

                if action == 1 {
                    return pending().await;
                }
                if action == 2 {
                    return Err(ErrorKind::Other);
                }
                return Ok(());
            }

            Err(ErrorKind::Other)
        }
    }

    fn command_complete_event() -> IntEventBus1 {
        let mut event = IntEventBus1::new_zero();
        event.set_cmd_1_completed(true);
        event
    }

    fn pending_clear<M: RawMutex, B: I2c>(controller: &Controller<M, B>, port: usize) -> IntEventBus1 {
        controller.pending_interrupt_clear(port)
    }

    #[tokio::test]
    async fn test_event_read_cancellation_commits_nothing() {
        let event = command_complete_event();
        let (bus, state) = TestBus::new(event, BusFault::EventReadPending(1));
        let mut controller: TestController = Controller::new_tps66993(bus, ADDR0[0]).unwrap();
        let (_pd, mut interrupt) = controller.make_parts();
        let mut pin = TestPin { high: false };

        assert!(
            tokio::time::timeout(CoreDuration::from_millis(10), interrupt.process_interrupt(&mut pin))
                .await
                .is_err()
        );
        assert_eq!(pending_clear(&controller, 0), IntEventBus1::new_zero());
        assert_eq!(controller.interrupt_waker.try_take(), None);
        assert_eq!(state.lock().unwrap().clear_submissions, 0);
    }

    #[tokio::test]
    async fn test_event_read_failure_commits_nothing() {
        let event = command_complete_event();
        let (bus, state) = TestBus::new(event, BusFault::EventReadError(1));
        let mut controller: TestController = Controller::new_tps66993(bus, ADDR0[0]).unwrap();
        let (_pd, mut interrupt) = controller.make_parts();
        let mut pin = TestPin { high: false };

        interrupt.process_interrupt(&mut pin).await.unwrap();
        assert_eq!(pending_clear(&controller, 0), IntEventBus1::new_zero());
        assert_eq!(controller.interrupt_waker.try_take(), None);
        assert_eq!(state.lock().unwrap().clear_submissions, 0);
    }

    #[tokio::test]
    async fn test_cancellation_before_w1c_submission_keeps_event_pending() {
        let event = command_complete_event();
        let (bus, state) = TestBus::new(event, BusFault::EventReadPending(2));
        let mut controller: TestController = Controller::new_tps66994(bus, ADDR0).unwrap();
        let (_pd, mut interrupt) = controller.make_parts();
        let mut pin = TestPin { high: false };

        assert!(
            tokio::time::timeout(CoreDuration::from_millis(10), interrupt.process_interrupt(&mut pin))
                .await
                .is_err()
        );
        assert_eq!(pending_clear(interrupt.controller, 0), event);
        assert_eq!(
            controller.interrupt_waker.try_take(),
            Some([event, IntEventBus1::new_zero()])
        );
        assert_eq!(state.lock().unwrap().clear_submissions, 0);
    }

    #[tokio::test]
    async fn test_cancellation_after_w1c_submission_retries_pending_clear() {
        let event = command_complete_event();
        let (bus, state) = TestBus::new(event, BusFault::ClearPending(1));
        let mut controller: TestController = Controller::new_tps66993(bus, ADDR0[0]).unwrap();
        let (_pd, mut interrupt) = controller.make_parts();
        let mut low_pin = TestPin { high: false };

        assert!(
            tokio::time::timeout(CoreDuration::from_millis(10), interrupt.process_interrupt(&mut low_pin))
                .await
                .is_err()
        );
        assert_eq!(pending_clear(interrupt.controller, 0), event);
        assert_eq!(state.lock().unwrap().clear_submissions, 1);

        state.lock().unwrap().fault = BusFault::None;
        let mut high_pin = TestPin { high: true };
        interrupt.process_interrupt(&mut high_pin).await.unwrap();
        assert_eq!(pending_clear(&controller, 0), IntEventBus1::new_zero());
        assert_eq!(state.lock().unwrap().clear_submissions, 2);
        assert_eq!(
            controller.interrupt_waker.try_take(),
            Some([event, IntEventBus1::new_zero()])
        );
    }

    #[tokio::test]
    async fn test_clear_failure_keeps_event_pending_for_retry() {
        let event = command_complete_event();
        let (bus, state) = TestBus::new(event, BusFault::ClearError(1));
        let mut controller: TestController = Controller::new_tps66993(bus, ADDR0[0]).unwrap();
        let (_pd, mut interrupt) = controller.make_parts();
        let mut low_pin = TestPin { high: false };

        interrupt.process_interrupt(&mut low_pin).await.unwrap();
        assert_eq!(pending_clear(interrupt.controller, 0), event);

        state.lock().unwrap().fault = BusFault::None;
        let mut high_pin = TestPin { high: true };
        interrupt.process_interrupt(&mut high_pin).await.unwrap();
        assert_eq!(pending_clear(&controller, 0), IntEventBus1::new_zero());
        assert_eq!(state.lock().unwrap().clear_submissions, 2);
    }

    #[tokio::test]
    async fn test_clear_completion_retires_only_w1c_responsibility() {
        let event = command_complete_event();
        let (bus, state) = TestBus::new(event, BusFault::None);
        let mut controller: TestController = Controller::new_tps66993(bus, ADDR0[0]).unwrap();
        let (_pd, mut interrupt) = controller.make_parts();
        let mut pin = TestPin { high: false };

        assert_eq!(
            interrupt.process_interrupt(&mut pin).await.unwrap(),
            [event, IntEventBus1::new_zero()]
        );
        assert_eq!(pending_clear(&controller, 0), IntEventBus1::new_zero());
        assert_eq!(state.lock().unwrap().clear_submissions, 1);
        assert_eq!(
            controller.interrupt_waker.try_take(),
            Some([event, IntEventBus1::new_zero()])
        );
    }

    #[tokio::test]
    async fn test_tfuq_recovers_output_after_lost_completion_notification() {
        let mut command_data = [0u8; REG_DATA1_LEN];
        command_data[0] = ReturnValue::Success as u8;
        let output = command_data.get_mut(1..=TFUQ_RETURN_LEN).unwrap();
        output[7] = TfuqBlockStatus::DataValidAndAuthentic as u8;

        let transactions = [
            create_register_write(PORT0_ADDR0, REG_DATA1, [0x01, 0x00]),
            create_register_write(PORT0_ADDR0, 0x08, (Command::Tfuq as u32).to_le_bytes()),
            create_register_read(PORT0_ADDR0, 0x08, (Command::Success as u32).to_le_bytes()),
            create_register_read(PORT0_ADDR0, REG_DATA1, command_data),
        ];
        let mut controller: Controller<NoopRawMutex, Mock> =
            Controller::new_tps66994(Mock::new(&transactions), ADDR0).unwrap();
        let (mut pd, _interrupt) = controller.make_parts();
        let mut delay = Delay {};

        assert_eq!(
            pd.fw_update_validate_stream(&mut delay, 0).await.unwrap(),
            TfuqBlockStatus::DataValidAndAuthentic
        );
        pd.lock_inner().await.bus.done();
    }

    #[tokio::test]
    async fn test_timeout_fallback_preserves_rejected_semantics() {
        let mut command_data = [0u8; REG_DATA1_LEN];
        command_data[0] = ReturnValue::Rejected as u8;
        let transactions = [
            create_register_write(PORT0_ADDR0, 0x08, (Command::Drst as u32).to_le_bytes()),
            create_register_read(PORT0_ADDR0, 0x08, (Command::Success as u32).to_le_bytes()),
            create_register_read(PORT0_ADDR0, REG_DATA1, command_data),
        ];
        let mut controller: Controller<NoopRawMutex, Mock> =
            Controller::new_tps66994(Mock::new(&transactions), ADDR0).unwrap();
        let (mut pd, _interrupt) = controller.make_parts();

        assert_eq!(
            pd.execute_command(PORT0, Command::Drst, None, None).await,
            Err(Error::Pd(PdError::Rejected))
        );
        pd.lock_inner().await.bus.done();
    }

    #[tokio::test]
    async fn test_timeout_fallback_keeps_incomplete_result_as_busy() {
        let transactions = [
            create_register_write(PORT0_ADDR0, 0x08, (Command::Drst as u32).to_le_bytes()),
            create_register_read(PORT0_ADDR0, 0x08, (Command::Drst as u32).to_le_bytes()),
        ];
        let mut controller: Controller<NoopRawMutex, Mock> =
            Controller::new_tps66994(Mock::new(&transactions), ADDR0).unwrap();
        let (mut pd, _interrupt) = controller.make_parts();

        assert_eq!(
            pd.execute_command(PORT0, Command::Drst, None, None).await,
            Err(Error::Pd(PdError::Busy))
        );
        pd.lock_inner().await.bus.done();
    }

    #[tokio::test]
    async fn test_timeout_fallback_rejects_malformed_result_without_overwriting_output() {
        let mut command_data = [0u8; REG_DATA1_LEN];
        command_data[0] = 0x02;
        let transactions = [
            create_register_write(PORT0_ADDR0, 0x08, (Command::Drst as u32).to_le_bytes()),
            create_register_read(PORT0_ADDR0, 0x08, (Command::Success as u32).to_le_bytes()),
            create_register_read(PORT0_ADDR0, REG_DATA1, command_data),
        ];
        let mut controller: Controller<NoopRawMutex, Mock> =
            Controller::new_tps66994(Mock::new(&transactions), ADDR0).unwrap();
        let (mut pd, _interrupt) = controller.make_parts();
        let mut output = [0xA5; 4];

        assert_eq!(
            pd.execute_command(PORT0, Command::Drst, None, Some(&mut output)).await,
            Err(Error::Pd(PdError::InvalidParams))
        );
        assert_eq!(output, [0xA5; 4]);
        pd.lock_inner().await.bus.done();
    }

    /// Tests `wait_interrupt_any` with a mask for both ports.
    #[tokio::test]
    async fn test_wait_interrupt_any_both() {
        static CONTROLLER: StaticCell<controller::Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(controller::Controller::new_tps66994(Mock::new(&[]), ADDR0).unwrap());
        let (mut pd, _interrupt) = controller.make_parts();

        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);
        port0.set_sink_ready(true);
        port0.set_cmd_1_completed(true);

        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);
        port1.set_alert_message_received(true);

        pd.controller.interrupt_waker.signal([port0, port1]);

        let mut mask0 = IntEventBus1::new_zero();
        mask0.set_cmd_1_completed(true);

        let mut mask1 = IntEventBus1::new_zero();
        mask1.set_plug_event(true);
        mask1.set_alert_message_received(true);

        let flags = pd.wait_interrupt_any(false, [mask0, mask1]).await;
        assert_eq!(flags, [mask0, mask1]);

        let mut unhandled0 = IntEventBus1::new_zero();
        unhandled0.set_new_consumer_contract(true);
        unhandled0.set_sink_ready(true);

        let unhandled1 = IntEventBus1::new_zero();

        // Should already be signaled
        assert_eq!(
            pd.controller.interrupt_waker.try_take().unwrap(),
            [unhandled0, unhandled1]
        );
    }

    /// Tests `wait_interrupt` with a mask for a single port.
    #[tokio::test]
    async fn test_wait_interrupt_any_single() {
        static CONTROLLER: StaticCell<controller::Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(controller::Controller::new_tps66994(Mock::new(&[]), ADDR0).unwrap());
        let (mut pd, _interrupt) = controller.make_parts();

        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);
        port0.set_sink_ready(true);
        port0.set_cmd_1_completed(true);

        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);
        port1.set_alert_message_received(true);

        pd.controller.interrupt_waker.signal([port0, port1]);

        let mut mask0 = IntEventBus1::new_zero();
        mask0.set_cmd_1_completed(true);

        let mask1 = IntEventBus1::new_zero();

        let flags = pd.wait_interrupt_any(false, [mask0, mask1]).await;
        assert_eq!(flags, [mask0, mask1]);

        let mut unhandled0 = IntEventBus1::new_zero();
        unhandled0.set_new_consumer_contract(true);
        unhandled0.set_sink_ready(true);

        let unhandled1 = port1;

        // Should already be signaled
        assert_eq!(
            pd.controller.interrupt_waker.try_take().unwrap(),
            [unhandled0, unhandled1]
        );
    }

    /// Tests `wait_interrupt` with both masks set to zero.
    #[tokio::test]
    async fn test_wait_interrupt_any_zero_masks() {
        static CONTROLLER: StaticCell<controller::Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(controller::Controller::new_tps66994(Mock::new(&[]), ADDR0).unwrap());
        let (mut pd, _interrupt) = controller.make_parts();

        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);
        port0.set_sink_ready(true);
        port0.set_cmd_1_completed(true);

        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);
        port1.set_alert_message_received(true);

        pd.controller.interrupt_waker.signal([port0, port1]);

        let mask0 = IntEventBus1::new_zero();
        let mask1 = IntEventBus1::new_zero();
        let flags = pd.wait_interrupt_any(false, [mask0, mask1]).await;
        assert_eq!(flags, [mask0, mask1]);

        // Should already be signaled with nothing changed
        assert_eq!(pd.controller.interrupt_waker.try_take().unwrap(), [port0, port1]);
    }

    #[tokio::test]
    async fn test_wait_interrupt_any_timeout() {
        // Port0 mocked pending interrupts
        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);

        // Port1 mocked pending interrupts
        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);

        static CONTROLLER: StaticCell<Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(Controller::new_tps66994(Mock::new(&[]), ADDR0).unwrap());
        let (mut pd, _interrupt) = controller.make_parts();

        pd.controller.interrupt_waker.signal([port0, port1]);

        // The mask doesn't match the pending interrupts, so we should get a timeout
        let mut mask0 = IntEventBus1::new_zero();
        mask0.set_cmd_1_completed(true);

        let mut mask1 = IntEventBus1::new_zero();
        mask1.set_new_provider_contract(true);

        assert_eq!(
            with_timeout(Duration::from_millis(10), pd.wait_interrupt_any(false, [mask0, mask1])).await,
            Err(TimeoutError)
        );

        // Use all mask to get leftover interrupts
        let mut leftover0 = IntEventBus1::new_zero();
        leftover0.set_new_consumer_contract(true);

        let mut leftover1 = IntEventBus1::new_zero();
        leftover1.set_plug_event(true);

        let leftover_flags = with_timeout(
            Duration::from_millis(10),
            pd.wait_interrupt_any(false, [IntEventBus1::all(), IntEventBus1::all()]),
        )
        .await
        .unwrap();
        assert_eq!(leftover_flags[0], leftover0);
        assert_eq!(leftover_flags[1], leftover1);
    }
}
