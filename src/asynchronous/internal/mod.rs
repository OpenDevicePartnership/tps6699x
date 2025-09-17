//! Asynchronous, low-level TPS6699x driver. This module provides a low-level interface
use device_driver::AsyncRegisterInterface;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::pdinfo::AltMode;
use embedded_usb_pd::pdo::{self, sink, source, ExpectedPdo};
use embedded_usb_pd::{Error, LocalPortId, PdError};

use crate::registers::rx_caps::EPR_PDO_START_INDEX;
use crate::{registers, DeviceError, Mode, MAX_SUPPORTED_PORTS, PORT0, PORT1, TPS66993_NUM_PORTS, TPS66994_NUM_PORTS};

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

impl<B: I2c> device_driver::AsyncRegisterInterface for Port<'_, B> {
    type Error = Error<B::Error>;

    type AddressType = u8;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        // Sized to accommodate up to 255 bytes of data
        let mut buf = [0u8; 257];

        // Buffer length is sent as a byte
        if data.len() > 255 {
            return Err(PdError::InvalidParams.into());
        }

        buf[0] = address;
        buf[1] = data.len() as u8;
        let _ = &buf[2..data.len() + 2].copy_from_slice(data);

        self.bus
            .write(self.addr, &buf[..data.len() + 2])
            .await
            .map_err(Error::Bus)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        // Sized to accommodate length byte + up to 255 bytes of data
        let mut buf = [0u8; 256];
        let full_len = data.len() + 1;
        let reg = [address];

        if data.is_empty() {
            return Err(PdError::InvalidParams.into());
        }

        self.bus
            .write_read(self.addr, &reg, &mut buf[..full_len])
            .await
            .map_err(Error::Bus)?;

        let len = buf[0] as usize;
        if len < data.len() {
            PdError::InvalidParams.into()
        } else if len == 0xff || len == 0 {
            // Controller is busy and can't respond
            PdError::Busy.into()
        } else {
            data.copy_from_slice(&buf[1..data.len() + 1]);
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
}

impl<B: I2c> Tps6699x<B> {
    pub(super) fn new(bus: B, addr: [u8; MAX_SUPPORTED_PORTS], num_ports: usize) -> Self {
        Self { bus, addr, num_ports }
    }

    pub fn new_tps66993(bus: B, addr: u8) -> Self {
        Self::new(bus, [addr, 0x00], TPS66993_NUM_PORTS)
    }

    pub fn new_tps66994(bus: B, addr: [u8; 2]) -> Self {
        Self::new(bus, addr, TPS66994_NUM_PORTS)
    }

    /// Get the I2C address for a port
    fn port_addr(&self, port: LocalPortId) -> Result<u8, Error<B::Error>> {
        if port.0 as usize >= self.num_ports {
            PdError::InvalidPort.into()
        } else {
            Ok(self.addr[port.0 as usize])
        }
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

    /// Clear interrupts on a port, returns asserted interrupts
    pub async fn clear_interrupt(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::IntEventBus1, Error<B::Error>> {
        let p = self.borrow_port(port)?;
        let mut registers = p.into_registers();

        let flags = registers.int_event_bus_1().read_async().await?;
        // Clear interrupt if anything is set
        if flags != registers::field_sets::IntEventBus1::new_zero() {
            registers.int_clear_bus_1().write_async(|r| *r = flags).await?;
        }

        Ok(flags)
    }

    /// Modify interrupt mask
    pub async fn modify_interrupt_mask(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::field_sets::IntEventBus1) -> registers::field_sets::IntEventBus1,
    ) -> Result<registers::field_sets::IntEventBus1, Error<B::Error>> {
        let port = self.borrow_port(port)?;
        let mut registers = port.into_registers();
        registers.int_mask_bus_1().modify_async(|r| f(r)).await
    }

    /// Modify interrupt mask on all ports
    pub async fn modify_interrupt_mask_all(
        &mut self,
        f: impl Fn(&mut registers::field_sets::IntEventBus1) -> registers::field_sets::IntEventBus1,
    ) -> Result<(), Error<B::Error>> {
        for port in 0..self.num_ports() {
            let port = LocalPortId(port as u8);
            let _ = self.modify_interrupt_mask(port, &f).await?;
        }
        Ok(())
    }

    /// Get port status
    pub async fn get_port_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::Status, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().status().read_async().await
    }

