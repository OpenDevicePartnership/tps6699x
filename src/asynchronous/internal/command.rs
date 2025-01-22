//! This module implements functions to access the command register and its associate data register.
//! The data register is larger than what device_driver can handle so access is done directly through the `AsyncRegisterInterface` trait.
use device_driver::AsyncRegisterInterface;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::{Error, PdError, PortId};

use super::Tps6699x;
use crate::command::*;
use crate::{error, registers as regs, Mode, PORT0};

impl<const N: usize, B: I2c> Tps6699x<N, B> {
    /// Sends a command without verifying that it is valid
    pub async fn send_command_unchecked(
        &mut self,
        port: PortId,
        cmd: Command,
        data: Option<&[u8]>,
    ) -> Result<(), Error<B::Error>> {
        let mut registers = self.borrow_port(port)?.into_registers();

        if let Some(data) = data {
            registers
                .interface()
                .write_register(regs::REG_DATA1, (data.len() * 8) as u32, data)
                .await?;
        }

        registers.cmd_1().write_async(|r| r.set_command(cmd as u32)).await?;

        Ok(())
    }

    /// Sends a command, verifying that it is valid
    pub async fn send_command(
        &mut self,
        delay: &mut impl DelayNs,
        port: PortId,
        cmd: Command,
        data: Option<&[u8]>,
    ) -> Result<(), Error<B::Error>> {
        self.send_command_unchecked(port, cmd, data).await?;

        delay.delay_us(cmd.valid_check_delay_us()).await;
        if Command::Invalid
            == self
                .borrow_port(port)?
                .into_registers()
                .cmd_1()
                .read_async()
                .await?
                .command()
        {
            return PdError::UnrecognizedCommand.into();
        }

        Ok(())
    }

    pub async fn check_command_complete(&mut self, port: PortId) -> Result<bool, Error<B::Error>> {
        let mut registers = self.borrow_port(port)?.into_registers();
        let status = registers.cmd_1().read_async().await?.command();

        Ok(Command::Success == status)
    }

    pub async fn read_command_result(
        &mut self,
        port: PortId,
        data: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        if !self.check_command_complete(port).await? {
            return PdError::Busy.into();
        }

        if let Some(ref data) = data {
            if data.len() > regs::REG_DATA1_LEN - 1 {
                // Data length too long
                return PdError::InvalidParams.into();
            }
        }

        // Read and return value and data
        let mut buf = [0u8; regs::REG_DATA1_LEN];
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .read_register(regs::REG_DATA1, (regs::REG_DATA1_LEN * 8) as u32, &mut buf)
            .await?;

        let ret = ReturnValue::try_from(buf[0]).map_err(Error::Pd)?;

        // Overwrite return value
        if let Some(data) = data {
            data.copy_from_slice(&buf[1..=data.len()]);
        }

        Ok(ret)
    }

    /// Reset the controller
    pub async fn reset(&mut self, delay: &mut impl DelayNs, args: &ResetArgs) -> Result<(), Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        let mut arg_bytes = [0u8; RESET_ARGS_LEN];

        args.encode_into_slice(&mut arg_bytes).map_err(Error::Pd)?;
        self.send_command_unchecked(PORT0, Command::Gaid, Some(&arg_bytes))
            .await?;

        delay.delay_ms(RESET_DELAY_MS).await;

