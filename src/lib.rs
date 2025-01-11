#![no_std]

use embedded_usb_pd::PdError;

pub mod asynchronous;
pub mod command;

/// I2C address set 0
pub const ADDR0: [u8; 2] = [0x20, 0x24];
/// I2C address set 1
pub const ADDR1: [u8; 2] = [0x21, 0x25];

pub mod registers {
    use device_driver;
    device_driver::create_device!(
        device_name: Registers,
        manifest: "device.yaml"
    );
}

/// Converts a 4-byte string into a u32
const fn u32_from_str(value: &str) -> u32 {
    if value.len() != command::CMD_LEN {
        panic!("Invalid command string")
    }

    let bytes = value.as_bytes();
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]).to_le()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Boot mode
    Boot = u32_from_str("BOOT"),
    /// Firmware corrupt on both banks
    F211 = u32_from_str("F211"),
    /// Before app config
    App0 = u32_from_str("APP0"),
    /// After app config
    App1 = u32_from_str("APP1"),
    /// App FW waiting for power
    Wtpr = u32_from_str("WTPR"),
}

impl PartialEq<u32> for Mode {
    fn eq(&self, other: &u32) -> bool {
        *self as u32 == *other
    }
}

impl TryFrom<u32> for Mode {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if Mode::Boot == value {
            Ok(Mode::Boot)
        } else if Mode::F211 == value {
            Ok(Mode::F211)
        } else if Mode::App0 == value {
            Ok(Mode::App0)
        } else if Mode::App1 == value {
            Ok(Mode::App1)
        } else if Mode::Wtpr == value {
            Ok(Mode::Wtpr)
        } else {
            Err(PdError::InvalidParams)
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<[u8; 4]> for Mode {
    fn into(self) -> [u8; 4] {
        (self as u32).to_le_bytes()
    }
}
