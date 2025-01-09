#![no_std]

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
