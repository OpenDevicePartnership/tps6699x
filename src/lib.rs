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

/// Converts a 4-byte string into a u32
const fn u32_from_str(value: &str) -> u32 {
    if value.len() != command::CMD_LEN {
        panic!("Invalid command string")
    }

    let bytes = value.as_bytes();
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]).to_le()
}
