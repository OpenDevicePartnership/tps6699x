use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use embedded_usb_pd::PdError;

pub mod muxr;
pub mod trig;

/// Length of a command
const CMD_LEN: usize = 4;

/// TaskResult is only defined for lower 4 bits
pub const CMD_4CC_TASK_RETURN_CODE_MASK: u8 = 0x0F;

/// Converts a 4-byte string into a u32
const fn u32_from_str(value: &str) -> u32 {
    if value.len() != CMD_LEN {
        panic!("Invalid command string")
    }

    let bytes = value.as_bytes();
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]).to_le()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Command {
    /// Previous command succeeded
    Success = 0,
    /// Invalid Command
    Invalid = u32_from_str("!CMD"),
    /// Reset command
    Gaid = u32_from_str("GAID"),

    /// Tomcat firmware update mode enter
    Tfus = u32_from_str("TFUs"),
    /// Tomcat firmware update mode init
    Tfui = u32_from_str("TFUi"),
    /// Tomcat firmware update mode query
    Tfuq = u32_from_str("TFUq"),
    /// Tomcat firmware update mode exit
    Tfue = u32_from_str("TFUe"),
    /// Tomcat firmware update data
    Tfud = u32_from_str("TFUd"),
    /// Tomcat firmware update complete
    Tfuc = u32_from_str("TFUc"),

    /// System ready to sink
    Srdy = u32_from_str("SRDY"),
    /// SRDY reset
    Sryr = u32_from_str("SRYR"),

    /// Re-evaluate the Autonegotiate Sink register.
    ///
    /// # Input
    /// None.
    ///
    /// # Output
    /// [`ReturnValue`]
    Aneg = u32_from_str("ANeg"),

    /// Trigger an Input GPIO event
    Trig = u32_from_str("Trig"),

    /// Clear the dead battery flag.
    ///
    /// # Input
    /// None.
    ///
    /// # Output
    /// [`ReturnValue`]
    Dbfg = u32_from_str("DBfg"),

    /// Repeat transactions on I2C3m under certain conditions.
    ///
    /// # Input
    /// [`muxr::Input`]
    ///
    /// # Output
    /// [`ReturnValue`]
    Muxr = u32_from_str("MuxR"),

    /// PD Data Reset
    ///
    /// # Input
    /// None.
    ///
    /// # Output
    /// [`ReturnValue`]
    Drst = u32_from_str("DRST"),
}

impl TryFrom<u32> for Command {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if Command::Success == value {
            Ok(Command::Success)
        } else if Command::Invalid == value {
            Ok(Command::Invalid)
        } else if Command::Gaid == value {
            Ok(Command::Gaid)
        } else if Command::Tfus == value {
            Ok(Command::Tfus)
        } else if Command::Tfui == value {
            Ok(Command::Tfui)
        } else if Command::Tfuq == value {
            Ok(Command::Tfuq)
        } else if Command::Tfue == value {
            Ok(Command::Tfue)
        } else if Command::Tfud == value {
            Ok(Command::Tfud)
        } else if Command::Tfuc == value {
            Ok(Command::Tfuc)
        } else if Command::Srdy == value {
            Ok(Command::Srdy)
        } else if Command::Sryr == value {
            Ok(Command::Sryr)
        } else if Command::Aneg == value {
            Ok(Command::Aneg)
        } else if Command::Trig == value {
            Ok(Command::Trig)
        } else if Command::Dbfg == value {
            Ok(Command::Dbfg)
        } else if Command::Muxr == value {
            Ok(Command::Muxr)
        } else if Command::Drst == value {
            Ok(Command::Drst)
        } else {
            Err(PdError::InvalidParams)
        }
    }
}

