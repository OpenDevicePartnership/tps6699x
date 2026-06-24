//! Get custom discovered modes command
use embedded_usb_pd::vdm::structured::Svid;

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

#[cfg(test)]
mod tests {
    extern crate std;

    use bincode::config;

    use super::*;

    #[test]
    fn test_gcdm_input_encode() {
        // Construct via From<Svid> and verify encode produces correct bytes
        let input = Input::from(Svid(0xAB12));

        // Encode layout: reserved byte (0x00) + SVID u16 LE (0x12, 0xAB)
        const EXPECTED: [u8; INPUT_LEN] = [0x00, 0x12, 0xAB];
        let mut buf = [0u8; INPUT_LEN];
        bincode::encode_into_slice(&input, &mut buf, config::standard().with_fixed_int_encoding())
            .expect("encode failed");
        assert_eq!(buf, EXPECTED);

        // Verify accessor
        assert_eq!(input.svid, Svid(0xAB12));
    }

    #[test]
    fn test_gcdm_discovered_modes_decode() {
        // Construct expected struct with non-zero values
        let expected_struct = DiscoveredModes {
            alt_modes: [
                DiscoveredMode {
                    vdo: 0x12345678,
                    position: 1,
                },
                DiscoveredMode {
                    vdo: 0x9ABCDEF0,
                    position: 2,
                },
                DiscoveredMode {
                    vdo: 0xDEADBEEF,
                    position: 3,
                },
                DiscoveredMode {
                    vdo: 0xA1B2C3D4,
                    position: 4,
                },
                DiscoveredMode {
                    vdo: 0xFDB97531,
                    position: 5,
                },
                DiscoveredMode {
                    vdo: 0x2468ACE0,
                    position: 6,
                },
                DiscoveredMode {
                    vdo: 0x13579BDF,
                    position: 7,
                },
            ],
        };

        const EXPECTED_BYTES: [u8; OUTPUT_LEN] = [
            0x78, 0x56, 0x34, 0x12, 0x01, 0xF0, 0xDE, 0xBC, 0x9A, 0x02, 0xEF, 0xBE, 0xAD, 0xDE, 0x03, 0xD4, 0xC3, 0xB2,
            0xA1, 0x04, 0x31, 0x75, 0xB9, 0xFD, 0x05, 0xE0, 0xAC, 0x68, 0x24, 0x06, 0xDF, 0x9B, 0x57, 0x13, 0x07,
        ];

        // Decode from bytes and verify against expected struct
        let (decoded, _): (DiscoveredModes, _) =
            bincode::decode_from_slice(&EXPECTED_BYTES, config::standard().with_fixed_int_encoding())
                .expect("decode failed");
        assert_eq!(decoded, expected_struct);

        // Verify individual field accessors
        assert_eq!(decoded.alt_modes[0].vdo, 0x12345678);
        assert_eq!(decoded.alt_modes[0].position, 1);
        assert_eq!(decoded.alt_modes[3].vdo, 0xA1B2C3D4);
        assert_eq!(decoded.alt_modes[3].position, 4);
        assert_eq!(decoded.alt_modes[6].vdo, 0x13579BDF);
        assert_eq!(decoded.alt_modes[6].position, 7);
    }
}
