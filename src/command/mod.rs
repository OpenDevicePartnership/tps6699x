use bytemuck::{Pod, Zeroable};
use embedded_usb_pd::PdError;
use pack1::{U16LE, U32LE};

use crate::u32_from_str;

pub mod gcdm;
pub mod muxr;
pub mod trig;
pub mod vdms;

/// TaskResult is only defined for lower 4 bits
pub const CMD_4CC_TASK_RETURN_CODE_MASK: u8 = 0x0F;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Command {
    /// Previous command succeeded
    Success = 0,
    /// Invalid Command
    Invalid = u32_from_str(*b"!CMD"),
    /// Reset command
    Gaid = u32_from_str(*b"GAID"),

    /// Simulate a port disconnect.
    ///
    /// The `DISC` Modal Task causes the PD Controller to act as if the USB-C port
    /// is disconnected, with an optional Host-specified delay to restoring normal
    /// port operation. If currently there is no USB-C connection on the port, then
    /// this task will be rejected. The port that will be disconnected when writing
    /// the `DISC` Task will correspond to the I2C target address used when writing
    /// the `DISC` Task.
    ///
    /// # Input
    /// `DISCDelay`: 8-bit value in seconds for disconnect time. If 0, there is
    /// no automatic reconnect.
    ///
    /// # Output
    /// [`ReturnValue`]
    ///
    /// This Task always completes successfully, it has no reason to be rejected
    /// or timed-out. If another Modal Task was already active the `DISC` Modal
    /// Task will cancel that Modal Task and take its place. The `DISC` Modal Task
    /// completes immediately, it does not wait for the reconnect delay.
    DISC = u32_from_str(*b"DISC"),

    /// Tomcat firmware update mode enter
    Tfus = u32_from_str(*b"TFUs"),
    /// Tomcat firmware update mode init
    Tfui = u32_from_str(*b"TFUi"),
    /// Tomcat firmware update mode query
    Tfuq = u32_from_str(*b"TFUq"),
    /// Tomcat firmware update mode exit
    Tfue = u32_from_str(*b"TFUe"),
    /// Tomcat firmware update data
    Tfud = u32_from_str(*b"TFUd"),
    /// Tomcat firmware update complete
    Tfuc = u32_from_str(*b"TFUc"),

    /// System ready to sink
    Srdy = u32_from_str(*b"SRDY"),
    /// SRDY reset
    Sryr = u32_from_str(*b"SRYR"),

    /// Re-evaluate the Autonegotiate Sink register.
    ///
    /// # Input
    /// None.
    ///
    /// # Output
    /// [`ReturnValue`]
    Aneg = u32_from_str(*b"ANeg"),

    /// Trigger an Input GPIO event
    Trig = u32_from_str(*b"Trig"),

    /// Clear the dead battery flag.
    ///
    /// # Input
    /// None.
    ///
    /// # Output
    /// [`ReturnValue`]
    Dbfg = u32_from_str(*b"DBfg"),

    /// Repeat transactions on I2C3m under certain conditions.
    ///
    /// # Input
    /// [`muxr::Input`]
    ///
    /// # Output
    /// [`ReturnValue`]
    Muxr = u32_from_str(*b"MuxR"),

    /// PD Data Reset
    ///
    /// # Input
    /// None.
    ///
    /// # Output
    /// [`ReturnValue`]
    Drst = u32_from_str(*b"DRST"),

    /// Hard Reset
    ///
    /// # Input
    /// None
    ///
    /// # Output
    /// [`ReturnValue`]
    HRST = u32_from_str(*b"HRST"),

    /// Send VDM.
    ///
    /// # Input
    /// [`vdms::Input`]
    ///
    /// # Output
    /// None
    VDMs = u32_from_str(*b"VDMs"),

    /// Execute a UCSI command
    ///
    /// # Input
    /// [`embedded_usb_pd::ucsi::lpm::Command`]
    ///
    /// # Output
    /// [`embedded_usb_pd::ucsi::lpm::ResponseData`]
    Ucsi = u32_from_str(*b"UCSI"),

    /// Get custom discovered modes
    ///
    /// # Input
    /// [`gcdm::Input`]
    ///
    /// # Output
    /// [`gcdm::DiscoveredMode`]
    GCdm = u32_from_str(*b"GCdm"),
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
        } else if Command::DISC == value {
            Ok(Command::DISC)
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
        } else if Command::HRST == value {
            Ok(Command::HRST)
        } else if Command::VDMs == value {
            Ok(Command::VDMs)
        } else if Command::Ucsi == value {
            Ok(Command::Ucsi)
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
            Command::Gaid => RESET_DELAY_MS + 100,
            Command::Tfuc => 2 * RESET_DELAY_MS + TFUC_VERIFICATION_SLACK_MS,
            Command::Srdy | Command::Sryr => 250, // determined by experimentation
            Command::Trig => 500,                 // determined by experimentation
            Command::Drst => 100,                 // PD spec says 24/27/30 ms, round up
            _ => 1000,
        }
    }

    /// If the command returns a standard return value in the first output byte
    pub const fn has_return_value(self) -> bool {
        // Use match because `!=` is not const
        !matches!(self, Command::GCdm)
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
/// Time reserved after the TFUc reset delay to read and verify the application mode.
pub(crate) const TFUC_VERIFICATION_SLACK_MS: u32 = 500;
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
impl From<ResetArgs> for ResetArgsRaw {
    fn from(value: ResetArgs) -> Self {
        Self {
            switch_banks: if value.switch_banks { RESET_FEATURE_ENABLE } else { 0 },
            copy_bank: if value.copy_bank { RESET_FEATURE_ENABLE } else { 0 },
        }
    }
}

impl From<ResetArgsRaw> for ResetArgs {
    fn from(value: ResetArgsRaw) -> Self {
        Self {
            switch_banks: value.switch_banks == RESET_FEATURE_ENABLE,
            copy_bank: value.copy_bank == RESET_FEATURE_ENABLE,
        }
    }
}

/// Raw wire format of [`ResetArgs`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct ResetArgsRaw {
    /// [`RESET_FEATURE_ENABLE`] to swap banks on reset
    pub switch_banks: u8,
    /// [`RESET_FEATURE_ENABLE`] to copy the backup bank to the active bank
    pub copy_bank: u8,
}

