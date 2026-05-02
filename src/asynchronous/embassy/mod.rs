//! This module contains a high-level API uses embassy synchronization types
use core::future::Future;
use core::iter::zip;
use core::sync::atomic::AtomicBool;

use bincode::config;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::{Mutex, MutexGuard};
use embassy_sync::signal::Signal;
use embassy_time::{with_timeout, Timer};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::ado::{self, Ado};
use embedded_usb_pd::pdinfo::AltMode;
use embedded_usb_pd::{pdo, Error, LocalPortId, PdError};

use crate::asynchronous::embassy::interrupt::InterruptReceiver;
use crate::asynchronous::internal;
use crate::asynchronous::interrupt::InterruptController;
use crate::command::{gcdm, muxr, trig, vdms, Command, ReturnValue, SrdySwitch};
use crate::registers::autonegotiate_sink::AutoComputeSinkMaxVoltage;
use crate::registers::field_sets::IntEventBus1;
use crate::{error, registers, trace, DeviceError, Mode, MAX_SUPPORTED_PORTS};

pub mod fw_update;
pub mod interrupt;
pub mod rx_caps;
pub mod task;
pub mod ucsi;

pub mod controller {
    use super::*;
    use crate::asynchronous::embassy::interrupt::InterruptProcessor;
    use crate::{TPS66993_NUM_PORTS, TPS66994_NUM_PORTS};

    /// Configuration for [`Controller`]
    #[derive(Default)]
    #[non_exhaustive]
    pub struct Config {
        pub interrupt_processor_config: crate::asynchronous::embassy::interrupt::Config,
    }

    /// Controller struct. This struct is meant to be created and then immediately broken into its parts
    pub struct Controller<M: RawMutex, B: I2c> {
        /// Config
        pub(super) config: Config,
        /// Low-level TPS6699x driver
        pub(super) inner: Mutex<M, internal::Tps6699x<B>>,
        /// Command completion signals
        pub(super) command_complete: [Signal<M, ()>; MAX_SUPPORTED_PORTS],
        /// Signal for awaiting an interrupt
        pub(super) interrupt_waker: Signal<M, [IntEventBus1; MAX_SUPPORTED_PORTS]>,
        /// Current interrupt state
        pub(super) interrupts_enabled: [AtomicBool; MAX_SUPPORTED_PORTS],
        /// Number of active ports
        pub(super) num_ports: usize,
    }

    impl<M: RawMutex, B: I2c> Controller<M, B> {
        /// Private constructor
        fn new(
            bus: B,
            config: Config,
            addr: [u8; MAX_SUPPORTED_PORTS],
            num_ports: usize,
        ) -> Result<Self, Error<B::Error>> {
            Ok(Self {
                config,
                inner: Mutex::new(internal::Tps6699x::new(bus, addr, num_ports)),
                interrupt_waker: Signal::new(),
                command_complete: [const { Signal::new() }; MAX_SUPPORTED_PORTS],
                interrupts_enabled: [const { AtomicBool::new(true) }; MAX_SUPPORTED_PORTS],
                num_ports,
            })
        }

        /// Create a new controller for the TPS66993
        pub fn new_tps66993(bus: B, config: Config, addr: u8) -> Result<Self, Error<B::Error>> {
            Self::new(bus, config, [addr, 0], TPS66993_NUM_PORTS)
        }

        /// Create a new controller for the TPS66994
        pub fn new_tps66994(bus: B, config: Config, addr: [u8; TPS66994_NUM_PORTS]) -> Result<Self, Error<B::Error>> {
            Self::new(bus, config, addr, TPS66994_NUM_PORTS)
        }

        /// Breaks the controller into its parts
        pub fn make_parts(
            &mut self,
        ) -> (
            Tps6699x<'_, M, B>,
            InterruptProcessor<'_, M, B>,
            InterruptReceiver<'_, M, B>,
        ) {
            let tps = Tps6699x { controller: self };
            let interrupt = InterruptProcessor { controller: self };
            let receiver = InterruptReceiver { controller: self };
            (tps, interrupt, receiver)
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

    /// Execute the given command with no timeout
    async fn execute_command_no_timeout(
        &mut self,
        port: LocalPortId,
        cmd: Command,
        indata: Option<&[u8]>,
        outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        // Size of the command_complete array is MAX_SUPPORTED_PORTS so the `get`` call below doesn't guarentee
        // that the port is valid because it wouldn't catch trying to access a second port on a controller with
        // only one port.
        if port.0 as usize >= self.controller.num_ports {
            return Err(Error::Pd(PdError::InvalidPort));
        }

        let command_complete = self
            .controller
            .command_complete
            .get(port.0 as usize)
            .ok_or(Error::Pd(PdError::InvalidPort))?;
        command_complete.reset();
        {
            let mut inner = self.lock_inner().await;
            inner.send_command(port, cmd, indata).await?;
        }

        command_complete.wait().await;
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
        outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        let timeout = cmd.timeout();
        let result = with_timeout(timeout, self.execute_command_no_timeout(port, cmd, indata, outdata)).await;
        if let Ok(result) = result {
            result
        } else {
            error!("Command {:#?} timed out", cmd);
            // See if there's a definite error we can read
            let mut inner = self.lock_inner().await;
            match inner.read_command_result(port, None, cmd.has_return_value()).await? {
                ReturnValue::Rejected => PdError::Rejected,
                _ => PdError::Timeout,
            }
            .into()
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

impl<'a, M: RawMutex, B: I2c> InterruptController for Tps6699x<'a, M, B> {
    type Guard = interrupt::InterruptGuard<'a, M, B>;
    type BusError = B::Error;

    async fn interrupts_enabled(&self) -> Result<[bool; MAX_SUPPORTED_PORTS], Error<Self::BusError>> {
        Ok(self.controller.interrupts_enabled())
    }

    async fn enable_interrupts_guarded(
        &mut self,
        enabled: [bool; MAX_SUPPORTED_PORTS],
    ) -> Result<Self::Guard, Error<Self::BusError>> {
        Ok(interrupt::InterruptGuard::new(self.controller, enabled))
    }
}
