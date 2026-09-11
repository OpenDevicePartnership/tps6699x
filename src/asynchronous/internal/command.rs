//! This module implements functions to access the command register and its associate data register.
//! The data register is larger than what device_driver can handle so access is done directly through the `AsyncRegisterInterface` trait.
use bincode::config;
use device_driver::AsyncRegisterInterface;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::{Error, LocalPortId, PdError};

use super::Tps6699x;
use crate::command::*;
use crate::{debug, error, registers as regs, Mode, PORT0};

impl<B: I2c> Tps6699x<B> {
    /// Sends a command without verifying that it is valid
    pub async fn send_command(
        &mut self,
        port: LocalPortId,
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

    /// Check if the command has completed
    pub async fn check_command_complete(&mut self, port: LocalPortId) -> Result<bool, Error<B::Error>> {
        let status = self
            .borrow_port(port)?
            .into_registers()
            .cmd_1()
            .read_async()
            .await?
            .command();

        match Command::try_from(status).map_err(Error::Pd)? {
            Command::Success => Ok(true),
            Command::Invalid => Err(PdError::UnrecognizedCommand.into()),
            _ => Ok(false),
        }
    }

    /// Read the result of a command
    pub async fn read_command_result(
        &mut self,
        port: LocalPortId,
        data: Option<&mut [u8]>,
        has_return_value: bool,
    ) -> Result<ReturnValue, Error<B::Error>> {
        match self.check_command_complete(port).await {
            Ok(true) => {
                debug!("command completed");
            }
            Ok(false) => {
                return PdError::Busy.into();
            }
            Err(e) => {
                return Err(e);
            }
        }

        let max_len = if has_return_value {
            // -1 because one byte is used for the return value
            regs::REG_DATA1_LEN - 1
        } else {
            regs::REG_DATA1_LEN
        };

        if let Some(ref data) = data {
            if data.len() > max_len {
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

        if has_return_value {
            let return_code = buf[0] & CMD_4CC_TASK_RETURN_CODE_MASK;
            let ret = ReturnValue::try_from(return_code).map_err(Error::Pd)?;
            debug!("read_command_result: ret: {:?}", ret);
            // Overwrite return value
            if let Some(data) = data {
                data.copy_from_slice(buf.get(1..=data.len()).ok_or(PdError::InvalidParams)?);
            }
            Ok(ret)
        } else {
            // No return value to check
            debug!("read_command_result: Done");
            if let Some(data) = data {
                data.copy_from_slice(buf.get(..data.len()).ok_or(PdError::InvalidParams)?);
            }
            Ok(ReturnValue::Success)
        }
    }

    /// Reset the controller
    pub async fn reset(&mut self, delay: &mut impl DelayNs, args: &ResetArgs) -> Result<(), Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        let mut arg_bytes = [0u8; RESET_ARGS_LEN];

        bincode::encode_into_slice(args, &mut arg_bytes, config::standard().with_fixed_int_encoding())
            .map_err(|_| Error::Pd(PdError::Serialize))?;
        self.send_command(PORT0, Command::Gaid, Some(&arg_bytes)).await?;

        delay.delay_ms(RESET_DELAY_MS).await;

        Ok(())
    }

    /// Enter firmware update mode
    pub async fn execute_tfus(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        self.send_command(PORT0, Command::Tfus, None).await?;

        delay.delay_ms(TFUS_DELAY_MS).await;

        // Confirm we're in the correct mode
        let mode = self.get_mode().await?;
        if mode != Mode::F211 {
            error!("Failed to enter firmware update mode, mode: {:?}", mode);
            return Err(PdError::InvalidMode.into());
        }
        Ok(())
    }

    /// Complete firmware update
    pub async fn execute_tfuc(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        let mut arg_bytes = [0u8; RESET_ARGS_LEN];

        let args = ResetArgs {
            switch_banks: false,
            copy_bank: true,
        };

        bincode::encode_into_slice(args, &mut arg_bytes, config::standard().with_fixed_int_encoding())
            .map_err(|_| Error::Pd(PdError::Serialize))?;

        // This is a controller-level command, shouldn't matter which port we use
        let port = LocalPortId(0);
        self.send_command(port, Command::Tfuc, Some(&arg_bytes)).await?;

        delay.delay_ms(RESET_DELAY_MS).await;

        let mut elapsed_ms = RESET_DELAY_MS;
        loop {
            let mode = self.get_mode().await?;
            match mode {
                Mode::App0 | Mode::App1 => return Ok(()),
                Mode::Boot if elapsed_ms < TFUC_APP_VERIFY_WINDOW_MS => {
                    let poll_delay_ms = TFUC_MODE_POLL_INTERVAL_MS.min(TFUC_APP_VERIFY_WINDOW_MS - elapsed_ms);
                    delay.delay_ms(poll_delay_ms).await;
                    elapsed_ms += poll_delay_ms;
                }
                Mode::Boot => {
                    error!("Timed out waiting for normal mode, last mode: {:?}", mode);
                    return Err(PdError::Timeout.into());
                }
                _ => {
                    error!("Failed to enter normal mode, mode: {:?}", mode);
                    return Err(PdError::InvalidMode.into());
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use embedded_hal::i2c::ErrorKind;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use regs::REG_DATA1;

    use crate::asynchronous::internal::Tps6699x;
    use crate::{ADDR0, PORT0, PORT1};

    extern crate std;
    use std::vec::Vec;

    use super::*;
    use crate::test::*;

    /// Value used for generic command testing, no particular significance
    const TEST_CMD_DATA: u64 = 0x12345678abcdef;

    #[derive(Default)]
    struct RecordingDelay {
        delays_ns: Vec<u32>,
    }

    impl DelayNs for RecordingDelay {
        async fn delay_ns(&mut self, ns: u32) {
            self.delays_ns.push(ns);
        }
    }

    impl RecordingDelay {
        fn total_delay_ms(&self) -> u64 {
            self.delays_ns.iter().map(|&ns| u64::from(ns)).sum::<u64>() / 1_000_000
        }
    }

    fn tfuc_transactions(modes: &[Mode]) -> Vec<Transaction> {
        let mut transactions = Vec::new();
        transactions.push(create_register_write(PORT0_ADDR0, REG_DATA1, [0, RESET_FEATURE_ENABLE]));
        transactions.push(create_register_write(
            PORT0_ADDR0,
            0x08,
            (Command::Tfuc as u32).to_le_bytes(),
        ));
        transactions.extend(
            modes
                .iter()
                .map(|&mode| create_register_read(PORT0_ADDR0, 0x03, (mode as u32).to_le_bytes())),
        );
        transactions
    }

    async fn run_send_command<const N: usize>(
        tps6699x: &mut Tps6699x<Mock>,
        port: LocalPortId,
        expected_addr: u8,
        expected_cmd: Command,
        expected_data: Option<[u8; N]>,
    ) {
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
        tps6699x.bus.update_expectations(&transactions);

        if let Some(data) = expected_data {
            tps6699x.send_command(port, expected_cmd, Some(&data)).await.unwrap();
        } else {
            tps6699x.send_command(port, expected_cmd, None).await.unwrap();
        }

        tps6699x.bus.done();
    }

    /// Test send_command on both ports with ADDR0
    #[tokio::test]
    async fn test_send_command() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        run_send_command::<0>(&mut tps6699x, PORT0, PORT0_ADDR0, Command::Invalid, None).await;
        run_send_command(
            &mut tps6699x,
            PORT0,
            PORT0_ADDR0,
            Command::Invalid,
            Some(TEST_CMD_DATA.to_le_bytes()),
        )
        .await;
        run_send_command::<0>(&mut tps6699x, PORT1, PORT1_ADDR0, Command::Invalid, None).await;
        run_send_command(
            &mut tps6699x,
            PORT1,
            PORT1_ADDR0,
            Command::Invalid,
            Some(TEST_CMD_DATA.to_le_bytes()),
        )
        .await;
    }

    #[tokio::test]
    async fn test_reset() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        let mut delay = Delay {};
        let mut transactions = Vec::new();
        let expected_args = ResetArgs {
            switch_banks: true,
            copy_bank: false,
        };

        let mut arg_bytes = [0u8; RESET_ARGS_LEN];
        bincode::encode_into_slice(
            &expected_args,
            &mut arg_bytes,
            config::standard().with_fixed_int_encoding(),
        )
        .unwrap();

        transactions.push(create_register_write(PORT0_ADDR0, REG_DATA1, arg_bytes));
        transactions.push(create_register_write(
            PORT0_ADDR0,
            0x08,
            (Command::Gaid as u32).to_le_bytes(),
        ));
        tps6699x.bus.update_expectations(&transactions);

        tps6699x.reset(&mut delay, &expected_args).await.unwrap();
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_execute_tfus() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        let mut delay = Delay {};
        let mut transactions = Vec::new();

        transactions.push(create_register_write(
            PORT0_ADDR0,
            0x08,
            (Command::Tfus as u32).to_le_bytes(),
        ));
        transactions.push(create_register_read(
            PORT0_ADDR0,
            0x03,
            (Mode::F211 as u32).to_le_bytes(),
        ));
        tps6699x.bus.update_expectations(&transactions);

        tps6699x.execute_tfus(&mut delay).await.unwrap();
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_execute_tfuc_immediate_app_success() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        let mut delay = RecordingDelay::default();
        let transactions = tfuc_transactions(&[Mode::App0]);

        tps6699x.bus.update_expectations(&transactions);

        tps6699x.execute_tfuc(&mut delay).await.unwrap();
        assert_eq!(delay.total_delay_ms(), u64::from(RESET_DELAY_MS));
        assert_eq!(delay.delays_ns.len(), 1);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_execute_tfuc_boot_then_app_success() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        let mut delay = RecordingDelay::default();
        let transactions = tfuc_transactions(&[Mode::Boot, Mode::App1]);

        tps6699x.bus.update_expectations(&transactions);

        tps6699x.execute_tfuc(&mut delay).await.unwrap();
        assert_eq!(
            delay.total_delay_ms(),
            u64::from(RESET_DELAY_MS + TFUC_MODE_POLL_INTERVAL_MS)
        );
        assert_eq!(
            delay.delays_ns,
            [RESET_DELAY_MS * 1_000_000, TFUC_MODE_POLL_INTERVAL_MS * 1_000_000]
        );
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_execute_tfuc_repeated_boot_times_out_at_verification_deadline() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        let mut delay = RecordingDelay::default();
        let poll_count = (TFUC_APP_VERIFY_WINDOW_MS - RESET_DELAY_MS) / TFUC_MODE_POLL_INTERVAL_MS;
        let modes = std::vec![Mode::Boot; poll_count as usize + 1];
        let transactions = tfuc_transactions(&modes);

        tps6699x.bus.update_expectations(&transactions);

        assert_eq!(
            tps6699x.execute_tfuc(&mut delay).await,
            Err(Error::Pd(PdError::Timeout))
        );
        assert_eq!(delay.total_delay_ms(), u64::from(TFUC_APP_VERIFY_WINDOW_MS));
        assert_eq!(delay.delays_ns.len(), poll_count as usize + 1);
        assert!(delay
            .delays_ns
            .iter()
            .skip(1)
            .all(|&ns| ns == TFUC_MODE_POLL_INTERVAL_MS * 1_000_000));
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_execute_tfuc_accepts_app_at_verification_deadline() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        let mut delay = RecordingDelay::default();
        let poll_count = (TFUC_APP_VERIFY_WINDOW_MS - RESET_DELAY_MS) / TFUC_MODE_POLL_INTERVAL_MS;
        let mut modes = std::vec![Mode::Boot; poll_count as usize];
        modes.push(Mode::App1);
        let transactions = tfuc_transactions(&modes);

        tps6699x.bus.update_expectations(&transactions);

        tps6699x.execute_tfuc(&mut delay).await.unwrap();
        assert_eq!(delay.total_delay_ms(), u64::from(TFUC_APP_VERIFY_WINDOW_MS));
        assert_eq!(delay.delays_ns.len(), poll_count as usize + 1);
        assert!(delay
            .delays_ns
            .iter()
            .skip(1)
            .all(|&ns| ns == TFUC_MODE_POLL_INTERVAL_MS * 1_000_000));
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_execute_tfuc_unexpected_mode_fails_immediately() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        let mut delay = RecordingDelay::default();
        let transactions = tfuc_transactions(&[Mode::F211]);

        tps6699x.bus.update_expectations(&transactions);

        assert_eq!(
            tps6699x.execute_tfuc(&mut delay).await,
            Err(Error::Pd(PdError::InvalidMode))
        );
        assert_eq!(delay.total_delay_ms(), u64::from(RESET_DELAY_MS));
        assert_eq!(delay.delays_ns.len(), 1);
        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_execute_tfuc_propagates_mode_read_bus_error() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);
        let mut delay = RecordingDelay::default();
        let mut transactions = tfuc_transactions(&[]);
        transactions.push(
            create_register_read(PORT0_ADDR0, 0x03, (Mode::App0 as u32).to_le_bytes()).with_error(ErrorKind::Other),
        );

        tps6699x.bus.update_expectations(&transactions);

        assert_eq!(
            tps6699x.execute_tfuc(&mut delay).await,
            Err(Error::Bus(ErrorKind::Other))
        );
        assert_eq!(delay.total_delay_ms(), u64::from(RESET_DELAY_MS));
        assert_eq!(delay.delays_ns.len(), 1);
        tps6699x.bus.done();
    }

    async fn run_check_command_complete(tps6699x: &mut Tps6699x<Mock>, port: LocalPortId, expected_addr: u8) {
        let mut transactions = Vec::new();

        // Successful command
        transactions.push(create_register_read(expected_addr, 0x08, [0x00, 0x00, 0x00, 0x00]));
        // Invalid command
        transactions.push(create_register_read(expected_addr, 0x08, [0x21, 0x43, 0x4D, 0x44]));
        // Command still in progress
        transactions.push(create_register_read(expected_addr, 0x08, [0x47, 0x41, 0x49, 0x44]));

        tps6699x.bus.update_expectations(&transactions);

        assert_eq!(tps6699x.check_command_complete(port).await, Ok(true));
        assert_eq!(
            tps6699x.check_command_complete(port).await,
            Err(Error::Pd(PdError::UnrecognizedCommand))
        );
        assert_eq!(tps6699x.check_command_complete(port).await, Ok(false));

        tps6699x.bus.done();
    }

    #[tokio::test]
    async fn test_check_command_complete() {
        let mut tps6699x = Tps6699x::new_tps66994(Mock::new(&[]), ADDR0);

        run_check_command_complete(&mut tps6699x, PORT0, PORT0_ADDR0).await;
        run_check_command_complete(&mut tps6699x, PORT1, PORT1_ADDR0).await;
    }
}