/// Delay for completion of TFUs command
pub(crate) const TFUS_DELAY_MS: u32 = 500;
/// Length of TFUi arguments
#[allow(dead_code)]
pub(crate) const TFUI_ARGS_LEN: usize = 8;

/// Arguments for TFUi command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfuiArgs {
    pub num_data_blocks_tx: u16,
    pub data_len: u16,
    pub timeout_secs: u16,
    pub broadcast_u16_address: u16,
}

/// Raw wire format of [`TfuiArgs`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct TfuiArgsRaw {
    pub num_data_blocks_tx: U16LE,
    pub data_len: U16LE,
    pub timeout_secs: U16LE,
    pub broadcast_u16_address: U16LE,
}

impl From<TfuiArgs> for TfuiArgsRaw {
    fn from(value: TfuiArgs) -> Self {
        Self {
            num_data_blocks_tx: value.num_data_blocks_tx.into(),
            data_len: value.data_len.into(),
            timeout_secs: value.timeout_secs.into(),
            broadcast_u16_address: value.broadcast_u16_address.into(),
        }
    }
}

impl From<TfuiArgsRaw> for TfuiArgs {
    fn from(value: TfuiArgsRaw) -> Self {
        Self {
            num_data_blocks_tx: value.num_data_blocks_tx.get(),
            data_len: value.data_len.get(),
            timeout_secs: value.timeout_secs.get(),
            broadcast_u16_address: value.broadcast_u16_address.get(),
        }
    }
}

/// Command type for TFUq command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TfuqCommandType {
    QueryTfuStatus = 0x00,
}

impl From<TfuqCommandType> for u8 {
    fn from(value: TfuqCommandType) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for TfuqCommandType {
    type Error = PdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TfuqCommandType::QueryTfuStatus),
            _ => Err(PdError::InvalidParams),
        }
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

impl From<TfuqStatusQuery> for u8 {
    fn from(value: TfuqStatusQuery) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for TfuqStatusQuery {
    type Error = PdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TfuqStatusQuery::StatusDefault),
            0x01 => Ok(TfuqStatusQuery::StatusInProgress),
            0x02 => Ok(TfuqStatusQuery::StatusBank0),
            0x03 => Ok(TfuqStatusQuery::StatusBank1),
            _ => Err(PdError::InvalidParams),
        }
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

