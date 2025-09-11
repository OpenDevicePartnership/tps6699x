#![no_std]

use embedded_usb_pd::{LocalPortId, PdError};

pub mod asynchronous;
pub mod command;
pub mod fmt;
pub mod fw_update;
pub mod registers;
pub mod stream;

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
pub const PORT0: LocalPortId = LocalPortId(0);
/// Port 1 constant
pub const PORT1: LocalPortId = LocalPortId(1);

/// Device error type to wrap common error with custom error type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceError<BE, T> {
    /// Standard USB PD error
    Error(embedded_usb_pd::Error<BE>),
    /// Other error
    Other(T),
}

impl<BE, T> From<embedded_usb_pd::Error<BE>> for DeviceError<BE, T> {
    fn from(value: embedded_usb_pd::Error<BE>) -> Self {
        DeviceError::Error(value)
    }
}

impl<BE, T> From<DeviceError<BE, T>> for embedded_usb_pd::Error<BE> {
    fn from(value: DeviceError<BE, T>) -> Self {
        match value {
            DeviceError::Error(e) => e,
            DeviceError::Other(_) => embedded_usb_pd::Error::Pd(PdError::Failed),
        }
    }
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

    use bincode::encode_into_slice;
    use embedded_hal_async::delay::DelayNs;
    use embedded_hal_mock::eh1::i2c::Transaction;
    use embedded_usb_pd::pdo::source::FixedFlags;
    use embedded_usb_pd::pdo::{self, MA10_UNIT, MV50_UNIT};
    use tokio::time::sleep;

    use super::*;
    use crate::command::{TfudArgs, TfuiArgs, TFUD_ARGS_LEN};
    use crate::fw_update::{APP_IMAGE_SIZE_OFFSET, HEADER_BLOCK_LEN, HEADER_METADATA_OFFSET};

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

    /// Mock default data block size
    pub const MOCK_DEFAULT_DATA_BLOCK_SIZE: u16 = 0x4000;
    /// Mock last data block size
    pub const MOCK_LAST_DATA_BLOCK_SIZE: u16 = 0x1800;
    /// Mock app config size
    pub const MOCK_APP_CONFIG_SIZE: u16 = 0x800;
    /// Mock default timeout for this block in seconds
    pub const MOCK_TIMEOUT_SECS: u16 = 255;
    /// Mock broadcast address
    pub const MOCK_BROADCAST_ADDR: u16 = 0x77;
    /// Mock overall update timeout
    pub const MOCK_UPDATE_TIMEOUT: u16 = 3000;
    /// Mock default data block count
    pub const MOCK_DEFAULT_DATA_BLOCK_COUNT: u16 = 11;
    /// Mock FW update app size
    pub const MOCK_APP_SIZE: u32 = 0x29800;

    /// Helper function to generate block header arguments
    const fn mock_data_block_header(block_number: u16) -> TfudArgs {
        TfudArgs {
            block_number,
            data_len: MOCK_DEFAULT_DATA_BLOCK_SIZE,
            timeout_secs: MOCK_TIMEOUT_SECS,
            broadcast_u16_address: MOCK_BROADCAST_ADDR,
        }
    }

    /// Mock update header
    pub const MOCK_UPDATE_HEADER: TfuiArgs = TfuiArgs {
        num_data_blocks_tx: MOCK_DEFAULT_DATA_BLOCK_COUNT,
        data_len: HEADER_BLOCK_LEN as u16,
        timeout_secs: MOCK_UPDATE_TIMEOUT,
        broadcast_u16_address: MOCK_BROADCAST_ADDR,
    };

    // Data block indexes start at block index 1
    /// Mock data block header 0
    pub const MOCK_DATA_BLOCK_HEADER_0: TfudArgs = mock_data_block_header(1);
    /// Mock data block header 0 offset
    pub const MOCK_DATA_BLOCK_HEADER_0_OFFSET: usize = 0x80c;

    /// Mock data block header 1
    pub const MOCK_DATA_BLOCK_HEADER_1: TfudArgs = mock_data_block_header(2);
    /// Mock data block header 1 offset
    pub const MOCK_DATA_BLOCK_HEADER_1_OFFSET: usize = 0x4814;

    /// Mock data block header 2
    pub const MOCK_DATA_BLOCK_HEADER_2: TfudArgs = mock_data_block_header(3);
    /// Mock data block header 2 offset
    pub const MOCK_DATA_BLOCK_HEADER_2_OFFSET: usize = 0x881c;