impl Command {
    /// Returns the delay in microseconds before checking that the command was valid
    pub fn valid_check_delay_us(self) -> u32 {
        match self {
            // Reset-type commands don't need to be checked
            Command::Success | Command::Invalid | Command::Gaid | Command::Tfus => 0,
            Command::Tfuc => 5000,
            Command::Tfui => 1500,
            Command::Tfuq | Command::Tfue => 200,
            Command::Tfud => 7000,
            _ => 1000,
        }
    }

    /// Returns the timeout in milliseconds for the command to complete.
    pub const fn timeout_ms(self) -> u32 {
        match self {
            Command::Tfus => TFUS_DELAY_MS + 100,
            Command::Tfui | Command::Tfue | Command::Tfud | Command::Tfuq => 200, // docs say 100ms, but 200ms is more reliable
            Command::Gaid | Command::Tfuc => RESET_DELAY_MS + 100,
            Command::Srdy | Command::Sryr => 250, // determined by experimentation
            Command::Trig => 500,                 // determined by experimentation
            Command::Drst => 100,                 // PD spec says 24/27/30 ms, round up
            _ => 1000,
        }
    }

    /// The timeout for a command. This is [`Self::timeout_ms`] converted to an [`embassy_time::Duration`].
    #[cfg(feature = "embassy")]
    pub const fn timeout(self) -> embassy_time::Duration {
        embassy_time::Duration::from_millis(self.timeout_ms() as u64)
    }
}

impl PartialEq<u32> for Command {
    fn eq(&self, other: &u32) -> bool {
        *self as u32 == *other
    }
}

/// A status code, often in the first byte of the `DATAX` register after a command is executed.
///
/// [`ReturnValue::Task0`] through [`ReturnValue::Task10`] are reserved for standard tasks and may be used by certain
/// tasks for task-specific error codes. These should be treated as an error when encountered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ReturnValue {
    /// Task completed successfully.
    Success = 0x00,

    /// Task timed-out or aborted with `ABRT` command.
    Abort = 0x01,

    /// Rejected
    Rejected = 0x03,

    /// Task rejected because the Rx Buffer was locked. This is for tasks that can require the PD controller to use the
    /// Rx Buffer.
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

impl ReturnValue {
    /// Returns `Ok(())` if the return value is `Success`, otherwise returns `error`.
    pub fn success_or(self, error: PdError) -> Result<(), PdError> {
        match self {
            ReturnValue::Success => Ok(()),
            _ => Err(error),
        }
    }
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

#[allow(clippy::from_over_into)]
impl Into<Result<(), PdError>> for ReturnValue {
    fn into(self) -> Result<(), PdError> {
        match self {
            ReturnValue::Success => Ok(()),
            _ => Err(PdError::Failed),
        }
    }
}

/// Delay to wait for the device to restart
pub(crate) const RESET_DELAY_MS: u32 = 1600;
/// Length of arguments for the reset command
pub(crate) const RESET_ARGS_LEN: usize = 2;
/// Constant to enable a feature in the command args
pub(crate) const RESET_FEATURE_ENABLE: u8 = 0xAC;

/// Arugments to reset-like commands
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResetArgs {
    /// True to swap banks on reset
    pub switch_banks: bool,
    /// True to copy the backup bank to the active bank
    pub copy_bank: bool,
}
impl Encode for ResetArgs {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let switch_banks = if self.switch_banks { RESET_FEATURE_ENABLE } else { 0 };
        Encode::encode(&switch_banks, encoder)?;
        let copy_bank = if self.copy_bank { RESET_FEATURE_ENABLE } else { 0 };
        Encode::encode(&copy_bank, encoder)
    }
}

/// Delay for completion of TFUs command
pub(crate) const TFUS_DELAY_MS: u32 = 500;
/// Length of TFUi arguments
#[allow(dead_code)]
pub(crate) const TFUI_ARGS_LEN: usize = 8;

/// Arguments for TFUi command
#[derive(Debug, Clone, Copy, Decode, Encode, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfuiArgs {
    pub num_data_blocks_tx: u16,
    pub data_len: u16,
    pub timeout_secs: u16,
    pub broadcast_u16_address: u16,
}