impl From<TfuqBlockStatus> for u8 {
    fn from(value: TfuqBlockStatus) -> Self {
        value as u8
    }
}

/// Length of TFUq args
#[allow(dead_code)]
pub(crate) const TFUQ_ARGS_LEN: usize = 2;

/// Arguments for TFUq command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfuqArgs {
    pub status_query: TfuqStatusQuery,
    pub command: TfuqCommandType,
}

/// Raw wire format of [`TfuqArgs`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct TfuqArgsRaw {
    pub status_query: u8,
    pub command: u8,
}

impl From<TfuqArgs> for TfuqArgsRaw {
    fn from(value: TfuqArgs) -> Self {
        Self {
            status_query: value.status_query.into(),
            command: value.command.into(),
        }
    }
}

impl TryFrom<TfuqArgsRaw> for TfuqArgs {
    type Error = PdError;

    fn try_from(value: TfuqArgsRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            status_query: value.status_query.try_into()?,
            command: value.command.try_into()?,
        })
    }
}

/// Length of return data from TFUq command
#[allow(dead_code)]
pub(crate) const TFUQ_RETURN_LEN: usize = 40;
/// Length of the meaningful portion of the TFUq return data
#[allow(dead_code)]
pub(crate) const TFUQ_RETURN_VALUE_LEN: usize = 34;
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

/// Raw wire format of [`TfuqReturnValue`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct TfuqReturnValueRaw {
    pub active_host: u8,
    pub current_state: u8,
    _reserved0: U16LE,
    pub image_write_status: u8,
    pub blocks_written_bitfield: U16LE,
    pub block_status: [u8; TFUQ_RETURN_BLOCK_STATUS_LEN],
    pub num_of_header_bytes_received: U32LE,
    _reserved1: U16LE,
    pub num_of_data_bytes_received: U32LE,
    _reserved2: U16LE,
    pub num_of_app_config_updates: U16LE,
}

impl From<TfuqReturnValue> for TfuqReturnValueRaw {
    fn from(value: TfuqReturnValue) -> Self {
        let mut block_status = [0u8; TFUQ_RETURN_BLOCK_STATUS_LEN];
        for (raw, status) in block_status.iter_mut().zip(value.block_status.iter()) {
            *raw = (*status).into();
        }

        Self {
            active_host: value.active_host,
            current_state: value.current_state,
            _reserved0: U16LE::default(),
            image_write_status: value.image_write_status,
            blocks_written_bitfield: value.blocks_written_bitfield.into(),
            block_status,
            num_of_header_bytes_received: value.num_of_header_bytes_received.into(),
            _reserved1: U16LE::default(),
            num_of_data_bytes_received: value.num_of_data_bytes_received.into(),
            _reserved2: U16LE::default(),
            num_of_app_config_updates: value.num_of_app_config_updates.into(),
        }
    }
}

impl TryFrom<TfuqReturnValueRaw> for TfuqReturnValue {
    type Error = PdError;

