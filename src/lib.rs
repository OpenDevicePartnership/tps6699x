#![no_std]

use embedded_usb_pd::{PdError, PortId};

pub mod asynchronous;
pub mod command;
pub mod fmt;
pub mod registers;
pub mod stream;

pub(crate) mod fw_update;

/// I2C address set 0
pub const ADDR0: [u8; 2] = [0x20, 0x24];
/// I2C address set 1
pub const ADDR1: [u8; 2] = [0x21, 0x25];

/// Number of ports present on the TPS66994
pub const TPS66994_NUM_PORTS: usize = 2;
/// Number of ports present on the TPS66993
pub const TPS66993_NUM_PORTS: usize = 1;
/// Maximum number of ports supported by any device
pub const MAX_SUPPORTED_PORTS: usize = 2;

/// Port 0 constant
pub const PORT0: PortId = PortId(0);
/// Port 1 constant
pub const PORT1: PortId = PortId(1);

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

const U32_STR_LEN: usize = 4;
/// Converts a 4-byte string into a u32
pub(crate) const fn u32_from_str(value: &str) -> u32 {
    if value.len() != U32_STR_LEN {
        panic!("Invalid command string")
    }
    let bytes = value.as_bytes();
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]).to_le()
}

/// Common unit test functions
#[cfg(test)]
pub(crate) mod test {
    extern crate std;
    use core::time::Duration;
    use std::vec;
    use std::vec::Vec;

    use embedded_hal_async::delay::DelayNs;
    use embedded_hal_mock::eh1::i2c::Transaction;
    use tokio::time::sleep;

    use super::*;

    pub const PORT0_ADDR0: u8 = ADDR0[0];
    pub const PORT1_ADDR0: u8 = ADDR0[1];
    pub const PORT0_ADDR1: u8 = ADDR1[0];
    pub const PORT1_ADDR1: u8 = ADDR1[1];

    /// Wrapper to easily create a register read transaction
    pub fn create_register_read<const N: usize, R: Into<[u8; N]>>(addr: u8, reg: u8, value: R) -> Transaction {
        // +1 for the length byte
        let mut response = Vec::with_capacity(N + 1);
        response.push(N as u8);
        response.splice(1..1, value.into().iter().cloned());

        Transaction::write_read(addr, vec![reg], response)
    }

    /// Wrapper to easily create a register write transaction
    pub fn create_register_write<const N: usize, R: Into<[u8; N]>>(addr: u8, reg: u8, value: R) -> Transaction {
        // +1 for the register + length byte
        let mut response = Vec::with_capacity(N + 2);
        response.push(reg);
        response.push(N as u8);
        response.splice(2..2, value.into().iter().cloned());

        Transaction::write(addr, response)
    }

    pub struct Delay {}
    impl DelayNs for Delay {
        async fn delay_ns(&mut self, ns: u32) {
            sleep(Duration::from_nanos(ns as u64)).await;
        }
    }
}
