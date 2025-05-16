use embedded_usb_pd::PdError;

use crate::asynchronous::fw_update::{app_config_block_metadata_offset, data_block_metadata_offset};
use crate::command::{TfudArgs, TfuiArgs};
use crate::stream::{ReadOperation, SeekOperation};
use crate::trace;

/// Header block index
pub const HEADER_BLOCK_INDEX: usize = 0;
/// Data block start index
pub const DATA_BLOCK_START_INDEX: usize = 1;
/// App config block index
pub const APP_CONFIG_BLOCK_INDEX: usize = 12;

/// Image ID length
pub const IMAGE_ID_LEN: usize = 4;

/// App image size offset
pub const APP_IMAGE_SIZE_OFFSET: usize = 0x4F8;
/// App image size length
pub const APP_IMAGE_SIZE_LEN: usize = 4;

/// Header metadata offset
pub const HEADER_METADATA_OFFSET: usize = IMAGE_ID_LEN;
/// Header metadata length
pub const HEADER_METADATA_LEN: usize = 8;
/// Header block data offset
pub const HEADER_BLOCK_OFFSET: usize = HEADER_METADATA_OFFSET + HEADER_METADATA_LEN;
/// Header block length
pub const HEADER_BLOCK_LEN: usize = 0x800;
/// Header block up to app image size len
pub const HEADER_BLOCK_START_LEN: usize = APP_IMAGE_SIZE_OFFSET - HEADER_BLOCK_OFFSET;
/// Header block size following app image size len
pub const HEADER_BLOCK_REST_LEN: usize = HEADER_BLOCK_LEN - APP_IMAGE_SIZE_LEN - HEADER_BLOCK_START_LEN;

/// Data block length
pub const DATA_BLOCK_LEN: usize = 0x4000;
/// Data block metadata length
pub const DATA_BLOCK_METADATA_LEN: usize = 8;
/// App config metadata length
pub const APP_CONFIG_METADATA_LEN: usize = 8;
/// Maximum metadata length
pub const MAX_METADATA_LEN: usize = 8;

/// Delay after sending burst write for TFUi command
pub const TFUI_BURST_WRITE_DELAY_MS: u32 = 250;
/// Delay after sending burst write for TFUd command
pub const TFUD_BURST_WRITE_DELAY_MS: u32 = 150;
/// Default PD FW chunking size
pub const UPDATE_CHUNK_LENGTH: usize = 1024;

/// Current update state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum State {
    /// Receiving update args
    UpdateArgs,
    /// Receiving header block
    HeaderBlockStart,
    /// Receiving image size
    ImageSize,
    // Receiver rest of header block
    HeaderBlockRest,
    /// Receiving data block N header
    DataBlockHeader(usize),
    /// Receiving data block N
    DataBlock(usize),
    /// Receiving app config block header
    ConfigHeader,
    /// Receiving app config block
    ConfigBlock,
    /// Complete
    Complete,
}

impl State {
    /// Produce the next read operation.
    ///
    /// Used by [`State::UpdateArgs`], [`State::HeaderBlockStart`], [`State::ImageSize`], [`State::HeaderBlockRest`],
    /// [`State::DataBlockHeader`], and [`State::ConfigHeader`] states.
    /// The read lengths for all these states are fixed.
    pub fn next_read(&mut self) -> Result<ReadOperation, PdError> {
        match self {
            State::UpdateArgs => {
                trace!("next_read UpdateArgs");
                Ok(ReadOperation::new(HEADER_METADATA_LEN))
            }
            State::HeaderBlockStart => {
                trace!("next_read HeaderBlockStart");
                Ok(ReadOperation::new(HEADER_BLOCK_START_LEN))
            }
            State::ImageSize => {
                trace!("next_read ImageSize");
                Ok(ReadOperation::new(APP_IMAGE_SIZE_LEN))
            }
            State::HeaderBlockRest => {
                trace!("next_read HeaderBlockRest");
                Ok(ReadOperation::new(HEADER_BLOCK_REST_LEN))
            }
            State::DataBlockHeader(block_index) => {
                trace!("next_read DataBlockHeader {}", block_index);
                Ok(ReadOperation::new(DATA_BLOCK_METADATA_LEN))
            }
            State::ConfigHeader => {
                trace!("next_read ConfigHeader");
                Ok(ReadOperation::new(APP_CONFIG_METADATA_LEN))
            }
            _ => {
                trace!("next_read other: {:#?}", self);
                Err(PdError::Failed)
            }
        }
    }