/// Command type for TFUq command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TfuqCommandType {
    QueryTfuStatus = 0x00,
}

impl Encode for TfuqCommandType {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let val = *self as u8;
        Encode::encode(&val, encoder)
    }
}

/// Status we're checking for in the TFUq command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TfuqStatusQuery {
    StatusDefault = 0x00,
    StatusInProgress,
    StatusBank0,
    StatusBank1,
}

impl Encode for TfuqStatusQuery {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let val = *self as u8;
        Encode::encode(&val, encoder)
    }
}

/// Status of a block supplied to device
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TfuqBlockStatus {
    Success = 0x0,
    InvalidTfuState,
    InvalidHeaderSize,
    InvalidDataBlock,
    InvalidDataSize,
    InvalidSlaveAddress,
    InvalidTimeout,
    MaxAppConfigUpdate,
    HeaderRxInProgress,
    HeaderValidAndAuthentic,
    HeaderNotValid,
    HeaderKeyNotValid,
    HeaderRootAuthFailure,
    HeaderFwheaderAuthFailure,
    DataRxInProgress,
    DataValidAndAuthentic,
    DataValidButRepeated,
    DataNotValid,
    DataInvalidId,
    DataAuthFailure,
    F911IdNotValid,
    F911DataNotValid,
    F911AuthFailure,
    ImageDownloadTimeout,
    BlockDownloadTimeout,
    BlockWriteFailed,
    SpecialCmdFailed,
}

impl TryFrom<u8> for TfuqBlockStatus {
    type Error = PdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(TfuqBlockStatus::Success),
            0x1 => Ok(TfuqBlockStatus::InvalidTfuState),
            0x2 => Ok(TfuqBlockStatus::InvalidHeaderSize),
            0x3 => Ok(TfuqBlockStatus::InvalidDataBlock),
            0x4 => Ok(TfuqBlockStatus::InvalidDataSize),
            0x5 => Ok(TfuqBlockStatus::InvalidSlaveAddress),
            0x6 => Ok(TfuqBlockStatus::InvalidTimeout),
            0x7 => Ok(TfuqBlockStatus::MaxAppConfigUpdate),
            0x8 => Ok(TfuqBlockStatus::HeaderRxInProgress),
            0x9 => Ok(TfuqBlockStatus::HeaderValidAndAuthentic),
            0xA => Ok(TfuqBlockStatus::HeaderNotValid),
            0xB => Ok(TfuqBlockStatus::HeaderKeyNotValid),
            0xC => Ok(TfuqBlockStatus::HeaderRootAuthFailure),
            0xD => Ok(TfuqBlockStatus::HeaderFwheaderAuthFailure),
            0xE => Ok(TfuqBlockStatus::DataRxInProgress),
            0xF => Ok(TfuqBlockStatus::DataValidAndAuthentic),
            0x10 => Ok(TfuqBlockStatus::DataValidButRepeated),
            0x11 => Ok(TfuqBlockStatus::DataNotValid),
            0x12 => Ok(TfuqBlockStatus::DataInvalidId),
            0x13 => Ok(TfuqBlockStatus::DataAuthFailure),
            0x14 => Ok(TfuqBlockStatus::F911IdNotValid),
            0x15 => Ok(TfuqBlockStatus::F911DataNotValid),
            0x16 => Ok(TfuqBlockStatus::F911AuthFailure),
            0x17 => Ok(TfuqBlockStatus::ImageDownloadTimeout),
            0x18 => Ok(TfuqBlockStatus::BlockDownloadTimeout),
            0x19 => Ok(TfuqBlockStatus::BlockWriteFailed),
            0x1A => Ok(TfuqBlockStatus::SpecialCmdFailed),
            _ => Err(PdError::InvalidParams),
        }
    }
}

