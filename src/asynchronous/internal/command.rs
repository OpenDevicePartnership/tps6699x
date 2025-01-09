//! Support for 4CC commands which communicate through the CMD1 and DATA1 registers
//! The DATA1 register exceeds the 128 bit limit of the device driver crate so we have to handle it manually
use super::*;
use device_driver::AsyncRegisterInterface;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::Error;

use crate::command::{Command, ReturnValue};

const REG_DATA1: u8 = 0x09;
// Register is 512 bits
const REG_DATA1_LEN: usize = 64;
const CMD_SUCCESS: u32 = 0;
// '!CMD'
const CMD_UNKNOWN: u32 = 0x444E4321;

// These are controller-level commands, we use borrow_port just for convenience
impl<B: I2c> Tps6699x<B> {
    pub async fn send_command(
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
            data.copy_from_slice(&buf[1..data.len()]);
        }

        Ok(ret)
    }
}
