#![no_std]

pub mod asynchronous;

/// I2C address set 0
pub const ADDR0: [u8; 2] = [0x20, 0x24];
/// I2C address set 1
pub const ADDR1: [u8; 2] = [0x21, 0x25];