    fn try_from(value: TfuqReturnValueRaw) -> Result<Self, Self::Error> {
        let mut block_status = [TfuqBlockStatus::Success; TFUQ_RETURN_BLOCK_STATUS_LEN];
        for (status, raw) in block_status.iter_mut().zip(value.block_status.iter()) {
            *status = TfuqBlockStatus::try_from(*raw)?;
        }

        Ok(Self {
            active_host: value.active_host,
            current_state: value.current_state,
            image_write_status: value.image_write_status,
            blocks_written_bitfield: value.blocks_written_bitfield.get(),
            block_status,
            num_of_header_bytes_received: value.num_of_header_bytes_received.get(),
            num_of_data_bytes_received: value.num_of_data_bytes_received.get(),
            num_of_app_config_updates: value.num_of_app_config_updates.get(),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TfudArgs {
    pub block_number: u16,
    pub data_len: u16,
    pub timeout_secs: u16,
    pub broadcast_u16_address: u16,
}

/// Raw wire format of [`TfudArgs`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct TfudArgsRaw {
    pub block_number: U16LE,
    pub data_len: U16LE,
    pub timeout_secs: U16LE,
    pub broadcast_u16_address: U16LE,
}

impl From<TfudArgs> for TfudArgsRaw {
    fn from(value: TfudArgs) -> Self {
        Self {
            block_number: value.block_number.into(),
            data_len: value.data_len.into(),
            timeout_secs: value.timeout_secs.into(),
            broadcast_u16_address: value.broadcast_u16_address.into(),
        }
    }
}

impl From<TfudArgsRaw> for TfudArgs {
    fn from(value: TfudArgsRaw) -> Self {
        Self {
            block_number: value.block_number.get(),
            data_len: value.data_len.get(),
            timeout_secs: value.timeout_secs.get(),
            broadcast_u16_address: value.broadcast_u16_address.get(),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    fn test_encode_reset_args(args: ResetArgs, expected: [u8; RESET_ARGS_LEN]) {
        let raw = ResetArgsRaw::from(args);
        assert_eq!(bytemuck::bytes_of(&raw), &expected);

        // Round trip back to the native type
        let decoded: ResetArgsRaw = bytemuck::pod_read_unaligned(&expected);
        assert_eq!(ResetArgs::from(decoded), args);
    }

    #[test]
    fn test_raw_sizes() {
        assert_eq!(core::mem::size_of::<ResetArgsRaw>(), RESET_ARGS_LEN);
        assert_eq!(core::mem::size_of::<TfuiArgsRaw>(), TFUI_ARGS_LEN);
        assert_eq!(core::mem::size_of::<TfudArgsRaw>(), TFUD_ARGS_LEN);
        assert_eq!(core::mem::size_of::<TfuqArgsRaw>(), TFUQ_ARGS_LEN);
        assert_eq!(core::mem::size_of::<TfuqReturnValueRaw>(), TFUQ_RETURN_VALUE_LEN);
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
    fn test_tfuc_timeout_reserves_independent_verification_slack() {
        let actual = Command::Tfuc.timeout_ms();
        assert!(
            TFUC_VERIFICATION_SLACK_MS > 0,
            "TFUC verification slack must be positive"
        );
        assert!(
            actual > 2 * RESET_DELAY_MS,
            "TFUc timeout ({} ms) should be longer than internal delay of 2 * RESET_DELAY_MS",
            actual
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

        // Test encoding
        let raw = TfuiArgsRaw::from(args);
        assert_eq!(bytemuck::bytes_of(&raw), &expected);

        // Test decoding
        let decoded: TfuiArgsRaw = bytemuck::pod_read_unaligned(&expected);
        assert_eq!(TfuiArgs::from(decoded), args);
    }

    #[test]
    fn test_tfuq_args_encode() {
        let args = TfuqArgs {
            status_query: TfuqStatusQuery::StatusBank0,
            command: TfuqCommandType::QueryTfuStatus,
        };
        let raw = TfuqArgsRaw::from(args);
        assert_eq!(bytemuck::bytes_of(&raw), &[0x02, 0x00]);
        assert_eq!(TfuqArgs::try_from(raw).unwrap(), args);
    }

    #[test]
    fn test_tfuq_args_encode_all_variants() {
        // Verify each status_query variant encodes to the correct byte position
        let cases = [
            (TfuqStatusQuery::StatusDefault, [0x00, 0x00]),
            (TfuqStatusQuery::StatusInProgress, [0x01, 0x00]),
            (TfuqStatusQuery::StatusBank0, [0x02, 0x00]),
            (TfuqStatusQuery::StatusBank1, [0x03, 0x00]),
        ];
        for (query, expected) in cases {
            let args = TfuqArgs {
                status_query: query,
                command: TfuqCommandType::QueryTfuStatus,
            };
            let raw = TfuqArgsRaw::from(args);
            assert_eq!(bytemuck::bytes_of(&raw), &expected, "Failed for {:?}", query);
            assert_eq!(TfuqArgs::try_from(raw).unwrap(), args, "Failed for {:?}", query);
        }
    }

    #[test]
    fn test_tfuq_args_invalid_raw() {
        assert_eq!(
            TfuqArgs::try_from(TfuqArgsRaw {
                status_query: 0x04,
                command: 0x00
            }),
            Err(PdError::InvalidParams)
        );
        assert_eq!(
            TfuqArgs::try_from(TfuqArgsRaw {
                status_query: 0x00,
                command: 0x01
            }),
            Err(PdError::InvalidParams)
        );
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
        let raw: TfuqReturnValueRaw = bytemuck::pod_read_unaligned(&expected);
        assert_eq!(TfuqReturnValue::try_from(raw).unwrap(), args);

        // Reserved bytes are zeroed when re-encoding
        assert_eq!(bytemuck::bytes_of(&TfuqReturnValueRaw::from(args)), &expected);
    }

    #[test]
    fn test_tfuq_return_value_invalid_block_status() {
        let mut raw = TfuqReturnValueRaw::from(TfuqReturnValue {
            active_host: 0,
            current_state: 0,
            image_write_status: 0,
            blocks_written_bitfield: 0,
            block_status: [TfuqBlockStatus::Success; TFUQ_RETURN_BLOCK_STATUS_LEN],
            num_of_header_bytes_received: 0,
            num_of_data_bytes_received: 0,
            num_of_app_config_updates: 0,
        });
        raw.block_status[0] = 0xFF;
        assert_eq!(TfuqReturnValue::try_from(raw), Err(PdError::InvalidParams));
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

        // Test encoding
        let raw = TfudArgsRaw::from(args);
        assert_eq!(bytemuck::bytes_of(&raw), &expected);

        // Test decoding
        let decoded: TfudArgsRaw = bytemuck::pod_read_unaligned(&expected);
        assert_eq!(TfudArgs::from(decoded), args);
    }

    #[test]
    fn test_try_from_u32_command() {
        assert_eq!(Command::try_from(Command::Success as u32).unwrap(), Command::Success);
        assert_eq!(Command::try_from(Command::Invalid as u32).unwrap(), Command::Invalid);
        assert_eq!(Command::try_from(Command::Gaid as u32).unwrap(), Command::Gaid);
        assert_eq!(Command::try_from(Command::DISC as u32).unwrap(), Command::DISC);
        assert_eq!(Command::try_from(Command::Tfus as u32).unwrap(), Command::Tfus);
        assert_eq!(Command::try_from(Command::Tfui as u32).unwrap(), Command::Tfui);
        assert_eq!(Command::try_from(Command::Tfuq as u32).unwrap(), Command::Tfuq);
        assert_eq!(Command::try_from(Command::Tfue as u32).unwrap(), Command::Tfue);
        assert_eq!(Command::try_from(Command::Tfud as u32).unwrap(), Command::Tfud);
        assert_eq!(Command::try_from(Command::Tfuc as u32).unwrap(), Command::Tfuc);
        assert_eq!(Command::try_from(Command::Srdy as u32).unwrap(), Command::Srdy);
        assert_eq!(Command::try_from(Command::Sryr as u32).unwrap(), Command::Sryr);
        assert_eq!(Command::try_from(Command::Aneg as u32).unwrap(), Command::Aneg);
        assert_eq!(Command::try_from(Command::Trig as u32).unwrap(), Command::Trig);
        assert_eq!(Command::try_from(Command::Dbfg as u32).unwrap(), Command::Dbfg);
        assert_eq!(Command::try_from(Command::Muxr as u32).unwrap(), Command::Muxr);
        assert_eq!(Command::try_from(Command::Drst as u32).unwrap(), Command::Drst);
        assert_eq!(Command::try_from(Command::HRST as u32).unwrap(), Command::HRST);
        assert_eq!(Command::try_from(Command::VDMs as u32).unwrap(), Command::VDMs);
        assert_eq!(Command::try_from(Command::Ucsi as u32).unwrap(), Command::Ucsi);
        assert_eq!(Command::try_from(0xFFFFFFFFu32), Err(PdError::InvalidParams));
    }

    #[test]
    fn test_try_from_u8_return_value() {
        assert_eq!(ReturnValue::try_from(0x00).unwrap(), ReturnValue::Success);
        assert_eq!(ReturnValue::try_from(0x01).unwrap(), ReturnValue::Abort);
        assert_eq!(ReturnValue::try_from(0x03).unwrap(), ReturnValue::Rejected);
        assert_eq!(ReturnValue::try_from(0x04).unwrap(), ReturnValue::RxLocked);
        assert_eq!(ReturnValue::try_from(0x05).unwrap(), ReturnValue::Task0);
        assert_eq!(ReturnValue::try_from(0x06).unwrap(), ReturnValue::Task1);
        assert_eq!(ReturnValue::try_from(0x07).unwrap(), ReturnValue::Task2);
        assert_eq!(ReturnValue::try_from(0x08).unwrap(), ReturnValue::Task3);
        assert_eq!(ReturnValue::try_from(0x09).unwrap(), ReturnValue::Task4);
        assert_eq!(ReturnValue::try_from(0x0A).unwrap(), ReturnValue::Task5);
        assert_eq!(ReturnValue::try_from(0x0B).unwrap(), ReturnValue::Task6);
        assert_eq!(ReturnValue::try_from(0x0C).unwrap(), ReturnValue::Task7);
        assert_eq!(ReturnValue::try_from(0x0D).unwrap(), ReturnValue::Task8);
        assert_eq!(ReturnValue::try_from(0x0E).unwrap(), ReturnValue::Task9);
        assert_eq!(ReturnValue::try_from(0x0F).unwrap(), ReturnValue::Task10);
        assert_eq!(ReturnValue::try_from(0x02u8), Err(PdError::InvalidParams));
        for invalid in 0x10u8..=0xFF {
            assert_eq!(ReturnValue::try_from(invalid), Err(PdError::InvalidParams));
        }
    }

    #[test]
    fn test_try_from_u8_tfuq_block_status() {
        assert_eq!(TfuqBlockStatus::try_from(0x00).unwrap(), TfuqBlockStatus::Success);
        assert_eq!(
            TfuqBlockStatus::try_from(0x01).unwrap(),
            TfuqBlockStatus::InvalidTfuState
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x02).unwrap(),
            TfuqBlockStatus::InvalidHeaderSize
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x03).unwrap(),
            TfuqBlockStatus::InvalidDataBlock
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x04).unwrap(),
            TfuqBlockStatus::InvalidDataSize
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x05).unwrap(),
            TfuqBlockStatus::InvalidSlaveAddress
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x06).unwrap(),
            TfuqBlockStatus::InvalidTimeout
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x07).unwrap(),
            TfuqBlockStatus::MaxAppConfigUpdate
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x08).unwrap(),
            TfuqBlockStatus::HeaderRxInProgress
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x09).unwrap(),
            TfuqBlockStatus::HeaderValidAndAuthentic
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x0A).unwrap(),
            TfuqBlockStatus::HeaderNotValid
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x0B).unwrap(),
            TfuqBlockStatus::HeaderKeyNotValid
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x0C).unwrap(),
            TfuqBlockStatus::HeaderRootAuthFailure
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x0D).unwrap(),
            TfuqBlockStatus::HeaderFwheaderAuthFailure
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x0E).unwrap(),
            TfuqBlockStatus::DataRxInProgress
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x0F).unwrap(),
            TfuqBlockStatus::DataValidAndAuthentic
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x10).unwrap(),
            TfuqBlockStatus::DataValidButRepeated
        );
        assert_eq!(TfuqBlockStatus::try_from(0x11).unwrap(), TfuqBlockStatus::DataNotValid);
        assert_eq!(TfuqBlockStatus::try_from(0x12).unwrap(), TfuqBlockStatus::DataInvalidId);
        assert_eq!(
            TfuqBlockStatus::try_from(0x13).unwrap(),
            TfuqBlockStatus::DataAuthFailure
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x14).unwrap(),
            TfuqBlockStatus::F911IdNotValid
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x15).unwrap(),
            TfuqBlockStatus::F911DataNotValid
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x16).unwrap(),
            TfuqBlockStatus::F911AuthFailure
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x17).unwrap(),
            TfuqBlockStatus::ImageDownloadTimeout
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x18).unwrap(),
            TfuqBlockStatus::BlockDownloadTimeout
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x19).unwrap(),
            TfuqBlockStatus::BlockWriteFailed
        );
        assert_eq!(
            TfuqBlockStatus::try_from(0x1A).unwrap(),
            TfuqBlockStatus::SpecialCmdFailed
        );
        for invalid in 0x1B..=0xFF {
            assert_eq!(TfuqBlockStatus::try_from(invalid), Err(PdError::InvalidParams));
        }
    }
}