    /// Produce the next read operation.
    ///
    /// Used by [`State::DataBlock`] and [`State::ConfigBlock`] states.
    /// The `block_args` parameter is required because it contains the length of the data to be read.
    pub fn next_read_block_args(&mut self, block_args: &TfudArgs) -> Result<ReadOperation, PdError> {
        match self {
            State::DataBlock(block_index) => {
                trace!("next_read_block_args DataBlock {}", block_index);
                Ok(ReadOperation::new(block_args.data_len.into()))
            }
            State::ConfigBlock => {
                trace!("next_read_block_args ConfigBlock");
                Ok(ReadOperation::new(block_args.data_len.into()))
            }
            _ => {
                trace!("next_read_block_args other: {:#?}", self);
                Err(PdError::Failed)
            }
        }
    }

    /// Produce the next seek operation.
    ///
    /// For states that use this function the seek is a no-op and happens when data immediately followes the previous data,
    /// but keeps things consistent so that we're always alternating between read and seek operations.
    /// Used by the following states: [`State::ImageSize`], [`State::DataBlockHeader`], [`State::ConfigHeader`]
    /// and [`State::ConfigBlock`]. Requires the current position to be passed in.
    pub fn next_seek_nop(&mut self, current: usize) -> Result<SeekOperation, PdError> {
        match self {
            State::ImageSize => {
                trace!("next_seek_nop ImageSize");
                *self = State::HeaderBlockRest;
                Ok(SeekOperation::new(current))
            }
            State::DataBlockHeader(block_index) => {
                trace!("next_seek_nop DataBlockHeader({})", block_index);
                *self = State::DataBlock(*block_index);
                Ok(SeekOperation::new(current))
            }
            State::ConfigHeader => {
                trace!("next_seek_nop ConfigHeader");
                *self = State::ConfigBlock;
                Ok(SeekOperation::new(current))
            }
            State::ConfigBlock => {
                trace!("next_seek_nop ConfigBlock");
                *self = State::Complete;
                Ok(SeekOperation::new(current))
            }
            _ => {
                trace!("next_seek_nop other: {:#?}", self);
                Err(PdError::Failed)
            }
        }
    }

    /// Produce the next seek operation.
    ///
    /// Used only by [`State::DataBlock`] state.
    /// The arguments here are required to calculate the offset of the next data block header.
    pub fn next_seek_block_args(
        &mut self,
        update_args: &TfuiArgs,
        image_size: usize,
    ) -> Result<SeekOperation, PdError> {
        match self {
            State::DataBlock(block_index) => {
                trace!("next_seek_block_args: DataBlock({})", block_index);
                let next_block = *block_index + 1;
                if next_block < update_args.num_data_blocks_tx.into() {
                    *self = State::DataBlockHeader(next_block);
                    Ok(SeekOperation::new(data_block_metadata_offset(next_block)))
                } else {
                    *self = State::ConfigHeader;
                    Ok(SeekOperation::new(app_config_block_metadata_offset(
                        update_args.num_data_blocks_tx.into(),
                        image_size,
                    )))
                }
            }
            _ => {
                trace!("next_seek_block_args other: {:#?}", self);
                Err(PdError::Failed)
            }
        }
    }