    /// Get active PDO contract
    pub async fn get_active_pdo_contract(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::ActivePdoContract, Error<B::Error>> {
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
    ) -> Result<registers::field_sets::ActiveRdoContract, Error<B::Error>> {
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
        let mut buf = [0u8; registers::autonegotiate_sink::LEN];
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .read_register(
                registers::autonegotiate_sink::ADDR,
                (registers::autonegotiate_sink::LEN * 8) as u32,
                &mut buf,
            )
            .await?;

        Ok(buf.into())
    }

    /// Set the Autonegotiate Sink register (`0x37`).
    pub async fn set_autonegotiate_sink(
        &mut self,
        port: LocalPortId,
        value: registers::autonegotiate_sink::AutonegotiateSink,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .write_register(
                registers::autonegotiate_sink::ADDR,
                (registers::autonegotiate_sink::LEN * 8) as u32,
                value.as_bytes(),
            )
            .await
    }

    /// Modify autonegotiate sink settings
    pub async fn modify_autonegotiate_sink(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(
            &mut registers::autonegotiate_sink::AutonegotiateSink,
        ) -> registers::autonegotiate_sink::AutonegotiateSink,
    ) -> Result<registers::autonegotiate_sink::AutonegotiateSink, Error<B::Error>> {
        let mut reg = self.get_autonegotiate_sink(port).await?;
        reg = f(&mut reg);
        self.set_autonegotiate_sink(port, reg.clone()).await?;
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
    ) -> Result<registers::field_sets::PowerPathStatus, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(port)?
            .into_registers()
            .power_path_status()
            .read_async()
            .await
    }

