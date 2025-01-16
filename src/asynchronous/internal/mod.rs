//! This module implements a low-level TPS6699x driver. The super module provides a high-level API
//! that uses standard PD types and provides functions that implement features that may occur over several interrupts
use defmt::debug;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::{Error, PdError, PortId};

use crate::command::{Command, RESET_DELAY_MS};
use crate::{registers, Mode};

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

        debug!(
            "PD write, address: {:#x}, register: {:#x}, length: {}",
            self.addr,
            address,
            data.len()
        );
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

        debug!(
            "PD read, address: {:#x}, register: {:#x}, length: {}",
            self.addr,
            address,
            data.len()
        );

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
        } else {
            data.copy_from_slice(&buf[1..len + 1]);
            Ok(())
        }
    }
}

pub struct Tps6699x<B: I2c> {
    pub(super) bus: B,
    /// I2C addresses for ports
    addr: [u8; 2],
}

impl<B: I2c> Tps6699x<B> {
    pub fn new(bus: B, addr: [u8; 2]) -> Self {
        Self { bus, addr }
    }

    /// Get the I2C address for a port
    fn port_addr(&self, port: PortId) -> Result<u8, Error<B::Error>> {
        if port.0 as usize > self.addr.len() {
            PdError::InvalidPort.into()
        } else {
            Ok(self.addr[port.0 as usize])
        }
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
        let mode = self
            .borrow_port(PortId(0))?
            .into_registers()
            .mode()
            .read_async()
            .await?;
        let mode = Mode::try_from(mode.mode()).map_err(Error::Pd)?;
        Ok(mode)
    }

    /// Get FW version
    pub async fn get_fw_version(&mut self) -> Result<u32, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(PortId(0))?
            .into_registers()
            .version()
            .read_async()
            .await
            .map(|r| r.version())
    }

    pub async fn get_customer_use(&mut self) -> Result<u64, Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.borrow_port(PortId(0))?
            .into_registers()
            .customer_use()
            .read_async()
            .await
            .map(|r| r.customer_use())
    }
}

#[cfg(test)]
mod test {
    use device_driver::AsyncRegisterInterface;
    use embedded_hal_async::i2c::ErrorKind;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use super::*;
    use crate::command::{ReturnValue, REG_DATA1_LEN};
    use crate::{ADDR0, ADDR1};

    const PORT0: PortId = PortId(0);
    const PORT1: PortId = PortId(1);

    /// Default I2C addresse for testing
    const PORT0_ADDR0: u8 = 0x20;
    /// Default I2C addresse for testing
    const PORT1_ADDR0: u8 = 0x24;
    /// Default I2C addresse for testing
    const PORT0_ADDR1: u8 = 0x21;
    /// Default I2C addresse for testing
    const PORT1_ADDR1: u8 = 0x25;

    /// Test helper for reading successfully from a port
    async fn test_read_port_success(
        port: &mut Port<'_, Mock>,
        reg: u8,
        expected_addr: u8,
        buf: &mut [u8],
        expected: &[u8],
    ) -> Result<(), Error<ErrorKind>> {
        let mut response = Vec::with_capacity(expected.len() + 1);
        response.push(expected.len() as u8);
        response.splice(1..1, expected.iter().cloned());

        let transaction = [Transaction::write_read(expected_addr, vec![reg], response)];

        buf.fill(0);
        port.bus.update_expectations(&transaction);
        port.read_register(reg, 0, buf).await?;
        port.bus.done();
        assert_eq!(&buf[..expected.len()], expected);
        Ok(())
    }

    /// Tests successfully reading from both ports
    async fn test_read_ports_success(addr: [u8; 2]) {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, addr);
        let mut buf = [0u8; 3];
        let expected = [0x00, 0x01, 0x02];

        let mut port = tps6699x.borrow_port(PORT0).unwrap();
        assert_eq!(port.addr, addr[0]);
        test_read_port_success(&mut port, 0x00, addr[0], &mut buf, &expected)
            .await
            .unwrap();