    /// Produce the next seek operation.
    ///
    /// Used by [`State::UpdateArgs`], [`State::HeaderBlockStart`], and [`State::HeaderBlockRest`] states
    pub fn next_seek(&mut self) -> Result<SeekOperation, PdError> {
        match self {
            State::UpdateArgs => {
                trace!("next_seek UpdateArgs");
                *self = State::HeaderBlockStart;
                Ok(SeekOperation::new(HEADER_BLOCK_OFFSET))
            }
            State::HeaderBlockStart => {
                trace!("next_seek HeaderBlockStart");
                *self = State::ImageSize;
                Ok(SeekOperation::new(APP_IMAGE_SIZE_OFFSET))
            }
            State::HeaderBlockRest => {
                trace!("next_seek HeaderBlockRest");
                *self = State::DataBlockHeader(0);
                Ok(SeekOperation::new(data_block_metadata_offset(0)))
            }
            _ => {
                trace!("next_seek other: {:#?}", self);
                Err(PdError::Failed)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    /// Test successful state transitions for [`next_read`]
    #[test]
    fn test_success_next_read() {
        // Test UpdateArgs state
        let mut state = State::UpdateArgs;
        let result = state.next_read();
        assert_eq!(result, Ok(ReadOperation::new(HEADER_METADATA_LEN)));
        assert_eq!(state, State::UpdateArgs);

        let mut state = State::HeaderBlockStart;
        let result = state.next_read();
        assert_eq!(result, Ok(ReadOperation::new(HEADER_BLOCK_START_LEN)));
        assert_eq!(state, State::HeaderBlockStart);

        // Test ImageSize state
        let mut state = State::ImageSize;
        let result = state.next_read();
        assert_eq!(result, Ok(ReadOperation::new(APP_IMAGE_SIZE_LEN)));
        assert_eq!(state, State::ImageSize);

        // Test HeaderBlockRest state
        let mut state = State::HeaderBlockRest;
        let result = state.next_read();
        assert_eq!(result, Ok(ReadOperation::new(HEADER_BLOCK_REST_LEN)));
        assert_eq!(state, State::HeaderBlockRest);

        // Test DataBlockHeader state
        let mut state = State::DataBlockHeader(0);
        let result = state.next_read();
        assert_eq!(result, Ok(ReadOperation::new(DATA_BLOCK_METADATA_LEN)));
        assert_eq!(state, State::DataBlockHeader(0));

        // Test ConfigHeader state
        let mut state = State::ConfigHeader;
        let result = state.next_read();
        assert_eq!(result, Ok(ReadOperation::new(APP_CONFIG_METADATA_LEN)));
        assert_eq!(state, State::ConfigHeader);
    }

    /// Test failed state transitions for [`next_read`]
    #[test]
    fn test_failure_next_read() {
        // Test DataBlock state
        let mut state = State::DataBlock(0);
        let result = state.next_read();
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::DataBlock(0));

        // Test ConfigBlock state
        let mut state = State::ConfigBlock;
        let result = state.next_read();
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::ConfigBlock);

        // Test Complete state
        let mut state = State::Complete;
        let result = state.next_read();
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::Complete);
    }

    /// Test successful state transitions for [`next_read_block_args`]
    #[test]
    fn test_success_next_read_block_args() {
        // Test DataBlock state
        let mut state = State::DataBlock(0);
        let result = state.next_read_block_args(&MOCK_DATA_BLOCK_HEADER_0);
        assert_eq!(result, Ok(ReadOperation::new(MOCK_DEFAULT_DATA_BLOCK_SIZE as usize)));
        assert_eq!(state, State::DataBlock(0));

        // Test ConfigBlock state
        let mut state = State::ConfigBlock;
        let result = state.next_read_block_args(&APP_CONFIG_HEADER);
        assert_eq!(result, Ok(ReadOperation::new(MOCK_APP_CONFIG_SIZE as usize)));
        assert_eq!(state, State::ConfigBlock);
    }

    /// Test failed state transitions for [`next_read_block_args`]
    #[test]
    fn test_failure_next_read_block_args() {
        // Test UpdateArgs state
        let mut state = State::UpdateArgs;
        let result = state.next_read_block_args(&MOCK_DATA_BLOCK_HEADER_0);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::UpdateArgs);