    /// Get PD status
    pub async fn get_pd_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::PdStatus, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().pd_status().read_async().await
    }

    /// Get port control
    pub async fn get_port_control(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::PortControl, Error<B::Error>> {
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
        control: registers::field_sets::PortControl,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .port_control()
            .write_async(|r| *r = control)
            .await
    }

    /// Get global system config
    pub async fn get_system_config(&mut self) -> Result<registers::field_sets::SystemConfig, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(PORT0)?
            .into_registers()
            .system_config()
            .read_async()
            .await
    }

    /// Set global system config
    pub async fn set_system_config(
        &mut self,
        config: registers::field_sets::SystemConfig,
    ) -> Result<(), Error<B::Error>> {
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
    pub async fn get_boot_flags(&mut self) -> Result<registers::boot_flags::BootFlags, Error<B::Error>> {
        let mut buf = [0u8; registers::boot_flags::LEN];
        self.borrow_port(PORT0)?
            .into_registers()
            .interface()
            .read_register(
                registers::boot_flags::ADDR,
                (registers::boot_flags::LEN * 8) as u32,
                &mut buf,
            )
            .await?;

        Ok(registers::boot_flags::BootFlagsRaw(buf))
    }

    /// Get DP status
    pub async fn get_dp_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::dp_status::DpStatus, Error<B::Error>> {
        let mut buf = [0u8; registers::dp_status::LEN];
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .read_register(
                registers::dp_status::ADDR,
                (registers::dp_status::LEN * 8) as u32,
                &mut buf,
            )
            .await?;

        Ok(registers::dp_status::DpStatusRaw(buf))
    }

    /// Get Intel VID status
    pub async fn get_intel_vid_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::IntelVidStatus, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .intel_vid_status()
            .read_async()
            .await
    }

    /// Get USB status
    pub async fn get_usb_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::UsbStatus, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().usb_status().read_async().await
    }

    /// Get user VID status
    pub async fn get_user_vid_status(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::UserVidStatus, Error<B::Error>> {
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
            dp_status.dp_mode_active() != 0,
            intel_vid_status.tbt_mode_active(),
            usb_status.eudo_sop_sent_status() == registers::EudoSopSentStatus::SuccessfulEnterUsb,
        ))
    }

    /// Get DP config
    pub async fn get_dp_config(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::DpConfig, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().dp_config().read_async().await
    }

    /// Set DP config
    pub async fn set_dp_config(
        &mut self,
        port: LocalPortId,
        config: registers::field_sets::DpConfig,
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
        f: impl FnOnce(&mut registers::field_sets::DpConfig) -> registers::field_sets::DpConfig,
    ) -> Result<registers::field_sets::DpConfig, Error<B::Error>> {
        let port = self.borrow_port(port)?;
        let mut registers = port.into_registers();
        registers.dp_config().modify_async(|r| f(r)).await
    }

    /// Get Tbt config
    pub async fn get_tbt_config(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::TbtConfig, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().tbt_config().read_async().await
    }

    /// Set Tbt config
    pub async fn set_tbt_config(
        &mut self,
        port: LocalPortId,
        config: registers::field_sets::TbtConfig,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .tbt_config()
            .write_async(|r| *r = config)
            .await
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
        let mut buf = [0u8; registers::port_config::LEN];
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .read_register(
                registers::port_config::ADDR,
                (registers::port_config::LEN * 8) as u32,
                &mut buf,
            )
            .await?;
        Ok(buf.into())
    }

    /// Set port config
    pub async fn set_port_config(
        &mut self,
        port: LocalPortId,
        config: registers::port_config::PortConfig,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .write_register(
                registers::port_config::ADDR,
                (registers::port_config::LEN * 8) as u32,
                config.as_bytes(),
            )
            .await
    }

    /// Get Rx ADO
    pub async fn get_rx_ado(&mut self, port: LocalPortId) -> Result<registers::field_sets::RxAdo, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().rx_ado().read_async().await
    }

    /// Get Rx attention Vdm
    pub async fn get_rx_attn_vdm(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::field_sets::RxAttnVdm, Error<B::Error>> {
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
        let mut buf = [0u8; registers::rx_other_vdm::LEN];
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .read_register(
                registers::rx_other_vdm::ADDR,
                (registers::rx_other_vdm::LEN * 8) as u32,
                &mut buf,
            )
            .await?;
        Ok(buf.into())
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
    ) -> Result<(usize, usize), DeviceError<B::Error, ExpectedPdo>> {
        // Clamp to the maximum number of PDOs
        let num_pdos = if !out_epr_pdos.is_empty() {
            EPR_PDO_START_INDEX + out_epr_pdos.len()
        } else {
            // SPR PDOs start at index 0
            out_spr_pdos.len()
        }
        .min(registers::rx_caps::TOTAL_PDOS);

        // 4 bytes for each PDO
        let read_size = registers::rx_caps::HEADER_LEN + 4 * num_pdos;
        let mut buf = [0u8; registers::rx_caps::LEN];
        self.borrow_port(port)
            .map_err(DeviceError::from)?
            .into_registers()
            .interface()
            .read_register(register, (read_size * 8) as u32, &mut buf)
            .await?;

        let rx_caps = registers::rx_caps::RxCaps::<T>::try_from(buf).map_err(DeviceError::Other)?;
        let num_sprs = out_spr_pdos.len().min(rx_caps.num_valid_pdos() as usize);
        for (i, pdo) in out_spr_pdos.iter_mut().enumerate().take(num_sprs) {
            // SPR PDOs start at index 0
            *pdo = rx_caps[i];
        }

        let num_eprs = out_epr_pdos.len().min(rx_caps.num_valid_epr_pdos() as usize);
        for (i, pdo) in out_epr_pdos.iter_mut().enumerate().take(num_eprs) {
            *pdo = rx_caps[EPR_PDO_START_INDEX + i];
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
    ) -> Result<(usize, usize), DeviceError<B::Error, ExpectedPdo>> {
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
    ) -> Result<(usize, usize), DeviceError<B::Error, ExpectedPdo>> {
        self.get_rx_caps(port, registers::rx_caps::RX_SNK_ADDR, out_spr_pdos, out_epr_pdos)
            .await
    }

    /// Get Tx Identity
    pub async fn get_tx_identity(
        &mut self,
        port: LocalPortId,
    ) -> Result<registers::tx_identity::TxIdentity, Error<B::Error>> {
        let mut buf = [0u8; registers::tx_identity::LEN];
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .read_register(
                registers::tx_identity::ADDR,
                (registers::tx_identity::LEN * 8) as u32,
                &mut buf,
            )
            .await?;
        Ok(buf.into())
    }

    /// Set Tx Identity
    pub async fn set_tx_identity(
        &mut self,
        port: LocalPortId,
        value: registers::tx_identity::TxIdentity,
    ) -> Result<(), Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .write_register(
                registers::tx_identity::ADDR,
                (registers::tx_identity::LEN * 8) as u32,
                value.as_bytes(),
            )
            .await
    }

    /// Modify Tx Identity settings
    pub async fn modify_tx_identity(
        &mut self,
        port: LocalPortId,
        f: impl FnOnce(&mut registers::tx_identity::TxIdentity) -> registers::tx_identity::TxIdentity,
    ) -> Result<registers::tx_identity::TxIdentity, Error<B::Error>> {
        let mut reg = self.get_tx_identity(port).await?;
        reg = f(&mut reg);
        self.set_tx_identity(port, reg.clone()).await?;
        Ok(reg)
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::vec::Vec;

    use device_driver::AsyncRegisterInterface;
    use embedded_hal_async::i2c::ErrorType;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
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
        port.read_register(reg, (expected.len() * 8) as u32, &mut result)
            .await?;
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
        let r = port.read_register(reg, (expected.len() * 8) as u32, &mut result).await;
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
        port.read_register(reg, (expected.len() * 8) as u32, &mut result)
            .await?;
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
        port.write_register(reg, (expected.len() * 8) as u32, &expected).await?;
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
        if test_read_port_overread(tps6699x, port_id, expected_addr, reg, expected)
            .await
            .is_ok()
        {
            return Err(PdError::Failed.into());
        }

        Ok(())
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

    async fn test_clear_interrupt(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::field_sets::IntEventBus1;

        // Create a fully asserted interrupt register
        let int = !IntEventBus1::new_zero();
        let mut transactions = Vec::new();

        // Read the interrupt register
        transactions.push(create_register_read(expected_addr, 0x14, int));

        // Write to the interrupt clear register
        transactions.push(create_register_write(expected_addr, 0x18, int));
        tps6699x.bus.update_expectations(&transactions);

        assert_eq!(tps6699x.clear_interrupt(port).await.unwrap(), int);
        tps6699x.bus.done();
    }

    /// Test clearing interrupts with address set 0
    #[tokio::test]
    async fn test_clear_interrupt_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        test_clear_interrupt(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_clear_interrupt(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    /// Test clearing interrupts with address set 0
    #[tokio::test]
    async fn test_clear_interrupt_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR1);

        test_clear_interrupt(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_clear_interrupt(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_get_port_status(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::field_sets::Status;

        let mut transactions = Vec::new();
        // Read status register
        transactions.push(create_register_read(expected_addr, 0x1A, Status::new_zero()));
        tps6699x.bus.update_expectations(&transactions);

        let status = tps6699x.get_port_status(port).await.unwrap();
        assert_eq!(status, Status::new_zero());
        tps6699x.bus.done();
    }

    /// Test get port status on address set 0
    #[tokio::test]
    async fn test_get_port_status_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        test_get_port_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_get_port_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    /// Test get port status on address set 1
    #[tokio::test]
    async fn test_get_port_status_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR1);

        test_get_port_status(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_get_port_status(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_get_active_pdo_contract(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::field_sets::ActivePdoContract;

        let mut transactions = Vec::new();
        // Read status register
        transactions.push(create_register_read(expected_addr, 0x34, ActivePdoContract::new_zero()));
        tps6699x.bus.update_expectations(&transactions);

        let active_pdo_contract = tps6699x.get_active_pdo_contract(port).await.unwrap();
        assert_eq!(active_pdo_contract, ActivePdoContract::new_zero());
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_active_pdo_contract_ports_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        test_get_active_pdo_contract(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_get_active_pdo_contract(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_get_active_pdo_contract_ports_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR1);

        test_get_active_pdo_contract(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_get_active_pdo_contract(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_get_active_rdo_contract(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        use registers::field_sets::ActiveRdoContract;

        let mut transactions = Vec::new();
        // Read status register
        transactions.push(create_register_read(expected_addr, 0x35, ActiveRdoContract::new_zero()));
        tps6699x.bus.update_expectations(&transactions);

        let active_rdo_contract = tps6699x.get_active_rdo_contract(port).await.unwrap();
        assert_eq!(active_rdo_contract, ActiveRdoContract::new_zero());
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_active_rdo_contract_ports_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);

        test_get_active_rdo_contract(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_get_active_rdo_contract(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_get_active_rdo_contract_ports_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR1);

        test_get_active_rdo_contract(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_get_active_rdo_contract(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_get_mode(tps6699x: &mut Tps6699x<Mock>, expected_addr: u8, expected_mode: Mode) {
        let mut transactions = Vec::new();
        transactions.push(create_register_read(expected_addr, 0x03, expected_mode));
        tps6699x.bus.update_expectations(&transactions);

        let mode = tps6699x.get_mode().await.unwrap();
        assert_eq!(mode, expected_mode);
        tps6699x.bus.done();
    }

    async fn test_get_modes(tps6699x: &mut Tps6699x<Mock>, expected_addr: u8) {
        test_get_mode(tps6699x, expected_addr, Mode::Boot).await;
        test_get_mode(tps6699x, expected_addr, Mode::F211).await;
        test_get_mode(tps6699x, expected_addr, Mode::App0).await;
        test_get_mode(tps6699x, expected_addr, Mode::App1).await;
        test_get_mode(tps6699x, expected_addr, Mode::Wtpr).await;
    }

    #[tokio::test]
    async fn test_get_modes_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);
        test_get_modes(&mut tps6699x, PORT0_ADDR0).await;
    }

    #[tokio::test]
    async fn test_get_modes_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR1);
        test_get_modes(&mut tps6699x, PORT0_ADDR1).await;
    }

    async fn test_get_fw_version(tps6699x: &mut Tps6699x<Mock>, expected_addr: u8, expected_version: u32) {
        let mut transactions = Vec::new();
        transactions.push(create_register_read(
            expected_addr,
            0x0F,
            expected_version.to_le_bytes(),
        ));
        tps6699x.bus.update_expectations(&transactions);

        let version = tps6699x.get_fw_version().await.unwrap();
        assert_eq!(version, expected_version);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_fw_version_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);
        test_get_fw_version(&mut tps6699x, PORT0_ADDR0, TEST_FW_VERSION).await;
    }

    #[tokio::test]
    async fn test_get_fw_version_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR1);
        test_get_fw_version(&mut tps6699x, PORT0_ADDR1, TEST_FW_VERSION).await;
    }

    async fn test_get_customer_use(tps6699x: &mut Tps6699x<Mock>, expected_addr: u8, expected_value: u64) {
        let mut transactions = Vec::new();
        transactions.push(create_register_read(expected_addr, 0x06, expected_value.to_le_bytes()));
        tps6699x.bus.update_expectations(&transactions);

        let value = tps6699x.get_customer_use().await.unwrap();
        assert_eq!(value, expected_value);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_customer_use_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR0);
        test_get_customer_use(&mut tps6699x, PORT0_ADDR0, TEST_CUSTOMER_USE).await;
    }

    #[tokio::test]
    async fn test_get_customer_use_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new_tps66994(mock, ADDR1);
        test_get_customer_use(&mut tps6699x, PORT0_ADDR1, TEST_CUSTOMER_USE).await;
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

        // Expect a read of 14 bytes (header + 3 PDOs)
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
}
