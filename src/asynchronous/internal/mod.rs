//! Asynchronous, low-level TPS6699x driver. This module provides a low-level interface
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::{Error, PdError, PortId};

use crate::{registers, Mode, MAX_SUPPORTED_PORTS, PORT0, TPS66993_NUM_PORTS, TPS66994_NUM_PORTS};

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
        if len > data.len() {
            PdError::InvalidParams.into()
        } else if len == 0xff || len == 0 {
            // Controller is busy and can't respond
            PdError::Busy.into()
        } else {
            data.copy_from_slice(&buf[1..len + 1]);
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
    fn port_addr(&self, port: PortId) -> Result<u8, Error<B::Error>> {
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
    pub fn borrow_port(&mut self, port: PortId) -> Result<Port<'_, B>, Error<B::Error>> {
        let addr = self.port_addr(port)?;
        Ok(Port {
            bus: &mut self.bus,
            addr,
        })
    }

    /// Clear interrupts on a port, returns asserted interrupts
    pub async fn clear_interrupt(
        &mut self,
        port: PortId,
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

    /// Get port status
    pub async fn get_port_status(&mut self, port: PortId) -> Result<registers::field_sets::Status, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().status().read_async().await
    }

    /// Get active PDO contract
    pub async fn get_active_pdo_contract(
        &mut self,
        port: PortId,
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
        port: PortId,
    ) -> Result<registers::field_sets::ActiveRdoContract, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .active_rdo_contract()
            .read_async()
            .await
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
        self.borrow_port(PortId(0))?
            .into_registers()
            .customer_use()
            .read_async()
            .await
            .map(|r| r.customer_use())
    }

    /// Get power path status
    pub async fn get_power_path_status(
        &mut self,
        port: PortId,
    ) -> Result<registers::field_sets::PowerPathStatus, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(port)?
            .into_registers()
            .power_path_status()
            .read_async()
            .await
    }

    /// Get PD status
    pub async fn get_pd_status(&mut self, port: PortId) -> Result<registers::field_sets::PdStatus, Error<B::Error>> {
        self.borrow_port(port)?.into_registers().pd_status().read_async().await
    }

    /// Get port control
    pub async fn get_port_control(
        &mut self,
        port: PortId,
    ) -> Result<registers::field_sets::PortControl, Error<B::Error>> {
        self.borrow_port(port)?
            .into_registers()
            .port_control()
            .read_async()
            .await
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::vec::Vec;

    use device_driver::AsyncRegisterInterface;
    use embedded_hal_async::i2c::ErrorType;
    use embedded_hal_mock::eh1::i2c::Mock;

    use super::*;
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
        port_id: PortId,
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

    async fn test_write_port<const N: usize>(
        tps6699x: &mut Tps6699x<Mock>,
        port_id: PortId,
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
        port_id: PortId,
        expected_addr: u8,
        reg: u8,
        expected: [u8; N],
    ) -> Result<(), Error<<Mock as ErrorType>::Error>> {
        test_read_port::<N>(tps6699x, port_id, expected_addr, reg, expected).await?;
        test_write_port::<N>(tps6699x, port_id, expected_addr, reg, expected).await
    }

    async fn test_rw_ports(tps6699x: &mut Tps6699x<Mock>, port_id: PortId, expected_addr: u8) {
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

    async fn test_clear_interrupt(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
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

    async fn test_get_port_status(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
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

    async fn test_get_active_pdo_contract(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
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

    async fn test_get_active_rdo_contract(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
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
}
