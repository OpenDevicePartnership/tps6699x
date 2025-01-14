use embedded_usb_pd::PdError;

use crate::u32_from_str;

pub const REG_DATA1: u8 = 0x09;
// Register is 512 bits
pub const REG_DATA1_LEN: usize = 64;

/// Delay after reset before we can assume the controller is ready
// Derived from experimentation
pub const RESET_DELAY_MS: u32 = 1500;

pub const CMD_LEN: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Command {
    /// Previous command succeeded
    Success = 0,
    /// Invalid Command
    Invalid = u32_from_str("!CMD"),
    /// Cold-reset
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
}

impl PartialEq<u32> for Command {
    fn eq(&self, other: &u32) -> bool {
        *self as u32 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

pub const PD_FW_IMAGE_ID_LENGTH: usize = 4;
pub const PD_FW_HEADER_METADATA_OFFSET: usize = PD_FW_IMAGE_ID_LENGTH;
pub const PD_FW_HEADER_METADATA_LENGTH: usize = 8;
pub const PD_FW_APP_IMAGE_SIZE_OFFSET: usize = 0x4F8;
pub const PD_FW_HEADER_BLOCK_OFFSET: usize = PD_FW_HEADER_METADATA_OFFSET + PD_FW_HEADER_METADATA_LENGTH;
pub const PD_FW_HEADER_BLOCK_LENGTH: usize = 0x800;
pub const BURST_WRITE_SIZE: usize = 256;
pub const PD_FW_DATA_BLOCK_SIZE: usize = 0x4000;
pub const PD_FW_DATA_BLOCK_METADATA_SIZE: usize = 8;
pub const PD_FW_APP_CONFIG_SIZE: usize = 0x800;
pub const PD_FW_APP_CONFIG_METADATA_SIZE: usize = 0x8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfuiArgs {
    pub num_data_blocks_tx: u16,
    pub data_len: u16,
    pub timeout_secs: u16,
    pub broadcast_u16_address: u16,
}

const TFUI_ARGS_LEN: usize = 8;

impl TfuiArgs {
    pub fn encode_into_slice(&self, buf: &mut [u8]) -> Result<(), PdError> {
        if buf.len() < TFUI_ARGS_LEN {
            return Err(PdError::InvalidParams);
        }
        buf[0..2].copy_from_slice(&self.num_data_blocks_tx.to_le_bytes());
        buf[2..4].copy_from_slice(&self.data_len.to_le_bytes());
        buf[4..6].copy_from_slice(&self.timeout_secs.to_le_bytes());
        buf[6..8].copy_from_slice(&self.broadcast_u16_address.to_le_bytes());
        Ok(())
    }

    pub fn decode_from_slice(buf: &[u8]) -> Result<Self, PdError> {
        if buf.len() < TFUI_ARGS_LEN {
            return Err(PdError::InvalidParams);
        }
        let number_data_blocks_tx = u16::from_le_bytes([buf[0], buf[1]]);
        let tfu_block_size = u16::from_le_bytes([buf[2], buf[3]]);
        let timeout_secs = u16::from_le_bytes([buf[4], buf[5]]);
        let broadcast_u16_address = u16::from_le_bytes([buf[6], buf[7]]);
        Ok(Self {
            num_data_blocks_tx: number_data_blocks_tx,
            data_len: tfu_block_size,
            timeout_secs,
            broadcast_u16_address,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TfuqCommandType {
    QueryTfuStatus = 0x00,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TfuqStatusQuery {
    StatusDefault = 0x00,
    StatusInProgress,
    StatusBank0,
    StatusBank1,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfuqArgs {
    pub status_query: TfuqStatusQuery,
    pub command: TfuqCommandType,
}

impl TfuqArgs {
    pub fn encode_into_slice(&self, buf: &mut [u8]) -> Result<(), PdError> {
        if buf.len() < 2 {
            return Err(PdError::InvalidParams);
        }
        buf[0] = self.status_query as u8;
        buf[1] = self.command as u8;
        Ok(())
    }
}

pub const TFUQ_RETURN_LEN: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfuqReturnValue {
    pub active_host: u8,
    pub current_state: u8,
    pub image_write_status: u8,
    pub blocks_written_bitfield: u16,
    pub data_block_status: [u8; 13],
    pub num_of_header_bytes_received: u32,
    pub num_of_data_bytes_received: u32,
    pub num_of_app_config_updates: u16,
}

impl TfuqReturnValue {
    pub fn decode_from_slice(buf: &[u8]) -> Result<Self, PdError> {
        if buf.len() < TFUQ_RETURN_LEN {
            return Err(PdError::InvalidParams);
        }
        let active_host = buf[0];
        let current_state = buf[1];
        // _reserved1: bytes 2 and 3
        let image_write_status = buf[4];
        let blocks_written_bitfield = u16::from_le_bytes([buf[5], buf[6]]);
        let data_block_status = <[u8; 13]>::try_from(&buf[7..20]).unwrap();
        let num_of_header_bytes_received = u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]);
        // _reserved2: bytes 24 and 25
        let num_of_data_bytes_received = u32::from_le_bytes([buf[26], buf[27], buf[28], buf[29]]);
        // _reserved3: bytes 30 and 31
        let num_of_app_config_updates = u16::from_le_bytes([buf[30], buf[31]]);

        Ok(Self {
            active_host,
            current_state,
            image_write_status,
            blocks_written_bitfield,
            data_block_status,
            num_of_header_bytes_received,
            num_of_data_bytes_received,
            num_of_app_config_updates,
        })
    }
}

pub const TFUD_ARGS_LEN: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfudArgs {
    pub block_number: u16,
    pub data_len: u16,
    pub timeout_secs: u16,
    pub broadcast_u16_address: u16,
}

impl TfudArgs {
    pub fn encode_into_slice(&self, buf: &mut [u8]) -> Result<(), PdError> {
        if buf.len() < TFUD_ARGS_LEN {
            return Err(PdError::InvalidParams);
        }
        buf[0..2].copy_from_slice(&self.block_number.to_le_bytes());
        buf[2..4].copy_from_slice(&self.data_len.to_le_bytes());
        buf[4..6].copy_from_slice(&self.timeout_secs.to_le_bytes());
        buf[6..8].copy_from_slice(&self.broadcast_u16_address.to_le_bytes());
        Ok(())
    }

    pub fn decode_from_slice(buf: &[u8]) -> Result<Self, PdError> {
        if buf.len() < TFUD_ARGS_LEN {
            return Err(PdError::InvalidParams);
        }
        let block_number = u16::from_le_bytes([buf[0], buf[1]]);
        let header_size = u16::from_le_bytes([buf[2], buf[3]]);
        let timeout_secs = u16::from_le_bytes([buf[4], buf[5]]);
        let broadcast_u16_address = u16::from_le_bytes([buf[6], buf[7]]);
        Ok(Self {
            block_number,
            data_len: header_size,
            timeout_secs,
            broadcast_u16_address,
        })
    }
}

pub const RESET_ARGS_LEN: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResetArgs {
    pub switch_banks: u8,
    pub copy_bank: u8,
}

pub const RESET_FEATURE_ENABLE: u8 = 0xAC;

impl ResetArgs {
    pub fn encode_into_slice(&self, buf: &mut [u8]) -> Result<(), PdError> {
        if buf.len() < RESET_ARGS_LEN {
            return Err(PdError::InvalidParams);
        }
        buf[0] = self.switch_banks;
        buf[1] = self.copy_bank;
        Ok(())
    }
}
