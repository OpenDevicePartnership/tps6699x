//! Support for 4CC commands which communicate through the CMD1 and DATA1 registers
//! The DATA1 register exceeds the 128 bit limit of the device driver crate so we have to handle it manually
use device_driver::AsyncRegisterInterface;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::Error;

use super::*;
use crate::command::{ReturnValue, REG_DATA1, REG_DATA1_LEN};

// These are controller-level commands, we use borrow_port just for convenience
impl<B: I2c> Tps6699x<B> {
    /// Sends the command, not checking if the command is valid
    pub async fn send_raw_command_unchecked(
        &mut self,
        port: PortId,
        cmd: Command,
        data: Option<&[u8]>,
    ) -> Result<(), Error<B::Error>> {
        let mut registers = self.borrow_port(port)?.into_registers();

        if let Some(data) = data {
            registers
                .interface()
                .write_register(REG_DATA1, (data.len() * 8) as u32, data)
                .await?;
        }

        registers.cmd_1().write_async(|r| r.set_command(cmd as u32)).await
    }

    /// Sends the command, verifying that it is valid
    pub async fn send_raw_command(
        &mut self,
        port: PortId,
        cmd: Command,
        data: Option<&[u8]>,
    ) -> Result<(), Error<B::Error>> {
        self.send_raw_command_unchecked(port, cmd, data).await?;

        let mut registers = self.borrow_port(port)?.into_registers();
        let status = registers.cmd_1().read_async().await?.command();

        if Command::Invalid == status {
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
            if data.len() > REG_DATA1_LEN - 1 {
                // Data length too long
                return PdError::InvalidParams.into();
            }
        }

        // Read and return value and data
        let mut buf = [0u8; REG_DATA1_LEN];
        self.borrow_port(port)?
            .into_registers()
            .interface()
            .read_register(REG_DATA1, (REG_DATA1_LEN * 8) as u32, &mut buf)
            .await?;

        let ret = ReturnValue::try_from(buf[0]).map_err(Error::Pd)?;

        // Overwrite return value
        if let Some(data) = data {
            data.copy_from_slice(&buf[1..=data.len()]);
        }

        Ok(ret)
    }

    /// Reset the controller
    // This command doesn't trigger an interrupt on completion so it fits here better
    pub async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        // This is a controller-level command, shouldn't matter which port we use
        let port = PortId(0);
        self.send_raw_command_unchecked(port, Command::Gaid, None).await?;

        delay.delay_ms(RESET_DELAY_MS).await;

        // Command register should be set to success value
        if !self.check_command_complete(port).await? {
            return PdError::Busy.into();
        }

        self.clear_interrupt(PortId(0)).await?;
        self.clear_interrupt(PortId(1)).await?;

        Ok(())
    }
}
