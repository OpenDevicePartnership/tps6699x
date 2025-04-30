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