impl<Context> Decode<Context> for TfuqBlockStatus {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let val: u8 = Decode::decode(decoder)?;
        TfuqBlockStatus::try_from(val).map_err(|_| DecodeError::Other("Invalid TfuqBlockStatus"))
    }
}

/// Length of TFUq args
#[allow(dead_code)]
pub(crate) const TFUQ_ARGS_LEN: usize = 2;

/// Arguments for TFUq command
#[derive(Debug, Encode, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfuqArgs {
    pub status_query: TfuqStatusQuery,
    pub command: TfuqCommandType,
}

/// Length of return data from TFUq command
#[allow(dead_code)]
pub(crate) const TFUQ_RETURN_LEN: usize = 40;
/// Number of block statuses present in TFUq return data
#[allow(dead_code)]
pub(crate) const TFUQ_RETURN_BLOCK_STATUS_LEN: usize = 13;

/// Return data from TFUq command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfuqReturnValue {
    pub active_host: u8,
    pub current_state: u8,
    pub image_write_status: u8,
    pub blocks_written_bitfield: u16,
    pub block_status: [TfuqBlockStatus; TFUQ_RETURN_BLOCK_STATUS_LEN],
    pub num_of_header_bytes_received: u32,
    pub num_of_data_bytes_received: u32,
    pub num_of_app_config_updates: u16,
}

impl<Context> Decode<Context> for TfuqReturnValue {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let active_host = Decode::decode(decoder)?;
        let current_state = Decode::decode(decoder)?;
        let _reserved: u16 = Decode::decode(decoder)?;
        let image_write_status = Decode::decode(decoder)?;
        let blocks_written_bitfield = Decode::decode(decoder)?;
        let mut block_status = [TfuqBlockStatus::Success; TFUQ_RETURN_BLOCK_STATUS_LEN];
        for status in block_status.iter_mut() {
            *status = Decode::decode(decoder)?;
        }
        let num_of_header_bytes_received = Decode::decode(decoder)?;
        let _reserved: u16 = Decode::decode(decoder)?;
        let num_of_data_bytes_received = Decode::decode(decoder)?;
        let _reserved: u16 = Decode::decode(decoder)?;
        let num_of_app_config_updates = Decode::decode(decoder)?;

        Ok(TfuqReturnValue {
            active_host,
            current_state,
            image_write_status,
            blocks_written_bitfield,
            block_status,
            num_of_header_bytes_received,
            num_of_data_bytes_received,
            num_of_app_config_updates,
        })
    }
}

/// Srdy switch to enable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrdySwitch {
    /// PP 5V1
    Pp5V1,
    /// PP 5V2
    Pp5V2,
    /// PP Ext 1
    PpExt1,
    /// PP Ext 2
    PpExt2,
    /// Automatically based on global config register
    AutoConfig,
    /// Automatically based on PD controller policy
    AutoPolicy,
}

impl From<SrdySwitch> for u8 {
    fn from(value: SrdySwitch) -> Self {
        match value {
            SrdySwitch::Pp5V1 => 0x0,
            SrdySwitch::Pp5V2 => 0x1,
            SrdySwitch::PpExt1 => 0x2,
            SrdySwitch::PpExt2 => 0x3,
            SrdySwitch::AutoConfig => 0x6,
            SrdySwitch::AutoPolicy => 0x7,
        }
    }
}

/// Arguments for TFUd command
#[allow(dead_code)]
pub(crate) const TFUD_ARGS_LEN: usize = 8;
#[derive(Debug, Decode, Encode, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfudArgs {
    pub block_number: u16,
    pub data_len: u16,
    pub timeout_secs: u16,
    pub broadcast_u16_address: u16,
}
#[cfg(test)]
mod test {
    use bincode::config;

    use super::*;