        // Test HeaderBlockStart state
        let mut state = State::HeaderBlockStart;
        let result = state.next_read_block_args(&MOCK_DATA_BLOCK_HEADER_0);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::HeaderBlockStart);

        // Test ImageSize state
        let mut state = State::ImageSize;
        let result = state.next_read_block_args(&MOCK_DATA_BLOCK_HEADER_0);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::ImageSize);

        // Test HeaderBlockRest state
        let mut state = State::HeaderBlockRest;
        let result = state.next_read_block_args(&MOCK_DATA_BLOCK_HEADER_0);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::HeaderBlockRest);

        // Test DataBlockHeader state
        let mut state = State::DataBlockHeader(0);
        let result = state.next_read_block_args(&MOCK_DATA_BLOCK_HEADER_0);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::DataBlockHeader(0));

        // Test ConfigHeader state
        let mut state = State::ConfigHeader;
        let result = state.next_read_block_args(&MOCK_DATA_BLOCK_HEADER_0);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::ConfigHeader);

        // Test Complete state
        let mut state = State::Complete;
        let result = state.next_read_block_args(&MOCK_DATA_BLOCK_HEADER_0);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::Complete);
    }

    /// Test successful state transitions for [`next_seek_nop`]
    #[test]
    fn test_success_next_seek_nop() {
        // Test HeaderBlockStart state
        let mut state = State::ImageSize;
        let current_position = 1024;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::HeaderBlockRest);

        // Test DataBlockHeader state
        let mut state = State::DataBlockHeader(0);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(0));

        let mut state = State::DataBlockHeader(1);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(1));

        let mut state = State::DataBlockHeader(2);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(2));

        let mut state = State::DataBlockHeader(3);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(3));

        let mut state = State::DataBlockHeader(4);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(4));

        let mut state = State::DataBlockHeader(5);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(5));

        let mut state = State::DataBlockHeader(6);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(6));

        let mut state = State::DataBlockHeader(7);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(7));

        let mut state = State::DataBlockHeader(8);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(8));

        let mut state = State::DataBlockHeader(9);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(9));

        let mut state = State::DataBlockHeader(10);
        let current_position = 2048;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::DataBlock(10));

        // Test ConfigHeader state
        let mut state = State::ConfigHeader;
        let current_position = 4096;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::ConfigBlock);

        // Test ConfigHeader state
        let mut state = State::ConfigBlock;
        let current_position = 8192;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Ok(SeekOperation::new(current_position)));
        assert_eq!(state, State::Complete);
    }

    /// Test failed state transitions for [`next_seek_nop`]
    #[test]
    fn test_failure_next_seek_nop() {
        let current_position = 512;

        // Test UpdateArgs state
        let mut state = State::UpdateArgs;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::UpdateArgs);

        // Test HeaderBlockStart state
        let mut state = State::HeaderBlockStart;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::HeaderBlockStart);

        // Test DataBlock state
        let mut state = State::DataBlock(0);
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::DataBlock(0));

        // Test Complete state
        let mut state = State::Complete;
        let result = state.next_seek_nop(current_position);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::Complete);
    }

    /// Test successful state transitions for [`next_seek_block_args`]
    #[test]
    fn test_success_next_seek_block_args() {
        let app_size = MOCK_APP_SIZE as usize;

        // Test standard block to block transition
        let mut state = State::DataBlock(0);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_1_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(1));

        let mut state = State::DataBlock(1);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_2_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(2));

        let mut state = State::DataBlock(2);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_3_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(3));

        let mut state = State::DataBlock(3);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_4_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(4));

        let mut state = State::DataBlock(4);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_5_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(5));

        let mut state = State::DataBlock(5);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_6_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(6));

        let mut state = State::DataBlock(6);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_7_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(7));

        let mut state = State::DataBlock(7);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_8_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(8));

        let mut state = State::DataBlock(8);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_9_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(9));

        let mut state = State::DataBlock(9);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(MOCK_DATA_BLOCK_HEADER_10_OFFSET)));
        assert_eq!(state, State::DataBlockHeader(10));

        // Test transition to config block
        let mut state = State::DataBlock(10);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Ok(SeekOperation::new(APP_CONFIG_HEADER_OFFSET)));
        assert_eq!(state, State::ConfigHeader);
    }

    /// Test failed state transitions for [`next_seek_block_args`]
    #[test]
    fn test_failure_next_seek_block_args() {
        let app_size = MOCK_APP_SIZE as usize;

        // Test UpdateArgs state
        let mut state = State::UpdateArgs;
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::UpdateArgs);

        // Test HeaderBlockStart state
        let mut state = State::HeaderBlockStart;
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::HeaderBlockStart);

        // Test HeaderBlockRest state
        let mut state = State::HeaderBlockRest;
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::HeaderBlockRest);

        // Test DataBlockHeader state
        let mut state = State::DataBlockHeader(0);
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::DataBlockHeader(0));

        // Test ConfigBlock state
        let mut state = State::ConfigBlock;
        let result = state.next_seek_block_args(&MOCK_UPDATE_HEADER, app_size);
        assert_eq!(result, Err(PdError::Failed));
        assert_eq!(state, State::ConfigBlock);
    }
}