    /// Mock data block header 3
    pub const MOCK_DATA_BLOCK_HEADER_3: TfudArgs = mock_data_block_header(4);
    /// Mock data block header 3 offset
    pub const MOCK_DATA_BLOCK_HEADER_3_OFFSET: usize = 0xc824;

    /// Mock data block header 4
    pub const MOCK_DATA_BLOCK_HEADER_4: TfudArgs = mock_data_block_header(5);
    /// Mock data block header 4 offset
    pub const MOCK_DATA_BLOCK_HEADER_4_OFFSET: usize = 0x1082c;

    /// Mock data block header 5
    pub const MOCK_DATA_BLOCK_HEADER_5: TfudArgs = mock_data_block_header(6);
    /// Mock data block header 5 offset
    pub const MOCK_DATA_BLOCK_HEADER_5_OFFSET: usize = 0x14834;

    /// Mock data block header 6
    pub const MOCK_DATA_BLOCK_HEADER_6: TfudArgs = mock_data_block_header(7);
    /// Mock data block header 6 offset
    pub const MOCK_DATA_BLOCK_HEADER_6_OFFSET: usize = 0x1883c;

    /// Mock data block header 7
    pub const MOCK_DATA_BLOCK_HEADER_7: TfudArgs = mock_data_block_header(8);
    /// Mock data block header 7 offset
    pub const MOCK_DATA_BLOCK_HEADER_7_OFFSET: usize = 0x1c844;

    /// Mock data block header 8
    pub const MOCK_DATA_BLOCK_HEADER_8: TfudArgs = mock_data_block_header(9);
    /// Mock data block header 8 offset
    pub const MOCK_DATA_BLOCK_HEADER_8_OFFSET: usize = 0x2084c;

    /// Mock data block header 9
    pub const MOCK_DATA_BLOCK_HEADER_9: TfudArgs = mock_data_block_header(10);
    /// Mock data block header 9 offset
    pub const MOCK_DATA_BLOCK_HEADER_9_OFFSET: usize = 0x24854;

    /// Mock data block header 10
    pub const MOCK_DATA_BLOCK_HEADER_10: TfudArgs = TfudArgs {
        block_number: 11,
        data_len: MOCK_LAST_DATA_BLOCK_SIZE,
        timeout_secs: MOCK_TIMEOUT_SECS,
        broadcast_u16_address: MOCK_BROADCAST_ADDR,
    };
    /// Mock data block header 10 offset
    pub const MOCK_DATA_BLOCK_HEADER_10_OFFSET: usize = 0x2885c;

    /// Mock app config header
    pub const APP_CONFIG_HEADER: TfudArgs = TfudArgs {
        block_number: 12,
        data_len: MOCK_APP_CONFIG_SIZE,
        timeout_secs: MOCK_TIMEOUT_SECS,
        broadcast_u16_address: MOCK_BROADCAST_ADDR,
    };
    /// Mock app config header offset
    pub const APP_CONFIG_HEADER_OFFSET: usize = 0x2a064;

    /// Generates a mock FW update that contains no actual data, just headers
    pub fn generate_mock_fw() -> Vec<u8> {
        let mut buf = [0u8; TFUD_ARGS_LEN];
        let mut data: Vec<(usize, Vec<u8>)> = Vec::new();
        let config = bincode::config::standard().with_fixed_int_encoding();

        // Encode the update header
        encode_into_slice(MOCK_UPDATE_HEADER, &mut buf, config).unwrap();
        data.push((HEADER_METADATA_OFFSET, buf.clone().into()));

        // Encode the app size
        data.push((APP_IMAGE_SIZE_OFFSET, MOCK_APP_SIZE.to_le_bytes().into()));

        // Encode the data block headers
        let headers = [
            (MOCK_DATA_BLOCK_HEADER_0_OFFSET, MOCK_DATA_BLOCK_HEADER_0),
            (MOCK_DATA_BLOCK_HEADER_1_OFFSET, MOCK_DATA_BLOCK_HEADER_1),
            (MOCK_DATA_BLOCK_HEADER_2_OFFSET, MOCK_DATA_BLOCK_HEADER_2),
            (MOCK_DATA_BLOCK_HEADER_3_OFFSET, MOCK_DATA_BLOCK_HEADER_3),
            (MOCK_DATA_BLOCK_HEADER_4_OFFSET, MOCK_DATA_BLOCK_HEADER_4),
            (MOCK_DATA_BLOCK_HEADER_5_OFFSET, MOCK_DATA_BLOCK_HEADER_5),
            (MOCK_DATA_BLOCK_HEADER_6_OFFSET, MOCK_DATA_BLOCK_HEADER_6),
            (MOCK_DATA_BLOCK_HEADER_7_OFFSET, MOCK_DATA_BLOCK_HEADER_7),
            (MOCK_DATA_BLOCK_HEADER_8_OFFSET, MOCK_DATA_BLOCK_HEADER_8),
            (MOCK_DATA_BLOCK_HEADER_9_OFFSET, MOCK_DATA_BLOCK_HEADER_9),
            (MOCK_DATA_BLOCK_HEADER_10_OFFSET, MOCK_DATA_BLOCK_HEADER_10),
        ];

        for (offset, header) in headers.iter() {
            encode_into_slice(header, &mut buf, config).unwrap();
            data.push((*offset, buf.clone().into()));
        }

        // Encode the app config header
        encode_into_slice(APP_CONFIG_HEADER, &mut buf, config).unwrap();
        data.push((APP_CONFIG_HEADER_OFFSET, buf.clone().into()));

        let mut buffer = vec![0; 171 * 1024];
        for (offset, bytes) in data {
            buffer[offset as usize..offset as usize + bytes.len()].copy_from_slice(&bytes);
        }

        buffer
    }