    fn test_encode_reset_args(args: ResetArgs, expected: [u8; RESET_ARGS_LEN]) {
        let mut buf = [0; RESET_ARGS_LEN];
        bincode::encode_into_slice(args, &mut buf, config::standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(buf, expected);
    }

    #[test]
    fn test_reset_args_encode() {
        test_encode_reset_args(ResetArgs::default(), [0, 0]);
        test_encode_reset_args(
            ResetArgs {
                switch_banks: true,
                copy_bank: false,
            },
            [0xAC, 0],
        );
        test_encode_reset_args(
            ResetArgs {
                switch_banks: false,
                copy_bank: true,
            },
            [0, 0xAC],
        );
        test_encode_reset_args(
            ResetArgs {
                switch_banks: true,
                copy_bank: true,
            },
            [0xAC, 0xAC],
        );
    }

    #[test]
    fn test_tfui_args_encode_decode() {
        let args = TfuiArgs {
            num_data_blocks_tx: 0x1234,
            data_len: 0x5678,
            timeout_secs: 0x9ABC,
            broadcast_u16_address: 0xDEF0,
        };
        let expected = [0x34, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xDE];
        let mut buf = [0; TFUI_ARGS_LEN];

        // Test encoding
        bincode::encode_into_slice(args, &mut buf, config::standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(buf, expected);

        // Test decoding
        let (decoded, _): (TfuiArgs, _) =
            bincode::decode_from_slice(&buf, config::standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(decoded, args);
    }

    #[test]
    fn test_tfuq_args_encode() {
        let args = TfuqArgs {
            status_query: TfuqStatusQuery::StatusBank0,
            command: TfuqCommandType::QueryTfuStatus,
        };
        let mut buf = [0; TFUQ_ARGS_LEN];
        bincode::encode_into_slice(args, &mut buf, config::standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(buf, [0x02, 0x00]);
    }

    #[test]
    fn test_tfuq_return_value_decode() {
        let args = TfuqReturnValue {
            active_host: 0x01,
            current_state: 0x02,
            image_write_status: 0x03,
            blocks_written_bitfield: 0x0405,
            block_status: [
                TfuqBlockStatus::Success,
                TfuqBlockStatus::InvalidTfuState,
                TfuqBlockStatus::InvalidHeaderSize,
                TfuqBlockStatus::InvalidDataBlock,
                TfuqBlockStatus::InvalidDataSize,
                TfuqBlockStatus::InvalidSlaveAddress,
                TfuqBlockStatus::InvalidTimeout,
                TfuqBlockStatus::MaxAppConfigUpdate,
                TfuqBlockStatus::HeaderRxInProgress,
                TfuqBlockStatus::HeaderValidAndAuthentic,
                TfuqBlockStatus::HeaderNotValid,
                TfuqBlockStatus::HeaderKeyNotValid,
                TfuqBlockStatus::HeaderRootAuthFailure,
            ],
            num_of_header_bytes_received: 0x06070809,
            num_of_data_bytes_received: 0x0A0B0C0D,
            num_of_app_config_updates: 0x0E0F,
        };
        let expected = [
            0x01, 0x02, 0x00, 0x00, 0x03, 0x05, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
            0x0B, 0xC, 0x09, 0x08, 0x07, 0x06, 0x00, 0x00, 0x0D, 0x0C, 0x0B, 0x0A, 0x00, 0x00, 0x0F, 0x0E,
        ];
        let (decoded, _): (TfuqReturnValue, _) =
            bincode::decode_from_slice(&expected, config::standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(decoded, args);
    }

    #[test]
    fn test_tfud_args_encode_decode() {
        let args = TfudArgs {
            block_number: 0x1234,
            data_len: 0x5678,
            timeout_secs: 0x9ABC,
            broadcast_u16_address: 0xDEF0,
        };
        let expected = [0x34, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xDE];
        let mut buf = [0; TFUD_ARGS_LEN];

        // Test encoding
        bincode::encode_into_slice(args, &mut buf, config::standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(buf, expected);

        // Test decoding
        let (decoded, _): (TfudArgs, _) =
            bincode::decode_from_slice(&buf, config::standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(decoded, args);
    }
}
