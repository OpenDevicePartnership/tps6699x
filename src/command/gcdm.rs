//! Get custom discovered modes command
use embedded_usb_pd::vdm::Svid;

/// Input data length
pub const INPUT_LEN: usize = 3;

/// GCdm input
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Input {
    pub svid: Svid,
}

impl bincode::Encode for Input {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        // First byte is reserved
        0u8.encode(encoder)?;
        self.svid.0.encode(encoder)
    }
}

impl From<Svid> for Input {
    fn from(svid: Svid) -> Self {
        Self { svid }
    }
}

/// Representation of a custom discovered mode
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, bincode::Decode)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DiscoveredMode {
    /// Raw VDO data
    pub vdo: u32,
    /// VDO object position
    pub position: u8,
}

/// Output data length
pub const OUTPUT_LEN: usize = 35;

/// Length of the discovered modes array
pub const DISCOVERED_MODES_LEN: usize = 7;

/// GCdm output
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, bincode::Decode)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DiscoveredModes {
    pub alt_modes: [DiscoveredMode; DISCOVERED_MODES_LEN],
}