        let mut port = tps6699x.borrow_port(PORT1).unwrap();
        assert_eq!(port.addr, addr[1]);
        test_read_port_success(&mut port, 0x00, addr[1], &mut buf, &expected)
            .await
            .unwrap();
    }

    /// Test helper for read failures
    async fn test_read_port_failure(
        port: &mut Port<'_, Mock>,
        reg: u8,
        expected_addr: u8,
        buf: &mut [u8],
        expected: &[u8],
    ) -> Result<(), Error<ErrorKind>> {
        // I2C mock will still check transactions so create an undersized read
        let mut response = Vec::with_capacity(expected.len());
        response.push(expected.len() as u8);
        response.splice(1..1, expected[..buf.len()].iter().cloned());

        let transaction = [Transaction::write_read(expected_addr, vec![reg], response)];

        buf.fill(0);
        port.bus.update_expectations(&transaction);
        let res = port.read_register(reg, 0, buf).await.map(|_| ());
        port.bus.done();
        res
    }

    // Test read failures on both ports
    async fn test_read_ports_failure(addr: [u8; 2]) {
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(Mock::new(&[]), addr);
        let mut buf = [0u8; 2];
        let expected = [0x00, 0x01, 0x02];

        let mut port = tps6699x.borrow_port(PORT0).unwrap();
        assert_eq!(
            test_read_port_failure(&mut port, 0x00, addr[0], &mut buf, &expected)
                .await
                .unwrap_err(),
            Error::Pd(PdError::InvalidParams)
        );

        let mut port = tps6699x.borrow_port(PORT1).unwrap();
        assert_eq!(
            test_read_port_failure(&mut port, 0x00, addr[1], &mut buf, &expected)
                .await
                .unwrap_err(),
            Error::Pd(PdError::InvalidParams)
        );
    }

    /// Test address set 0
    #[tokio::test]
    async fn test_read_ports_0() {
        test_read_ports_success(ADDR0).await;
        test_read_ports_failure(ADDR0).await;
    }

    /// Test address set 1
    #[tokio::test]
    async fn test_read_ports_1() {
        test_read_ports_success(ADDR1).await;
        test_read_ports_failure(ADDR1).await;
    }

    /// Test helper for writing successfully to a port
    async fn test_port_write_success(
        port: &mut Port<'_, Mock>,
        reg: u8,
        expected_addr: u8,
        data: &[u8],
    ) -> Result<(), Error<ErrorKind>> {
        let mut expected = Vec::with_capacity(data.len() + 2);
        expected.push(reg);
        expected.push(data.len() as u8);
        expected.splice(2..2, data.iter().cloned());

        let transaction = [Transaction::write(expected_addr, expected)];

        port.bus.update_expectations(&transaction);
        port.write_register(reg, 0, data).await?;
        port.bus.done();
        Ok(())
    }

    /// Test writing successfully to both ports
    async fn test_write_ports_success(addr: [u8; 2]) {
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(Mock::new(&[]), addr);
        let expected = [0x00, 0x01, 0x02];

        let mut port = tps6699x.borrow_port(PORT0).unwrap();
        test_port_write_success(&mut port, 0x00, addr[0], &expected)
            .await
            .unwrap();

        let mut port = tps6699x.borrow_port(PORT1).unwrap();
        test_port_write_success(&mut port, 0x00, addr[1], &expected)
            .await
            .unwrap();
    }

    /// Test writing failures on both ports
    async fn test_write_ports_failure(addr: [u8; 2]) {
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(Mock::new(&[]), addr);
        let expected = [0u8; 256];

        let mut port = tps6699x.borrow_port(PORT0).unwrap();
        assert_eq!(
            port.write_register(0x00, 0, &expected).await.unwrap_err(),
            Error::Pd(PdError::InvalidParams)
        );

        let mut port = tps6699x.borrow_port(PORT1).unwrap();
        assert_eq!(
            port.write_register(0x00, 0, &expected).await.unwrap_err(),
            Error::Pd(PdError::InvalidParams)
        );

        // Needed otherwise the mock will panic on drop
        tps6699x.bus.done();
    }

    /// Test address set 0
    #[tokio::test]
    async fn test_write_ports_0() {
        test_write_ports_success(ADDR0).await;
        test_write_ports_failure(ADDR0).await;
    }

    /// Test address set 1
    #[tokio::test]
    async fn test_write_ports_1() {
        test_write_ports_success(ADDR1).await;
        test_write_ports_failure(ADDR1).await;
    }

    fn create_register_read<const N: usize, R: Into<[u8; N]>>(addr: u8, reg: u8, value: R) -> Vec<Transaction> {
        // +1 for the length byte
        let mut response = Vec::with_capacity(N + 1);
        response.push(N as u8);
        response.splice(1..1, value.into().iter().cloned());

        vec![Transaction::write_read(addr, vec![reg], response)]
    }

    fn create_register_write<const N: usize, R: Into<[u8; N]>>(addr: u8, reg: u8, value: R) -> Vec<Transaction> {
        // +1 for the register + length byte
        let mut response = Vec::with_capacity(N + 2);
        response.push(reg);
        response.push(N as u8);
        response.splice(2..2, value.into().iter().cloned());

        vec![Transaction::write(addr, response)]
    }

    async fn test_clear_interrupt(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
        use registers::field_sets::IntEventBus1;

        // Create a fully asserted interrupt register
        let int = !IntEventBus1::new_zero();
        let mut transactions = Vec::new();

        // Read the interrupt register
        transactions.extend(create_register_read(expected_addr, 0x14, int).into_iter());

        // Write to the interrupt clear register
        transactions.extend(create_register_write(expected_addr, 0x18, int).into_iter());
        tps6699x.bus.update_expectations(&transactions);

        assert_eq!(tps6699x.clear_interrupt(port).await.unwrap(), int);
        tps6699x.bus.done();
    }

    /// Test clearing interrupts with address set 0
    #[tokio::test]
    async fn test_clear_interrupt_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR0);

        test_clear_interrupt(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_clear_interrupt(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    /// Test clearing interrupts with address set 0
    #[tokio::test]
    async fn test_clear_interrupt_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR1);

        test_clear_interrupt(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_clear_interrupt(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_get_port_status(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
        use registers::field_sets::Status;

        let mut transactions = Vec::new();
        // Read status register
        transactions.extend(create_register_read(expected_addr, 0x1A, Status::new_zero()).into_iter());
        tps6699x.bus.update_expectations(&transactions);

        let status = tps6699x.get_port_status(port).await.unwrap();
        assert_eq!(status, Status::new_zero());
        tps6699x.bus.done();
    }

    /// Test get port status on address set 0
    #[tokio::test]
    async fn test_get_port_status_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR0);

        test_get_port_status(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_get_port_status(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    /// Test get port status on address set 0
    #[tokio::test]
    async fn test_get_port_status_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR1);

        test_get_port_status(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_get_port_status(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_get_active_pdo_contract(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
        use registers::field_sets::ActivePdoContract;

        let mut transactions = Vec::new();
        // Read status register
        transactions.extend(create_register_read(expected_addr, 0x34, ActivePdoContract::new_zero()).into_iter());
        tps6699x.bus.update_expectations(&transactions);

        let active_pdo_contract = tps6699x.get_active_pdo_contract(port).await.unwrap();
        assert_eq!(active_pdo_contract, ActivePdoContract::new_zero());
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_active_pdo_contract_ports_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR0);

        test_get_active_pdo_contract(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_get_active_pdo_contract(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_get_active_pdo_contract_ports_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR1);

        test_get_active_pdo_contract(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_get_active_pdo_contract(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_get_active_rdo_contract(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
        use registers::field_sets::ActiveRdoContract;

        let mut transactions = Vec::new();
        // Read status register
        transactions.extend(create_register_read(expected_addr, 0x35, ActiveRdoContract::new_zero()).into_iter());
        tps6699x.bus.update_expectations(&transactions);

        let active_rdo_contract = tps6699x.get_active_rdo_contract(port).await.unwrap();
        assert_eq!(active_rdo_contract, ActiveRdoContract::new_zero());
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_get_active_rdo_contract_ports_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR0);

        test_get_active_rdo_contract(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_get_active_rdo_contract(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_get_active_rdo_contract_ports_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR1);

        test_get_active_rdo_contract(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_get_active_rdo_contract(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_send_raw_command(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
        use registers::field_sets::Cmd1;

        let mut transactions = Vec::new();
        let mut cmd = Cmd1::new_zero();

        cmd.set_command(Command::Gaid as u32);

        // Test without data
        transactions.extend(create_register_write(expected_addr, 0x08, cmd).into_iter());
        transactions.extend(create_register_read(expected_addr, 0x08, cmd).into_iter());
        tps6699x.bus.update_expectations(&transactions);
        tps6699x.send_raw_command(port, Command::Gaid, None).await.unwrap();
        tps6699x.bus.done();

        // Test with data
        transactions.clear();
        transactions.extend(create_register_write(expected_addr, 0x09, [0xaa, 0xbb]));
        transactions.extend(create_register_write(expected_addr, 0x08, cmd).into_iter());
        transactions.extend(create_register_read(expected_addr, 0x08, cmd).into_iter());
        tps6699x.bus.update_expectations(&transactions);
        tps6699x
            .send_raw_command(port, Command::Gaid, Some(&[0xaa, 0xbb]))
            .await
            .unwrap();
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_send_raw_command_ports_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR0);

        test_send_raw_command(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_send_raw_command(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_send_raw_command_ports_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR1);

        test_send_raw_command(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_send_raw_command(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_read_command_result(tps6699x: &mut Tps6699x<Mock>, port: PortId, expected_addr: u8) {
        let mut transactions = Vec::new();

        // Return value, but no data
        transactions.extend(create_register_read(expected_addr, 0x08, [0x00, 0x00, 0x00, 0x00]).into_iter());
        transactions.extend(create_register_read(expected_addr, 0x09, [0x00; REG_DATA1_LEN]).into_iter());
        tps6699x.bus.update_expectations(&transactions);
        let ret = tps6699x.read_command_result(port, None).await.unwrap();
        assert_eq!(ret, ReturnValue::Success);
        tps6699x.bus.done();

        // Return value and data
        let mut buf = [0u8; 2];
        transactions.clear();
        transactions.extend(create_register_read(expected_addr, 0x08, [0x00, 0x00, 0x00, 0x00]).into_iter());

        let mut expected = [0x00; REG_DATA1_LEN];
        let _ = &mut expected[1..3].copy_from_slice(&[0x02, 0x03]);
        transactions.extend(create_register_read(expected_addr, 0x09, expected).into_iter());

        tps6699x.bus.update_expectations(&transactions);
        let ret = tps6699x.read_command_result(port, Some(&mut buf)).await.unwrap();
        assert_eq!(ret, ReturnValue::Success);
        assert_eq!(buf, [0x02, 0x03]);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_read_command_results_ports_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR0);

        test_read_command_result(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        test_read_command_result(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }

    #[tokio::test]
    async fn test_read_command_results_ports_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR1);

        test_read_command_result(&mut tps6699x, PORT0, PORT0_ADDR1).await;
        test_read_command_result(&mut tps6699x, PORT1, PORT1_ADDR1).await;
    }

    async fn test_get_mode(tps6699x: &mut Tps6699x<Mock>, expected_addr: u8, expected_mode: Mode) {
        let mut transactions = Vec::new();
        transactions.extend(create_register_read(expected_addr, 0x03, expected_mode).into_iter());
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
    async fn test_get_modes_address_0() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR0);
        test_get_modes(&mut tps6699x, PORT0_ADDR0).await;
    }

    #[tokio::test]
    async fn test_get_modes_address_1() {
        let mock = Mock::new(&[]);
        let mut tps6699x: Tps6699x<Mock> = Tps6699x::new(mock, ADDR1);
        test_get_modes(&mut tps6699x, PORT0_ADDR1).await;
    }
}