        Ok(())
    }

    /// Enter firmware update mode
    pub async fn execute_tfus(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.send_command_unchecked(PORT0, Command::Tfus, None).await?;

        delay.delay_ms(TFUS_DELAY_MS).await;

        // Confirm we're in the correct mode
        let mode = self.get_mode().await?;
        if mode != Mode::F211 {
            error!("Failed to enter firmware update mode, mode: {:?}", mode);
            return Err(PdError::InvalidMode.into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use embedded_hal_mock::eh1::i2c::Mock;
    use regs::REG_DATA1;

    use crate::asynchronous::internal::Tps6699x;
    use crate::{ADDR0, ADDR1, PORT0};

    extern crate std;
    use std::vec::Vec;

    use super::*;
    use crate::test::*;
    use crate::TPS66994_NUM_PORTS;

    /// Value used for generic command testing, no particular significance
    const TEST_CMD_DATA: u64 = 0x12345678abcdef;

    // Use dual-port version to fully test port-specific code
    type Tps66994<B> = Tps6699x<TPS66994_NUM_PORTS, B>;

    async fn test_send_command<const N: usize>(
        tps6699x: &mut Tps66994<Mock>,
        expected_addr: u8,
        expected_cmd: Command,
        expected_data: Option<[u8; N]>,
    ) {
        let mut delay = Delay {};
        let mut transactions = Vec::new();

        // Create data write if supplied
        if let Some(data) = expected_data {
            transactions.push(create_register_write(expected_addr, REG_DATA1, data));
        }

        transactions.push(create_register_write(
            expected_addr,
            0x08,
            (expected_cmd as u32).to_le_bytes(),
        ));
        // Check that the command was valid
        transactions.push(create_register_read(expected_addr, 0x08, [0u8; 4]));
        tps6699x.bus.update_expectations(&transactions);

        if let Some(data) = expected_data {
            tps6699x
                .send_command(&mut delay, PORT0, expected_cmd, Some(&data))
                .await
                .unwrap();
        } else {
            tps6699x
                .send_command(&mut delay, PORT0, expected_cmd, None)
                .await
                .unwrap();
        }

        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_send_command_0() {
        let mut tps6699x = Tps66994::new(Mock::new(&[]), ADDR0);
        test_send_command::<0>(&mut tps6699x, PORT0_ADDR0, Command::Invalid, None).await;
        test_send_command(
            &mut tps6699x,
            PORT0_ADDR0,
            Command::Invalid,
            Some(TEST_CMD_DATA.to_le_bytes()),
        )
        .await;
    }

    #[tokio::test]
    async fn test_send_command_1() {
        let mut tps6699x = Tps66994::new(Mock::new(&[]), ADDR1);
        test_send_command::<0>(&mut tps6699x, PORT0_ADDR1, Command::Invalid, None).await;
        test_send_command(
            &mut tps6699x,
            PORT0_ADDR1,
            Command::Invalid,
            Some(TEST_CMD_DATA.to_le_bytes()),
        )
        .await;
    }

    async fn test_reset(tps6699x: &mut Tps66994<Mock>, expected_addr: u8, expected_args: ResetArgs) {
        let mut delay = Delay {};
        let mut transactions = Vec::new();

        let mut arg_bytes = [0u8; RESET_ARGS_LEN];
        expected_args.encode_into_slice(&mut arg_bytes).unwrap();

        transactions.push(create_register_write(expected_addr, REG_DATA1, arg_bytes));
        transactions.push(create_register_write(
            expected_addr,
            0x08,
            (Command::Gaid as u32).to_le_bytes(),
        ));
        tps6699x.bus.update_expectations(&transactions);

        tps6699x.reset(&mut delay, &expected_args).await.unwrap();
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_reset_0() {
        let mut tps6699x = Tps66994::new(Mock::new(&[]), ADDR0);
        test_reset(
            &mut tps6699x,
            PORT0_ADDR0,
            ResetArgs {
                switch_banks: true,
                copy_bank: false,
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_reset_1() {
        let mut tps6699x = Tps66994::new(Mock::new(&[]), ADDR1);
        test_reset(
            &mut tps6699x,
            PORT0_ADDR1,
            ResetArgs {
                switch_banks: true,
                copy_bank: false,
            },
        )
        .await;
    }

    async fn test_execute_tfus(tps6699x: &mut Tps66994<Mock>, expected_addr: u8) {
        let mut delay = Delay {};
        let mut transactions = Vec::new();

        transactions.push(create_register_write(
            expected_addr,
            0x08,
            (Command::Tfus as u32).to_le_bytes(),
        ));
        transactions.push(create_register_read(
            expected_addr,
            0x03,
            (Mode::F211 as u32).to_le_bytes(),
        ));
        tps6699x.bus.update_expectations(&transactions);

        tps6699x.execute_tfus(&mut delay).await.unwrap();
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_execute_tfus_0() {
        let mut tps6699x = Tps66994::new(Mock::new(&[]), ADDR0);
        test_execute_tfus(&mut tps6699x, PORT0_ADDR0).await;
    }

    #[tokio::test]
    async fn test_execute_tfus_1() {
        let mut tps6699x = Tps66994::new(Mock::new(&[]), ADDR1);
        test_execute_tfus(&mut tps6699x, PORT0_ADDR1).await;
    }
}