    // The current USB PD code is focused on decoding PDOs, not creating them
    // TODO: https://github.com/OpenDevicePartnership/embedded-usb-pd/issues/30
    /// Create a test source PDO fixed raw value
    pub const fn test_src_pdo_fixed_raw(voltage_mv: u16, current_ma: u16) -> u32 {
        (((voltage_mv / MV50_UNIT) as u32) << 10) | (current_ma / MA10_UNIT) as u32
    }

    pub const fn test_src_pdo_fixed_flags() -> FixedFlags {
        FixedFlags {
            dual_role_power: false,
            usb_suspend_supported: false,
            unconstrained_power: false,
            usb_comms_capable: false,
            dual_role_data: false,
            unchunked_extended_messages_support: false,
            epr_capable: false,
        }
    }

    /// Test source PDO fixed 5V 3A raw
    pub const TEST_SRC_PDO_FIXED_5V3A_RAW: u32 = test_src_pdo_fixed_raw(5000, 3000);
    /// Test source PDO fixed 5V 3A
    pub const TEST_SRC_PDO_FIXED_5V3A: pdo::source::Pdo = pdo::source::Pdo::Fixed(pdo::source::FixedData {
        flags: test_src_pdo_fixed_flags(),
        voltage_mv: 5000,
        current_ma: 3000,
        peak_current: pdo::source::PeakCurrent::Pct100,
    });

    /// Test source PDO fixed 5V 1.5A raw
    pub const TEST_SRC_PDO_FIXED_5V1A5_RAW: u32 = test_src_pdo_fixed_raw(5000, 1500);
    /// Test source PDO fixed 5V 1.5A
    pub const TEST_SRC_PDO_FIXED_5V1A5: pdo::source::Pdo = pdo::source::Pdo::Fixed(pdo::source::FixedData {
        flags: test_src_pdo_fixed_flags(),
        voltage_mv: 5000,
        current_ma: 1500,
        peak_current: pdo::source::PeakCurrent::Pct100,
    });

    /// Test source PDO fixed 5V 900mA raw
    pub const TEST_SRC_PDO_FIXED_5V900MA_RAW: u32 = test_src_pdo_fixed_raw(5000, 900);

    /// Test source EPR PDO fixed 28V 5A raw
    pub const TEST_SRC_EPR_PDO_FIXED_28V5A_RAW: u32 = test_src_pdo_fixed_raw(28000, 5000);
    /// Test source EPR PDO fixed 28V 5A
    pub const TEST_SRC_EPR_PDO_FIXED_28V5A: pdo::source::Pdo = pdo::source::Pdo::Fixed(pdo::source::FixedData {
        flags: test_src_pdo_fixed_flags(),
        voltage_mv: 28000,
        current_ma: 5000,
        peak_current: pdo::source::PeakCurrent::Pct100,
    });

    /// Test source EPR PDO fixed 28V 3A raw
    pub const TEST_SRC_EPR_PDO_FIXED_28V3A_RAW: u32 = test_src_pdo_fixed_raw(28000, 3000);

    /// Test source EPR PDO fixed 28V 1.5A raw
    pub const TEST_SRC_EPR_PDO_FIXED_28V1A5_RAW: u32 = test_src_pdo_fixed_raw(28000, 1500);

    /// Test invalid source APDO raw, invalid due to APDO type being 11b
    pub const TEST_SRC_APDO_INVALID_RAW: u32 = 0xF0000000;
}
