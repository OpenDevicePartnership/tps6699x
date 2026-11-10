//! Asynchronous, low-level TPS6699x driver. This module provides a low-level interface
use device_driver::{AsyncRegisterInterface, Fieldset};
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::pdinfo::AltMode;
use embedded_usb_pd::pdo::{self, sink, source};
use embedded_usb_pd::{Error, LocalPortId, PdError};

use crate::registers::rx_caps::{EPR_PDO_START_INDEX, RxCapsError};
use crate::{
    DeviceError, MAX_SUPPORTED_PORTS, Mode, PORT0, PORT1, TPS66993_NUM_PORTS, TPS66994_NUM_PORTS, registers, warn,
};

mod command;

/// Wrapper to allow implementing device_driver traits on our I2C bus
pub struct Port<'a, B: I2c> {
    bus: &'a mut B,
    addr: u8,
}

impl<'a, B: I2c> Port<'a, B> {
    pub fn into_registers(self) -> registers::Registers<Port<'a, B>> {
        registers::Registers::new(self)
    }
}

impl<B: I2c> device_driver::RegisterInterfaceBase for Port<'_, B> {
    type Error = Error<B::Error>;

    type AddressType = u8;
}

impl<B: I2c> device_driver::AsyncRegisterInterface for Port<'_, B> {
    async fn write_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        // Sized to accommodate up to 255 bytes of data
        let mut buf = [0u8; 257];

        // Buffer length is sent as a byte
        if data.len() > 255 {
            return Err(PdError::InvalidParams.into());
        }

        buf[0] = address;
        buf[1] = data.len() as u8;
        let _ = &buf
            .get_mut(2..data.len() + 2)
            .ok_or(PdError::InvalidParams)?
            .copy_from_slice(data);

        self.bus
            .write(self.addr, buf.get(..data.len() + 2).ok_or(PdError::InvalidParams)?)
            .await
            .map_err(Error::Bus)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        // Sized to accommodate length byte + up to 255 bytes of data
        let mut buf = [0u8; 256];
        let full_len = data.len() + 1;
        let reg = [address];

        if data.is_empty() {
            return Err(PdError::InvalidParams.into());
        }

        self.bus
            .write_read(self.addr, &reg, buf.get_mut(..full_len).ok_or(PdError::InvalidParams)?)
            .await
            .map_err(Error::Bus)?;

        let len = buf[0] as usize;
        if len < data.len() {
            // Just log a warning to workaround registers that might not match the TRM documentation
            warn!(
                "Under-read from TPS6699x register {:#02x}: expected {} bytes, got {}",
                address,
                data.len(),
                len
            );
        }

        if len == 0xff || len == 0 {
            // Controller is busy and can't respond
            PdError::Busy.into()
        } else {
            data.copy_from_slice(buf.get(1..data.len() + 1).ok_or(PdError::InvalidParams)?);
            Ok(())
        }
    }
}

/// Low-level TSP6699x driver, generic over I2C bus (B)
pub struct Tps6699x<B: I2c> {
    pub(super) bus: B,
    /// I2C addresses for ports
    addr: [u8; MAX_SUPPORTED_PORTS],
    num_ports: usize,
    pending_interrupt_clears: [registers::IntEventBus1; MAX_SUPPORTED_PORTS],
}

impl<B: I2c> Tps6699x<B> {
    pub(super) fn new(bus: B, addr: [u8; MAX_SUPPORTED_PORTS], num_ports: usize) -> Self {
        Self {
            bus,
            addr,
            num_ports,
            pending_interrupt_clears: [registers::IntEventBus1::ZERO; MAX_SUPPORTED_PORTS],
        }
    }

    pub fn new_tps66993(bus: B, addr: u8) -> Self {
        Self::new(bus, [addr, 0x00], TPS66993_NUM_PORTS)
    }

    pub fn new_tps66994(bus: B, addr: [u8; 2]) -> Self {
        Self::new(bus, addr, TPS66994_NUM_PORTS)
    }

    /// Get the I2C address for a port
    fn port_addr(&self, port: LocalPortId) -> Result<u8, Error<B::Error>> {
        Ok(*self.addr.get(port.0 as usize).ok_or(PdError::InvalidPort)?)
    }

    /// Returns number of ports
    pub fn num_ports(&self) -> usize {
        self.num_ports
    }

