//! Support for 4CC commands which communicate through the CMD1 and DATA1 registers
//! The DATA1 register exceeds the 128 bit limit of the device driver crate so we have to handle it manually
use device_driver::AsyncRegisterInterface;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::Error;

use super::*;
use crate::command::{Command, Operation, ReturnValue, CMD_SUCCESS, CMD_UNKNOWN, REG_DATA1, REG_DATA1_LEN};

// These are controller-level commands, we use borrow_port just for convenience
impl<B: I2c> Tps6699x<B> {
    pub async fn send_raw_command(
        &mut self,
        port: PortId,
        cmd: Operation,
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

    async fn send_reset_command(&mut self, port: PortId) -> Result<(), Error<B::Error>> {
        // Arguments are two bytes that control FW bank selection, currently unused
        let args = [0u8; 2];
        self.send_raw_command(port, Operation::Gaid, Some(&args)).await
    }

    pub async fn send_command(&mut self, port: PortId, cmd: Command) -> Result<(), Error<B::Error>> {
        match cmd {
            Command::Reset => self.send_reset_command(port).await,
        }
    }

    pub async fn read_command_result(
        &mut self,
        port: PortId,
        data: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        let mut registers = self.borrow_port(port)?.into_registers();

        let status = registers.cmd_1().read_async().await?.command();
        if status == CMD_UNKNOWN {
            return PdError::UnrecognizedCommand.into();
        } else if status != CMD_SUCCESS {
            // Command has not completed
            return PdError::InProgress.into();
        }

        let mut buf = [0u8; REG_DATA1_LEN];

        if let Some(ref data) = data {
            if data.len() > REG_DATA1_LEN - 1 {
                // Data length too long
                return PdError::InvalidParams.into();
            }
        }

        // First byte is return value
        let read_len = match data {
            Some(ref data) => data.len() + 1,
            None => 1,
        };

        registers
            .interface()
            .read_register(REG_DATA1, (read_len * 8) as u32, &mut buf[..read_len])
            .await?;

        let ret = ReturnValue::try_from(buf[0]).map_err(Error::Pd)?;

        // Overwrite return value
        if let Some(data) = data {
            data.copy_from_slice(&buf[1..=data.len()]);
        }

        Ok(ret)
    }
}
