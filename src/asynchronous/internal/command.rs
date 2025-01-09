//! Support for 4CC commands which communicate through the CMD1 and DATA1 registers
//! The DATA1 register exceeds the 128 bit limit of the device driver crate so we have to handle it manually
use super::*;
use device_driver::AsyncRegisterInterface;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Command {
    /// Cold reset
    Gaid = 0x01,
}

#[repr(u8)]
pub enum ReturnValue {
    /// Success
    Success = 0x00,
    /// Timed-out or aborted with ABRT command
    Abort = 0x01,
    /// Rejected
    Rejected = 0x03,
    /// RX buffer locked
    RxLocked = 0x04,
    /// Task specific result
    Task0 = 0x05,
    /// Task specific result
    Task1 = 0x06,
    /// Task specific result
    Task2 = 0x07,
    /// Task specific result
    Task3 = 0x08,
    /// Task specific result
    Task4 = 0x09,
    /// Task specific result
    Task5 = 0x0A,
    /// Task specific result
    Task6 = 0x0B,
    /// Task specific result
    Task7 = 0x0C,
    /// Task specific result
    Task8 = 0x0D,
    /// Task specific result
    Task9 = 0x0E,
    /// Task specific result
    Task10 = 0x0F,
}

impl TryFrom<u8> for ReturnValue {
    type Error = PdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ReturnValue::Success),
            0x01 => Ok(ReturnValue::Abort),
            0x03 => Ok(ReturnValue::Rejected),
            0x04 => Ok(ReturnValue::RxLocked),
            0x05 => Ok(ReturnValue::Task0),
            0x06 => Ok(ReturnValue::Task1),
            0x07 => Ok(ReturnValue::Task2),
            0x08 => Ok(ReturnValue::Task3),
            0x09 => Ok(ReturnValue::Task4),
            0x0A => Ok(ReturnValue::Task5),
            0x0B => Ok(ReturnValue::Task6),
            0x0C => Ok(ReturnValue::Task7),
            0x0D => Ok(ReturnValue::Task8),
            0x0E => Ok(ReturnValue::Task9),
            0x0F => Ok(ReturnValue::Task10),
            _ => Err(PdError::InvalidParams),
        }
    }
}

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