    /// Borrows the given port, providing exclusive access to it and therefore the underlying bus object
    pub fn borrow_port(&mut self, port: LocalPortId) -> Result<Port<'_, B>, Error<B::Error>> {
        let addr = self.port_addr(port)?;
        Ok(Port {
            bus: &mut self.bus,
            addr,
        })
    }

    /// Read interrupts on a port and record the bits that still need to be cleared.
    ///
    /// Bits already pending a clear are omitted so a retry cannot publish the same
    /// hardware event twice.
    pub async fn read_interrupt(&mut self, port: LocalPortId) -> Result<registers::IntEventBus1, Error<B::Error>> {
        let p = self.borrow_port(port)?;
        let mut registers = p.into_registers();

        let flags = registers.int_event_bus_1().read_async().await?;
        let pending = self
            .pending_interrupt_clears
            .get_mut(port.0 as usize)
            .ok_or(PdError::InvalidPort)?;
        let unpublished = flags & !*pending;
        *pending |= flags;

        Ok(unpublished)
    }

    /// Clear all interrupt bits that were previously read on a port.
    ///
    /// The pending bits are retained until the W1C write completes, making a
    /// failed or cancelled clear independently retryable.
    pub async fn clear_pending_interrupts(&mut self, port: LocalPortId) -> Result<(), Error<B::Error>> {
        let flags = *self
            .pending_interrupt_clears
            .get(port.0 as usize)
            .ok_or(PdError::InvalidPort)?;
        if flags == registers::IntEventBus1::ZERO {
            return Ok(());
        }

        self.borrow_port(port)?
            .into_registers()
            .int_clear_bus_1()
            .write_async(|r| *r = flags)
            .await?;

        let pending = self
            .pending_interrupt_clears
            .get_mut(port.0 as usize)
            .ok_or(PdError::InvalidPort)?;
        *pending &= !flags;

        Ok(())
    }

    /// Whether a port has interrupt bits waiting for a W1C retry.
    pub fn has_pending_interrupt_clear(&self, port: LocalPortId) -> Result<bool, Error<B::Error>> {
        Ok(*self
            .pending_interrupt_clears
            .get(port.0 as usize)
            .ok_or(PdError::InvalidPort)?
            != registers::IntEventBus1::ZERO)
    }

    /// Modify interrupt mask
    pub async fn modify_interrupt_mask(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::IntEventBus1),
    ) -> Result<registers::IntEventBus1, Error<B::Error>> {
        let port = self.borrow_port(port)?;
        let mut registers = port.into_registers();
        let mut modified = registers::IntEventBus1::ZERO;
        registers
            .int_mask_bus_1()
            .modify_async(|r| {
                f(r);
                modified = *r;
            })
            .await?;
        Ok(modified)
    }

    /// Modify interrupt mask on all ports
    pub async fn modify_interrupt_mask_all(
        &mut self,
        f: impl Fn(&mut registers::IntEventBus1),
    ) -> Result<(), Error<B::Error>> {
        for port in 0..self.num_ports() {
            let port = LocalPortId(port as u8);
            let _ = self.modify_interrupt_mask(port, &f).await?;
        }
        Ok(())
    }

    /// Get port status
    pub async fn get_port_status(&mut self, port: LocalPortId) -> Result<registers::Status, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().status().read_async().await
    }

    /// Get active PDO contract
    pub async fn get_active_pdo_contract(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::ActivePdoContract, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .active_pdo_contract()
            .read_async()
            .await
    }

    /// Get active RDO contract
    pub async fn get_active_rdo_contract(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::ActiveRdoContract, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .active_rdo_contract()
            .read_async()
            .await
    }

    /// Get the Autonegotiate Sink register (`0x37`).
    pub async fn get_autonegotiate_sink(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::autonegotiate_sink::AutonegotiateSink, Error<B::Error>> {
        Ok(self
            .borrow_port(port)?
            .into_registers()
            .autonegotiate_sink()
            .read_async()
            .await?
            .into())
    }

    /// Set the Autonegotiate Sink register (`0x37`).
    pub async fn set_autonegotiate_sink(
        &mut self,
        port: LocalPortId,
        value: registers::autonegotiate_sink::AutonegotiateSink,
    ) -> Result<(), Error<B::Error>> {
        let value = value.try_into().map_err(|_| Error::Pd(PdError::InvalidParams))?;
        self.borrow_port(port)?
            .into_registers()
            .autonegotiate_sink()
            .write_async(|r| *r = value)
            .await
    }

    /// Modify autonegotiate sink settings
    pub async fn modify_autonegotiate_sink(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::autonegotiate_sink::AutonegotiateSink),
    ) -> Result<registers::autonegotiate_sink::AutonegotiateSink, Error<B::Error>> {
        let mut reg = self.get_autonegotiate_sink(port).await?;
        f(&mut reg);
        self.set_autonegotiate_sink(port, reg).await?;
        Ok(reg)
    }

    /// Get controller operation mode
    pub async fn get_mode(&mut self) -> Result<Mode, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        let mode = self.borrow_port(PORT0)?.into_registers().mode().read_async().await?;
        let mode = Mode::try_from(mode.mode()).map_err(Error::Pd)?;
        Ok(mode)
    }

    /// Get FW version
    pub async fn get_fw_version(&mut self) -> Result<u32, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(PORT0)?
            .into_registers()
            .version()
            .read_async()
            .await
            .map(|r| r.version())
    }

    /// Get customer use value
    pub async fn get_customer_use(&mut self) -> Result<u64, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(LocalPortId(0))?
            .into_registers()
            .customer_use()
            .read_async()
            .await
            .map(|r| r.customer_use())
    }

    /// Get power path status
    pub async fn get_power_path_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::PowerPathStatus, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(port)?
            .into_registers()
            .power_path_status()
            .read_async()
            .await
    }

    /// Get PD status
    pub async fn get_pd_status(&mut self, port: LocalPortId) -> Result<registers::PdStatus, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().pd_status().read_async().await
    }

    /// Get port control
    pub async fn get_port_control(&mut self, port: LocalPortId) -> Result<registers::PortControl, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .port_control()
            .read_async()
            .await
    }

    /// Set port control
    pub async fn set_port_control(
        &mut self,
        port: LocalPortId,
        control: registers::PortControl,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .port_control()
            .write_async(|r| *r = control)
            .await
    }

    /// Get global system config
    pub async fn get_system_config(&mut self) -> Result<registers::SystemConfig, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(PORT0)?
            .into_registers()
            .system_config()
            .read_async()
            .await
    }

    /// Set global system config
    pub async fn set_system_config(&mut self, config: registers::SystemConfig) -> Result<(), Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(PORT0)?
            .into_registers()
            .system_config()
            .write_async(|r| *r = config)
            .await
    }

    /// Enable/disable sourcing on a given port
    pub async fn enable_source(&mut self, port: LocalPortId, enable: bool) -> Result<(), Error<B::Error>> {
        let mut config = self.get_system_config().await?;

        let enable = if enable {
            registers::VbusSwConfig::Source
        } else {
            registers::VbusSwConfig::Disabled
        };
        match port {
            PORT0 => config.set_pa_pp_5_v_vbus_sw_config(enable),
            PORT1 => config.set_pb_pp_5_v_vbus_sw_config(enable),
            _ => return PdError::InvalidPort.into(),
        }

        self.set_system_config(config).await?;
        Ok(())
    }

    /// Get boot flags
    pub async fn get_boot_flags(&mut self) -> Result<registers::BootFlags, Error<B::Error>> {
        self.borrow_port(PORT0)?
            .into_registers()
            .boot_flags()
            .read_async()
            .await
    }

    /// Get DP status
    pub async fn get_dp_status(&mut self, port: LocalPortId) -> Result<registers::DpStatus, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().dp_status().read_async().await
    }

    /// Get Intel VID status
    pub async fn get_intel_vid_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::IntelVidStatus, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .intel_vid_status()
            .read_async()
            .await
    }

    /// Get USB status
    pub async fn get_usb_status(&mut self, port: LocalPortId) -> Result<registers::UsbStatus, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().usb_status().read_async().await
    }

    /// Get user VID status
    pub async fn get_user_vid_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::UserVidStatus, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .user_vid_status()
            .read_async()
            .await
    }

    /// Get complete alt-mode status
    pub async fn get_alt_mode_status(&mut self, port: LocalPortId) -> Result<AltMode, Error<B::Error>> {
        let dp_status = self.get_dp_status(port).await?;
        let usb_status = self.get_usb_status(port).await?;
        let user_vid_status = self.get_user_vid_status(port).await?;
        let intel_vid_status = self.get_intel_vid_status(port).await?;

        Ok(AltMode::new(
            user_vid_status.mode_1(),
            user_vid_status.mode_2(),
            user_vid_status.mode_3(),
            user_vid_status.mode_4(),
            dp_status.dp_mode_active(),
            intel_vid_status.tbt_mode_active(),
            usb_status.eudo_sop_sent_status() == registers::EudoSopSentStatus::SuccessfulEnterUsb,
        ))
    }

    /// Get DP config
    pub async fn get_dp_config(&mut self, port: LocalPortId) -> Result<registers::DpConfig, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().dp_config().read_async().await
    }

    /// Set DP config
    pub async fn set_dp_config(
        &mut self,
        port: LocalPortId,
        config: registers::DpConfig,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .dp_config()
            .write_async(|r| *r = config)
            .await
    }

    /// Modify DP config settings
    pub async fn modify_dp_config(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::DpConfig),
    ) -> Result<registers::DpConfig, Error<B::Error>> {
        let port = self.borrow_port(port)?;
        let mut registers = port.into_registers();
        let mut modified = registers::DpConfig::ZERO;
        registers
            .dp_config()
            .modify_async(|r| {
                f(r);
                modified = *r;
            })
            .await?;
        Ok(modified)
    }

    /// Get Tbt config
    pub async fn get_tbt_config(&mut self, port: LocalPortId) -> Result<registers::TbtConfig, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().tbt_config().read_async().await
    }

    /// Set Tbt config
    pub async fn set_tbt_config(
        &mut self,
        port: LocalPortId,
        config: registers::TbtConfig,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .tbt_config()
            .write_async(|r| *r = config)
            .await
    }

    /// Modify Tbt config settings
    pub async fn modify_tbt_config(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::TbtConfig),
    ) -> Result<registers::TbtConfig, Error<B::Error>> {
        let port = self.borrow_port(port)?;
        let mut registers = port.into_registers();
        let mut modified = registers::TbtConfig::ZERO;
        registers
            .tbt_config()
            .modify_async(|r| {
                f(r);
                modified = *r;
            })
            .await?;
        Ok(modified)
    }

    /// Set unconstrained power on a port
    pub async fn set_unconstrained_power(&mut self, port: LocalPortId, enable: bool) -> Result<(), Error<B::Error>> {
        let mut control = self.get_port_control(port).await?;
        control.set_unconstrained_power(enable);
        self.set_port_control(port, control).await
    }

    /// Get port config
    pub async fn get_port_config(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::port_config::PortConfig, Error<B::Error>> {
        Ok(self
            .borrow_port(port)?
            .into_registers()
            .port_config()
            .read_async()
            .await?
            .into())
    }

    /// Set port config
    pub async fn set_port_config(
        &mut self,
        port: LocalPortId,
        config: registers::port_config::PortConfig,
    ) -> Result<(), Error<B::Error>> {
        let config = config.try_into().map_err(|_| Error::Pd(PdError::InvalidParams))?;
        self.borrow_port(port)?
            .into_registers()
            .port_config()
            .write_async(|r| *r = config)
            .await
    }

    /// Get Sx App Config register (`0x20`).
    ///
    /// This register contains the current system power state.
    pub async fn get_sx_app_config(&mut self, port: LocalPortId) -> Result<registers::SxAppConfig, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .sx_app_config()
            .read_async()
            .await
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
        self.borrow_port(port)?
            .into_registers()
            .sx_app_config()
            .write_async(|config| config.set_sleep_state(state))
            .await
    }

    /// Get the discovered SVIDs on a port returned from `Discover SVIDs REQ` messages.
    pub async fn get_discovered_svids(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::discovered_svids::DiscoveredSvids, Error<B::Error>> {
        Ok(self
            .borrow_port(port)?
            .into_registers()
            .discovered_svids()
            .read_async()
            .await?
            .into())
    }

    /// Get Rx ADO
    pub async fn get_rx_ado(&mut self, port: LocalPortId) -> Result<registers::RxAdo, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().rx_ado().read_async().await
    }

    /// Get Rx attention Vdm
    pub async fn get_rx_attn_vdm(&mut self, port: LocalPortId) -> Result<registers::RxAttnVdm, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .rx_attn_vdm()
            .read_async()
            .await
    }

    /// Get Rx other Vdm
    pub async fn get_rx_other_vdm(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::rx_other_vdm::RxOtherVdm, Error<B::Error>> {
        Ok(self
            .borrow_port(port)?
            .into_registers()
            .rx_other_vdm()
            .read_async()
            .await?
            .into())
    }

    /// Get RX source/sink capabilities
    ///
    /// Returns (num pdos placed in out_spr_pdos , num pdos placed in out_epr_pdos)
    pub async fn get_rx_caps<T: pdo::RoleCommon>(
        &mut self,
        port: LocalPortId,
        register: u8,
        out_spr_pdos: &mut [T],
        out_epr_pdos: &mut [T],
    ) -> Result<(usize, usize), DeviceError<B::Error, RxCapsError>> {
        let mut raw = registers::RxSrcCaps::ZERO;
        self.borrow_port(port)
            .map_err(DeviceError::from)?
            .read_register(register, raw.as_slice_mut(), &registers::RxSrcCaps::METADATA)
            .await?;

        let rx_caps = registers::rx_caps::RxCaps::<T>::try_from(raw).map_err(DeviceError::Other)?;
        let num_sprs = out_spr_pdos.len().min(rx_caps.num_valid_pdos() as usize);
        for (i, pdo) in out_spr_pdos.iter_mut().enumerate().take(num_sprs) {
            // SPR PDOs start at index 0
            *pdo = *rx_caps
                .get(i)
                .ok_or(DeviceError::Error(Error::Pd(PdError::InvalidParams)))?;
        }

        let num_eprs = out_epr_pdos.len().min(rx_caps.num_valid_epr_pdos() as usize);
        for (i, pdo) in out_epr_pdos.iter_mut().enumerate().take(num_eprs) {
            *pdo = *rx_caps
                .get(EPR_PDO_START_INDEX + i)
                .ok_or(DeviceError::Error(Error::Pd(PdError::InvalidParams)))?;
        }

        Ok((num_sprs, num_eprs))
    }

    /// Get RX src capabilities
    ///
    /// Returns (num pdos placed in out_spr_pdos , num pdos placed in out_epr_pdos)
    pub async fn get_rx_src_caps(
        &mut self,
        port: LocalPortId,
        out_spr_pdos: &mut [source::Pdo],
        out_epr_pdos: &mut [source::Pdo],
    ) -> Result<(usize, usize), DeviceError<B::Error, RxCapsError>> {
        self.get_rx_caps(port, registers::rx_caps::RX_SRC_ADDR, out_spr_pdos, out_epr_pdos)
            .await
    }

    /// Get RX sink capabilities
    ///
    /// Returns (num pdos placed in out_spr_pdos , num pdos placed in out_epr_pdos)
    pub async fn get_rx_snk_caps(
        &mut self,
        port: LocalPortId,
        out_spr_pdos: &mut [sink::Pdo],
        out_epr_pdos: &mut [sink::Pdo],
    ) -> Result<(usize, usize), DeviceError<B::Error, RxCapsError>> {
        self.get_rx_caps(port, registers::rx_caps::RX_SNK_ADDR, out_spr_pdos, out_epr_pdos)
            .await
    }

    /// Get Tx Identity
    pub async fn get_tx_identity(&mut self, port: LocalPortId) -> Result<registers::TxIdentity, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .tx_identity()
            .read_async()
            .await
    }

    /// Set Tx Identity
    pub async fn set_tx_identity(
        &mut self,
        port: LocalPortId,
        value: registers::TxIdentity,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .tx_identity()
            .write_async(|r| *r = value)
            .await
    }

    /// Modify Tx Identity settings
    pub async fn modify_tx_identity(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::TxIdentity),
    ) -> Result<registers::TxIdentity, Error<B::Error>> {
        let port = self.borrow_port(port)?;
        let mut registers = port.into_registers();
        let mut modified = registers::TxIdentity::ZERO;
        registers
            .tx_identity()
            .modify_async(|r| {
                f(r);
                modified = *r;
            })
            .await?;
        Ok(modified)
    }

    /// Get the latest received SOP identity data
    pub async fn get_received_sop_identity_data(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::received_sop_identity_data::ReceivedSopIdentityData, Error<B::Error>> {
        Ok(self
            .borrow_port(port)?
            .into_registers()
            .received_sop_identity_data()
            .read_async()
            .await?
            .into())
    }

    /// Get the latest received SOP Prime identity data
    pub async fn get_received_sop_prime_identity_data(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::received_sop_prime_identity_data::ReceivedSopPrimeIdentityData, Error<B::Error>> {
        Ok(self
            .borrow_port(port)?
            .into_registers()
            .received_sop_prime_identity_data()
            .read_async()
            .await?
            .into())
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::vec::Vec;

    use device_driver::{AsyncRegisterInterface, FieldsetMetadata};
    use embedded_hal_async::i2c::ErrorType;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use embedded_usb_pd::pdo::MV50_UNIT;
    use embedded_usb_pd::pdo::source::Pdo;

    use super::*;
    use crate::registers::rx_caps::{self};
    use crate::test::*;
    use crate::{ADDR0, ADDR1, PORT0, PORT1};

    /// Test firmware version, no particular meaning to this value
    const TEST_FW_VERSION: u32 = 0x12345678;
    /// Test customer use value, no particular meaning to this value
    const TEST_CUSTOMER_USE: u64 = 0x123456789abcdef0;

    #[test]
    fn test() {
        create_register_read(0, 0, [0]);
    }

    #[tokio::test]
    async fn test_data1_generated_read_write() {
        let mut expected = [0u8; 64];
        expected[0] = 0x12;
        expected[31] = 0x34;
        expected[63] = 0x56;
        let transactions = [
            create_register_write(PORT0_ADDR0, 0x09, expected),
            create_register_read(PORT0_ADDR0, 0x09, expected),
        ];
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&transactions), ADDR0);
        let result = async {
            let mut registers = tps6699x.borrow_port(PORT0)?.into_registers();
            registers
                .data_1()
                .write_with_zero_async(|data| {
                    data.set_bytes(0, 0x12);
                    data.set_bytes(31, 0x34);
                    data.set_bytes(63, 0x56);
                })
                .await?;
            registers.data_1().read_async().await
        }
        .await;
        assert_eq!(
            result.map(|data| (data.bytes(0), data.bytes(31), data.bytes(63), <[u8; 64]>::from(data))),
            Ok((0x12, 0x34, 0x56, expected))
        );
        tps6699x.bus.done();
    }

    async fn test_read_port<const N: usize>(
        tps6699x: &mut Tps6699x<Mock>,
        port_id: LocalPortId,
        expected_addr: u8,
        reg: u8,
        expected: [u8; N],
    ) -> Result<(), Error<<Mock as ErrorType>::Error>> {
        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, reg, expected)]);

        let mut port = tps6699x.borrow_port(port_id)?;
        let mut result = [0; N];
        port.read_register(reg, &mut result, &FieldsetMetadata::DEFAULT).await?;
        tps6699x.bus.done();

        Ok(())
    }

    /// Test that attempting to read more than the available number of bytes fails
    async fn test_read_port_overread<const N: usize>(
        tps6699x: &mut Tps6699x<Mock>,
        port_id: LocalPortId,
        expected_addr: u8,
        reg: u8,
        expected: [u8; N],
    ) -> Result<(), Error<<Mock as ErrorType>::Error>> {
        // +1 for the length byte, +1 for the overread byte to make the I2C mock happy with the lengths
        let mut response = Vec::with_capacity(N + 2);
        response.push(N as u8);
        response.splice(1..1, expected.iter().cloned());
        response.push(0x00);

        tps6699x
            .bus
            .update_expectations(&[Transaction::write_read(expected_addr, std::vec![reg], response)]);

        let mut port = tps6699x.borrow_port(port_id)?;
        let mut result = std::vec![0; N + 1];
        let r = port.read_register(reg, &mut result, &FieldsetMetadata::DEFAULT).await;
        tps6699x.bus.done();
        r
    }

    /// Test that attempting to read less than the available number of bytes succeeds
    async fn test_read_port_underread<const N: usize>(
        tps6699x: &mut Tps6699x<Mock>,
        port_id: LocalPortId,
        expected_addr: u8,
        reg: u8,
        expected: [u8; N],
    ) -> Result<(), Error<<Mock as ErrorType>::Error>> {
        // +1 for the length byte, +1 for the unread byte to make the I2C mock happy with the lengths
        let mut response = Vec::with_capacity(N + 2);
        response.push((N + 1) as u8);
        response.splice(1..1, expected.iter().cloned());
        response.push(0x00);

        tps6699x
            .bus
            .update_expectations(&[Transaction::write_read(expected_addr, std::vec![reg], response)]);

        let mut port = tps6699x.borrow_port(port_id)?;
        let mut result = std::vec![0; N + 1];
        port.read_register(reg, &mut result, &FieldsetMetadata::DEFAULT).await?;
        tps6699x.bus.done();

        Ok(())
    }

    async fn test_write_port<const N: usize>(
        tps6699x: &mut Tps6699x<Mock>,
        port_id: LocalPortId,
        expected_addr: u8,
        reg: u8,
        expected: [u8; N],
    ) -> Result<(), Error<<Mock as ErrorType>::Error>> {
        tps6699x
            .bus
            .update_expectations(&[create_register_write(expected_addr, reg, expected)]);

        let mut port = tps6699x.borrow_port(port_id)?;
        let mut data = expected;
        port.write_register(reg, &mut data, &FieldsetMetadata::DEFAULT).await?;
        tps6699x.bus.done();

        Ok(())
    }

    async fn test_rw_port<const N: usize>(
        tps6699x: &mut Tps6699x<Mock>,
        port_id: LocalPortId,
        expected_addr: u8,
        reg: u8,
        expected: [u8; N],
    ) -> Result<(), Error<<Mock as ErrorType>::Error>> {
        test_read_port::<N>(tps6699x, port_id, expected_addr, reg, expected).await?;
        test_write_port::<N>(tps6699x, port_id, expected_addr, reg, expected).await?;
        test_read_port_underread::<N>(tps6699x, port_id, expected_addr, reg, expected).await?;
        // Over-reads should succeed
        test_read_port_overread(tps6699x, port_id, expected_addr, reg, expected).await
    }

    async fn test_rw_ports(tps6699x: &mut Tps6699x<Mock>, port_id: LocalPortId, expected_addr: u8) {
        // No particular signifigance to these values, just testing a mix of values
        test_rw_port(tps6699x, port_id, expected_addr, 0x00, [0x01, 0x02])
            .await
            .unwrap();
        test_rw_port(tps6699x, port_id, expected_addr, 0x20, [0x87, 0x2, 0x7])
            .await
            .unwrap();
        test_rw_port(tps6699x, port_id, expected_addr, 0x05, [0xff, 0xa7, 0x79, 0x87])
            .await
            .unwrap();
    }

    /// Test on the first set of I2C addresses
    #[tokio::test]
    async fn test_rw_ports_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        test_rw_ports(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_rw_ports(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    /// Test on the second set of I2C addresses
    #[tokio::test]
    async fn test_rw_ports_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR1);

        test_rw_ports(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_rw_ports(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn run_read_and_clear_interrupt(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::IntEventBus1;

        // Create a fully asserted interrupt register
        let int = !IntEventBus1::ZERO;
        let mut transactions = Vec::new();

        // Read the interrupt register
        transactions.push(create_register_read(expected_addr, 0x14, int));

        // Write to the interrupt clear register
        transactions.push(create_register_write(expected_addr, 0x18, int));
        tps6699x.bus.update_expectations(&transactions);

        assert_eq!(tps6699x.read_interrupt(port).await.unwrap(), int);
        assert!(tps6699x.has_pending_interrupt_clear(port).unwrap());
        tps6699x.clear_pending_interrupts(port).await.unwrap();
        assert!(!tps6699x.has_pending_interrupt_clear(port).unwrap());
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_read_and_clear_interrupt() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_read_and_clear_interrupt(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_read_and_clear_interrupt(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_port_status(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::Status;

        let mut transactions = Vec::new();
        // Read status register
        transactions.push(create_register_read(expected_addr, 0x1A, Status::ZERO));
        tps6699x.bus.update_expectations(&transactions);

        let status = tps6699x.get_port_status(port).await.unwrap();
        assert_eq!(status, Status::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_port_status() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_port_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_port_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_active_pdo_contract(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::ActivePdoContract;

        let mut transactions = Vec::new();
        // Read status register
        transactions.push(create_register_read(expected_addr, 0x34, ActivePdoContract::ZERO));
        tps6699x.bus.update_expectations(&transactions);

        let active_pdo_contract = tps6699x.get_active_pdo_contract(port).await.unwrap();
        assert_eq!(active_pdo_contract, ActivePdoContract::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_active_pdo_contract() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_active_pdo_contract(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_active_pdo_contract(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_active_rdo_contract(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::ActiveRdoContract;

        let mut transactions = Vec::new();
        // Read status register
        transactions.push(create_register_read(expected_addr, 0x35, ActiveRdoContract::ZERO));
        tps6699x.bus.update_expectations(&transactions);

        let active_rdo_contract = tps6699x.get_active_rdo_contract(port).await.unwrap();
        assert_eq!(active_rdo_contract, ActiveRdoContract::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_active_rdo_contract() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_active_rdo_contract(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_active_rdo_contract(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_mode(tps6699x: &mut Tps6699x<Mock>, expected_addr: u8, expected_mode: Mode) {
        let mut transactions = Vec::new();
        transactions.push(create_register_read(expected_addr, 0x03, expected_mode));
        tps6699x.bus.update_expectations(&transactions);

        let mode = tps6699x.get_mode().await.unwrap();
        assert_eq!(mode, expected_mode);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_modes() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);
        run_get_mode(&mut tps6699x, PORT0_ADDR0, Mode::Boot).await;
        run_get_mode(&mut tps6699x, PORT0_ADDR0, Mode::F211).await;
        run_get_mode(&mut tps6699x, PORT0_ADDR0, Mode::App0).await;
        run_get_mode(&mut tps6699x, PORT0_ADDR0, Mode::App1).await;
        run_get_mode(&mut tps6699x, PORT0_ADDR0, Mode::Wtpr).await;
    }

    #[tokio::test]
    async fn test_get_fw_version() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        let mut transactions = Vec::new();
        transactions.push(create_register_read(PORT0_ADDR0, 0x0F, TEST_FW_VERSION.to_le_bytes()));
        tps6699x.bus.update_expectations(&transactions);

        let version = tps6699x.get_fw_version().await.unwrap();
        assert_eq!(version, TEST_FW_VERSION);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_customer_use() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);
        let mut transactions = Vec::new();
        transactions.push(create_register_read(PORT0_ADDR0, 0x06, TEST_CUSTOMER_USE.to_le_bytes()));
        tps6699x.bus.update_expectations(&transactions);

        let value = tps6699x.get_customer_use().await.unwrap();
        assert_eq!(value, TEST_CUSTOMER_USE);
        tps6699x.bus.done();
    }

    /// Test reading SPR PDOs
    #[tokio::test]
    async fn test_get_rx_src_caps_spr() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        let mut buf = [0u8; rx_caps::LEN + 1];
        // Register length
        buf[0] = rx_caps::LEN as u8;
        // Set header: low 3 bits are SPR PDO count
        buf[1] = 0xa; // 2 SPR PDOs, 1 EPR PDOs
        // Fill PDOs with test data
        // SPR PDO 0 - Fixed PDO at 5V, 3A, 100% peak current
        buf[2..6].copy_from_slice(&TEST_SRC_PDO_FIXED_5V3A_RAW.to_le_bytes());
        // SPR PDO 1 - Fixed PDO at 5V, 1.5A, 100% peak current
        buf[6..10].copy_from_slice(&TEST_SRC_PDO_FIXED_5V1A5_RAW.to_le_bytes());
        // Fake SPR, used to test overread
        buf[10..14].copy_from_slice(&TEST_SRC_PDO_FIXED_5V900MA_RAW.to_le_bytes());
        // EPR PDO 0 - Fixed PDO at 28V, 5A, 100% peak current
        buf[30..34].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V5A_RAW.to_le_bytes());
        // Fake EPR, used to test overread
        buf[34..38].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V3A_RAW.to_le_bytes());
        // Fake EPR, used to test overread
        buf[38..42].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V1A5_RAW.to_le_bytes());

        // Expect the full register, including the length byte.
        tps6699x.bus.update_expectations(
            &[Transaction::write_read(PORT0_ADDR0, std::vec![rx_caps::RX_SRC_ADDR], {
                Vec::from(buf)
            })],
        );

        // Read PDOs, with attempted overread
        let mut out_spr_pdos = [Pdo::default(); 3];
        let mut out_epr_pdos = [Pdo::default(); 2];
        let (num_pdos, num_epr_pdos) = tps6699x
            .get_rx_src_caps(PORT0, &mut out_spr_pdos, &mut out_epr_pdos)
            .await
            .unwrap();
        tps6699x.bus.done();

        assert_eq!(num_pdos, 2);
        assert_eq!(num_epr_pdos, 1);
        assert_eq!(out_spr_pdos[0], TEST_SRC_PDO_FIXED_5V3A);
        assert_eq!(out_spr_pdos[1], TEST_SRC_PDO_FIXED_5V1A5);
        assert_eq!(out_spr_pdos[2], Pdo::default());

        assert_eq!(out_epr_pdos[0], TEST_SRC_EPR_PDO_FIXED_28V5A);
        assert_eq!(out_epr_pdos[1], Pdo::default());
    }

    #[tokio::test]
    async fn test_read_generated_rx_caps_registers() {
        let mut source_bytes = [0u8; rx_caps::LEN];
        source_bytes[0] = 0x61; // 1 SPR PDO, 4 EPR PDOs, last received is EPR.
        source_bytes[1..5].copy_from_slice(&TEST_SRC_PDO_FIXED_5V3A_RAW.to_le_bytes());
        source_bytes[41..45].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V5A_RAW.to_le_bytes());
        let mut sink_bytes = [0u8; rx_caps::LEN];
        sink_bytes[0] = 0x09; // 1 SPR PDO, 1 EPR PDO.
        sink_bytes[1..5].copy_from_slice(&TEST_SRC_PDO_FIXED_5V1A5_RAW.to_le_bytes());
        sink_bytes[29..33].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V3A_RAW.to_le_bytes());

        let mock = Mock::new(&[
            create_register_read(PORT1_ADDR0, 0x30, source_bytes),
            create_register_read(PORT1_ADDR0, 0x31, sink_bytes),
            create_register_read(PORT1_ADDR0, 0x31, sink_bytes),
        ]);
        let mut tps6699x = Tps6699x::new_tps66994(mock, ADDR0);
        let mut registers = tps6699x.borrow_port(PORT1).unwrap().into_registers();

        let source = registers.rx_src_caps().read_async().await.unwrap();
        let sink = registers.rx_snk_caps().read_async().await.unwrap();
        assert_eq!(<[u8; rx_caps::LEN]>::from(source), source_bytes);
        assert_eq!(<[u8; rx_caps::LEN]>::from(sink), sink_bytes);
        assert_eq!(source.epr_pdo_3(), TEST_SRC_EPR_PDO_FIXED_28V5A_RAW);

        let mut spr = [sink::Pdo::default(); 2];
        let mut epr = [sink::Pdo::default(); 2];
        let counts = tps6699x.get_rx_snk_caps(PORT1, &mut spr, &mut epr).await.unwrap();
        assert_eq!(counts, (1, 1));
        assert_eq!(
            spr[0],
            sink::Pdo::Fixed(sink::FixedData {
                voltage_mv: 5000,
                operational_current_ma: 1500,
                ..Default::default()
            })
        );
        assert_eq!(
            epr[0],
            sink::Pdo::Fixed(sink::FixedData {
                voltage_mv: 28000,
                operational_current_ma: 3000,
                ..Default::default()
            })
        );
        assert_eq!(spr[1], sink::Pdo::default());
        assert_eq!(epr[1], sink::Pdo::default());
        tps6699x.bus.done();
    }

    async fn run_modify_interrupt_mask(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::IntEventBus1;

        let initial = IntEventBus1::ZERO;
        let mut expected = initial;
        expected.set_plug_event(true);

        tps6699x.bus.update_expectations(&[
            create_register_read(expected_addr, 0x16, initial),
            create_register_write(expected_addr, 0x16, expected),
        ]);

        let result = tps6699x
            .modify_interrupt_mask(port, |r| {
                r.set_plug_event(true);
            })
            .await
            .unwrap();

        assert_eq!(result, expected);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_modify_interrupt_mask() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_modify_interrupt_mask(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_modify_interrupt_mask(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_modify_interrupt_mask_all() {
        use registers::IntEventBus1;

        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        let initial = IntEventBus1::ZERO;
        let mut expected = initial;
        expected.set_plug_event(true);

        tps6699x.bus.update_expectations(&[
            create_register_read(PORT0_ADDR0, 0x16, initial),
            create_register_write(PORT0_ADDR0, 0x16, expected),
            create_register_read(PORT1_ADDR0, 0x16, initial),
            create_register_write(PORT1_ADDR0, 0x16, expected),
        ]);

        tps6699x
            .modify_interrupt_mask_all(|r| {
                r.set_plug_event(true);
            })
            .await
            .unwrap();

        tps6699x.bus.done();
    }

    async fn run_get_autonegotiate_sink(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::autonegotiate_sink;

        let data = [0u8; autonegotiate_sink::LEN];
        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x37, data)]);

        let result = tps6699x.get_autonegotiate_sink(port).await.unwrap();
        assert_eq!(<[u8; autonegotiate_sink::LEN]>::try_from(result), Ok(data));
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_autonegotiate_sink() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_autonegotiate_sink(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_autonegotiate_sink(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_set_autonegotiate_sink(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::autonegotiate_sink::{self, AutonegotiateSink};

        let data = [0u8; autonegotiate_sink::LEN];
        tps6699x
            .bus
            .update_expectations(&[create_register_write(expected_addr, 0x37, data)]);

        tps6699x
            .set_autonegotiate_sink(port, AutonegotiateSink::from(data))
            .await
            .unwrap();
        tps6699x.bus.done();
    }

    async fn run_set_autonegotiate_sink_invalid(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId) {
        use embedded_usb_pd::pdo::{MA10_UNIT, MA50_UNIT, MV20_UNIT, MV50_UNIT, MW250_UNIT};
        use registers::autonegotiate_sink::AutonegotiateSink;

        let mut sink = AutonegotiateSink::default();
        sink.auto_neg_max_current_ma = 0x400 * MA10_UNIT;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.auto_neg_sink_min_required_power_mw = 0x400 * MW250_UNIT;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.auto_neg_max_voltage_mv = 0x400 * MV50_UNIT;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.auto_neg_min_voltage_mv = 0x400 * MV50_UNIT;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.auto_neg_capabilities_mismatch_power_mw = 0x400 * MW250_UNIT;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.pps_sink_voltage_threshold = 0x800;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.pps_operating_current_ma = 0x80 * MA50_UNIT;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.pps_output_voltage_mv = 0x800 * MV20_UNIT;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.epr_avs_operating_current = 0x80;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut sink = AutonegotiateSink::default();
        sink.epr_avs_output_voltage = 0x1000;
        assert_eq!(
            tps6699x.set_autonegotiate_sink(port, sink).await,
            Err(Error::Pd(PdError::InvalidParams))
        );
    }

    #[tokio::test]
    async fn test_set_autonegotiate_sink() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_set_autonegotiate_sink(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_set_autonegotiate_sink(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_set_autonegotiate_sink_invalid() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_set_autonegotiate_sink_invalid(&mut tps6699x, PORT0).await;
        run_set_autonegotiate_sink_invalid(&mut tps6699x, PORT1).await;
        tps6699x.bus.done();
    }

    async fn run_modify_autonegotiate_sink(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::autonegotiate_sink::{self, AutonegotiateSink};

        let initial_bytes = [0u8; autonegotiate_sink::LEN];
        let mut expected = AutonegotiateSink::from(initial_bytes);
        expected.no_usb_suspend = true;
        let expected_bytes: [u8; autonegotiate_sink::LEN] = expected.try_into().unwrap();

        tps6699x.bus.update_expectations(&[
            create_register_read(expected_addr, 0x37, initial_bytes),
            create_register_write(expected_addr, 0x37, expected_bytes),
        ]);

        let result = tps6699x
            .modify_autonegotiate_sink(port, |r| {
                r.no_usb_suspend = true;
            })
            .await
            .unwrap();

        assert_eq!(result, expected);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_modify_autonegotiate_sink() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_modify_autonegotiate_sink(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_modify_autonegotiate_sink(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_power_path_status(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::PowerPathStatus;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x26, PowerPathStatus::ZERO)]);

        let result = tps6699x.get_power_path_status(port).await.unwrap();
        assert_eq!(result, PowerPathStatus::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_power_path_status() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_power_path_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_power_path_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_pd_status(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::PdStatus;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x40, PdStatus::ZERO)]);

        let result = tps6699x.get_pd_status(port).await.unwrap();
        assert_eq!(result, PdStatus::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_pd_status() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_pd_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_pd_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_port_control(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::PortControl;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x29, PortControl::ZERO)]);

        let result = tps6699x.get_port_control(port).await.unwrap();
        assert_eq!(result, PortControl::ZERO);
        tps6699x.bus.done();
    }

    async fn run_set_port_control(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::PortControl;

        tps6699x
            .bus
            .update_expectations(&[create_register_write(expected_addr, 0x29, PortControl::ZERO)]);

        tps6699x.set_port_control(port, PortControl::ZERO).await.unwrap();
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_port_control() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_port_control(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_port_control(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_set_port_control() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_set_port_control(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_set_port_control(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_get_system_config() {
        use registers::SystemConfig;
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        tps6699x
            .bus
            .update_expectations(&[create_register_read(PORT0_ADDR0, 0x27, SystemConfig::ZERO)]);

        let result = tps6699x.get_system_config().await.unwrap();
        assert_eq!(result, SystemConfig::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_set_system_config() {
        use registers::SystemConfig;
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        tps6699x
            .bus
            .update_expectations(&[create_register_write(PORT0_ADDR0, 0x27, SystemConfig::ZERO)]);

        tps6699x.set_system_config(SystemConfig::ZERO).await.unwrap();
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_boot_flags() {
        use registers::BootFlags;

        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        tps6699x
            .bus
            .update_expectations(&[create_register_read(PORT0_ADDR0, 0x2D, BootFlags::ZERO)]);

        let result = tps6699x.get_boot_flags().await.unwrap();
        assert_eq!(result, BootFlags::ZERO);
        tps6699x.bus.done();
    }

    async fn run_get_dp_status(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::DpStatus;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x58, DpStatus::ZERO)]);

        let result = tps6699x.get_dp_status(port).await.unwrap();
        assert_eq!(result, DpStatus::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_dp_status() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_dp_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_dp_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_intel_vid_status(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::IntelVidStatus;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x59, IntelVidStatus::ZERO)]);

        let result = tps6699x.get_intel_vid_status(port).await.unwrap();
        assert_eq!(result, IntelVidStatus::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_intel_vid_status() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_intel_vid_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_intel_vid_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_usb_status(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::UsbStatus;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x24, UsbStatus::ZERO)]);

        let result = tps6699x.get_usb_status(port).await.unwrap();
        assert_eq!(result, UsbStatus::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_usb_status() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_usb_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_usb_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_user_vid_status(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::UserVidStatus;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x57, UserVidStatus::ZERO)]);

        let result = tps6699x.get_user_vid_status(port).await.unwrap();
        assert_eq!(result, UserVidStatus::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_user_vid_status() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_user_vid_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_user_vid_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_dp_config(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::DpConfig;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x51, DpConfig::ZERO)]);

        let result = tps6699x.get_dp_config(port).await.unwrap();
        assert_eq!(result, DpConfig::ZERO);
        tps6699x.bus.done();
    }

    async fn run_set_dp_config(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::DpConfig;

        tps6699x
            .bus
            .update_expectations(&[create_register_write(expected_addr, 0x51, DpConfig::ZERO)]);

        tps6699x.set_dp_config(port, DpConfig::ZERO).await.unwrap();
        tps6699x.bus.done();
    }

    async fn run_modify_dp_config(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::DpConfig;

        let initial = DpConfig::ZERO;
        let mut expected = initial;
        expected.set_enable_dp_svid(true);

        tps6699x.bus.update_expectations(&[
            create_register_read(expected_addr, 0x51, initial),
            create_register_write(expected_addr, 0x51, expected),
        ]);

        let result = tps6699x
            .modify_dp_config(port, |r| {
                r.set_enable_dp_svid(true);
            })
            .await
            .unwrap();

        assert_eq!(result, expected);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_dp_config() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_dp_config(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_dp_config(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_set_dp_config() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_set_dp_config(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_set_dp_config(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_modify_dp_config() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_modify_dp_config(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_modify_dp_config(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_tbt_config(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::TbtConfig;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x52, TbtConfig::ZERO)]);

        let result = tps6699x.get_tbt_config(port).await.unwrap();
        assert_eq!(result, TbtConfig::ZERO);
        tps6699x.bus.done();
    }

    async fn run_set_tbt_config(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::TbtConfig;

        tps6699x
            .bus
            .update_expectations(&[create_register_write(expected_addr, 0x52, TbtConfig::ZERO)]);

        tps6699x.set_tbt_config(port, TbtConfig::ZERO).await.unwrap();
        tps6699x.bus.done();
    }

    async fn run_modify_tbt_config(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::TbtConfig;

        let initial = TbtConfig::ZERO;
        let mut expected = initial;
        expected.set_tbt_vid_en(true);

        tps6699x.bus.update_expectations(&[
            create_register_read(expected_addr, 0x52, initial),
            create_register_write(expected_addr, 0x52, expected),
        ]);

        let result = tps6699x
            .modify_tbt_config(port, |r| {
                r.set_tbt_vid_en(true);
            })
            .await
            .unwrap();

        assert_eq!(result, expected);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_tbt_config() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_tbt_config(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_tbt_config(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_set_tbt_config() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_set_tbt_config(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_set_tbt_config(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_modify_tbt_config() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_modify_tbt_config(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_modify_tbt_config(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_port_config(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::port_config;

        let data = [0u8; port_config::LEN];
        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x28, data)]);

        let result = tps6699x.get_port_config(port).await.unwrap();
        assert_eq!(<[u8; port_config::LEN]>::try_from(result), Ok(data));
        tps6699x.bus.done();
    }

    /// Test logic for an invalid register value attempting to be written to hardware
    async fn run_set_port_config_invalid(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId) {
        use registers::port_config;

        let mut port_config = port_config::PortConfig::default();
        port_config.apdo_vbus_uvp_threshold = 0x4;
        assert_eq!(
            tps6699x.set_port_config(port, port_config).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut port_config = port_config::PortConfig::default();
        port_config.apdo_ilim_over_shoot = 0x4;
        assert_eq!(
            tps6699x.set_port_config(port, port_config).await,
            Err(Error::Pd(PdError::InvalidParams))
        );

        let mut port_config = port_config::PortConfig::default();
        port_config.gt_threshold_voltage_mv = (u16::MAX as u32 + 1) * MV50_UNIT as u32;
        assert_eq!(
            tps6699x.set_port_config(port, port_config).await,
            Err(Error::Pd(PdError::InvalidParams))
        );
    }

    async fn run_set_port_config(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::port_config::{self, PortConfig};

        let data = [0u8; port_config::LEN];
        tps6699x
            .bus
            .update_expectations(&[create_register_write(expected_addr, 0x28, data)]);

        tps6699x.set_port_config(port, PortConfig::from(data)).await.unwrap();
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_port_config() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_port_config(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_port_config(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_set_port_config() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_set_port_config(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_set_port_config(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_set_port_config_invalid() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_set_port_config_invalid(&mut tps6699x, PORT0).await;
        run_set_port_config_invalid(&mut tps6699x, PORT1).await;
        tps6699x.bus.done();
    }

    async fn run_get_rx_ado(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::RxAdo;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x74, RxAdo::ZERO)]);

        let result = tps6699x.get_rx_ado(port).await.unwrap();
        assert_eq!(result, RxAdo::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_rx_ado() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_rx_ado(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_rx_ado(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_rx_attn_vdm(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::RxAttnVdm;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x60, RxAttnVdm::ZERO)]);

        let result = tps6699x.get_rx_attn_vdm(port).await.unwrap();
        assert_eq!(result, RxAttnVdm::ZERO);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_rx_attn_vdm() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_rx_attn_vdm(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_rx_attn_vdm(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_rx_other_vdm(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::rx_other_vdm;

        let data = [0u8; rx_other_vdm::LEN];
        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x61, data)]);

        let result = tps6699x.get_rx_other_vdm(port).await.unwrap();
        assert_eq!(<[u8; rx_other_vdm::LEN]>::from(result), data);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_rx_other_vdm() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_rx_other_vdm(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_rx_other_vdm(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_tx_identity(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::TxIdentity;

        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x47, TxIdentity::ZERO)]);

        let result = tps6699x.get_tx_identity(port).await.unwrap();
        assert_eq!(result, TxIdentity::ZERO);
        tps6699x.bus.done();
    }

    async fn run_set_tx_identity(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::TxIdentity;

        tps6699x
            .bus
            .update_expectations(&[create_register_write(expected_addr, 0x47, TxIdentity::ZERO)]);

        tps6699x.set_tx_identity(port, TxIdentity::ZERO).await.unwrap();
        tps6699x.bus.done();
    }

    async fn run_modify_tx_identity(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::TxIdentity;

        let initial = TxIdentity::ZERO;
        let mut expected = initial;
        expected.set_modal_operation_supported(true);

        tps6699x.bus.update_expectations(&[
            create_register_read(expected_addr, 0x47, initial),
            create_register_write(expected_addr, 0x47, expected),
        ]);

        let result = tps6699x
            .modify_tx_identity(port, |r| {
                r.set_modal_operation_supported(true);
            })
            .await
            .unwrap();

        assert_eq!(result, expected);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_tx_identity() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_tx_identity(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_tx_identity(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_set_tx_identity() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_set_tx_identity(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_set_tx_identity(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_modify_tx_identity() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_modify_tx_identity(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_modify_tx_identity(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_discovered_svids(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::discovered_svids;

        let data = [0u8; discovered_svids::LEN];
        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x21, data)]);

        let result = tps6699x.get_discovered_svids(port).await.unwrap();
        assert_eq!(<[u8; discovered_svids::LEN]>::from(result), data);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_discovered_svids() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_discovered_svids(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_discovered_svids(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_received_sop_identity_data(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::received_sop_identity_data;

        let data = [0u8; received_sop_identity_data::LEN];
        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x48, data)]);

        let result = tps6699x.get_received_sop_identity_data(port).await.unwrap();
        assert_eq!(<[u8; received_sop_identity_data::LEN]>::from(result), data);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_received_sop_identity_data() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_received_sop_identity_data(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_received_sop_identity_data(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    async fn run_get_received_sop_prime_identity_data(
        tps6699x: &mut Tps6699x<Mock>,
        port: LocalPortId,
        expected_addr: u8,
    ) {
        use registers::received_sop_prime_identity_data;

        let data = [0u8; received_sop_prime_identity_data::LEN];
        tps6699x
            .bus
            .update_expectations(&[create_register_read(expected_addr, 0x49, data)]);

        let result = tps6699x.get_received_sop_prime_identity_data(port).await.unwrap();
        assert_eq!(<[u8; received_sop_prime_identity_data::LEN]>::from(result), data);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_received_sop_prime_identity_data() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        run_get_received_sop_prime_identity_data(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_get_received_sop_prime_identity_data(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }
}
