use bitfield::bitfield;
use embedded_usb_pd::pdo::{Common, ExpectedPdo, RoleCommon, sink, source};

/// Rx source caps register address
pub const RX_SRC_ADDR: u8 = 0x30;

/// Rx sink caps register address
pub const RX_SNK_ADDR: u8 = 0x31;

/// Length of the register in bytes
pub const LEN: usize = 45;

/// Total number of PDOs supported
pub const TOTAL_PDOS: usize = NUM_SPR_PDOS + NUM_EPR_PDOS;

/// Total length in bytes of the register header
/// [`RxSrcCapsRaw::num_valid_pdos`], [`RxSrcCapsRaw::num_valid_epr_pdos`] and [`RxSrcCapsRaw::last_src_cap_is_epr`]
pub const HEADER_LEN: usize = 1;

/// Starting index of SPR PDOs in the register
pub const SPR_PDO_START_INDEX: usize = 0;
/// Number of SPR PDOs
pub const NUM_SPR_PDOS: usize = 7;

/// Starting index of EPR PDOs in the register
pub const EPR_PDO_START_INDEX: usize = SPR_PDO_START_INDEX + NUM_SPR_PDOS;
/// Number of EPR PDOs
pub const NUM_EPR_PDOS: usize = 4;

bitfield! {
    /// Received source/sink capabilities register
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct RxCapsRaw([u8]);
    impl Debug;

    /// Number of Valid PDOs
    pub u8, num_valid_pdos, set_num_valid_pdos: 2, 0;
    /// Number of Valid EPR PDOs
    pub u8, num_valid_epr_pdos, set_num_valid_epr_pdos: 5, 3;
    /// Last Src Cap Received is EPR
    pub bool, last_src_cap_is_epr, set_last_src_cap_is_epr: 6;

    /// Standard PDO 0
    pub u32, pdo0, set_pdo0: 39, 8;
    /// Standard PDO 1
    pub u32, pdo1, set_pdo1: 71, 40;
    /// Standard PDO 2
    pub u32, pdo2, set_pdo2: 103, 72;
    /// Standard PDO 3
    pub u32, pdo3, set_pdo3: 135, 104;
    /// Standard PDO 4
    pub u32, pdo4, set_pdo4: 167, 136;
    /// Standard PDO 5
    pub u32, pdo5, set_pdo5: 199, 168;
    /// Standard PDO 6
    pub u32, pdo6, set_pdo6: 231, 200;

    /// EPR PDO 0
    pub u32, epr_pdo0, set_epr_pdo0: 263, 232;
    /// EPR PDO 1
    pub u32, epr_pdo1, set_epr_pdo1: 295, 264;
    /// EPR PDO 2
    pub u32, epr_pdo2, set_epr_pdo2: 327, 296;
    /// EPR PDO 3
    pub u32, epr_pdo3, set_epr_pdo3: 359, 328;
}

/// High-level wrapper around [`RxSrcCapsRaw`].
#[derive(Clone, Copy, Debug)]
pub struct RxCaps<T: Common> {
    /// Number of valid standard PDOs
    num_valid_pdos: u8,
    /// Number of valid EPR PDOs
    num_valid_epr_pdos: u8,
    /// Last source capabilities received is EPR
    last_src_cap_is_epr: bool,
    /// PDOs
    pdos: [T; TOTAL_PDOS],
}

impl<T: Common> RxCaps<T> {
    /// Get number of valid standard PDOs
    pub fn num_valid_pdos(&self) -> u8 {
        self.num_valid_pdos
    }

    /// Set number of valid standard PDOs
    pub fn set_num_valid_pdos(&mut self, num: u8) -> &mut Self {
        self.num_valid_pdos = num;
        self
    }

    /// Get number of valid EPR PDOs
    pub fn num_valid_epr_pdos(&self) -> u8 {
        self.num_valid_epr_pdos
    }

    /// Set number of valid EPR PDOs
    pub fn set_num_valid_epr_pdos(&mut self, num: u8) -> &mut Self {
        self.num_valid_epr_pdos = num;
        self
    }

    /// Get whether last source cap received is EPR
    pub fn last_src_cap_is_epr(&self) -> bool {
        self.last_src_cap_is_epr
    }

    /// Set whether last source cap received is EPR
    pub fn set_last_src_cap_is_epr(&mut self, is_epr: bool) -> &mut Self {
        self.last_src_cap_is_epr = is_epr;
        self
    }

    /// Checked indexing into the PDOs
    pub fn get(&self, index: usize) -> Option<&T> {
        self.pdos.get(index)
    }

    /// Checked mutable indexing into the PDOs
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.pdos.get_mut(index)
    }
}

/// Struct for [`RxCapsError::ExpectedPdo`]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidPdoIndex {
    pub requested: usize,
    pub max: usize,
}

/// Error type for functions that deal with received capabilities
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxCapsError {
    /// PDO conversion error
    ExpectedPdo(ExpectedPdo),
    /// Invalid PDO index accessed, contains (requested, max)
    InvalidPdoIndex(InvalidPdoIndex),
}

impl<T: RoleCommon> TryFrom<[u8; LEN]> for RxCaps<T> {
    type Error = RxCapsError;

    fn try_from(raw: [u8; LEN]) -> Result<Self, Self::Error> {
        let raw = RxCapsRaw(raw);
        let num_valid_pdos = raw.num_valid_pdos() as usize;
        let num_valid_epr_pdos = raw.num_valid_epr_pdos() as usize;

        let mut pdos = [T::default(); TOTAL_PDOS];

        // Decode only valid SPR PDOs
        for (i, pdo) in pdos.iter_mut().enumerate().take(num_valid_pdos) {
            *pdo = T::try_from(match i {
                0 => raw.pdo0(),
                1 => raw.pdo1(),
                2 => raw.pdo2(),
                3 => raw.pdo3(),
                4 => raw.pdo4(),
                5 => raw.pdo5(),
                6 => raw.pdo6(),
                _ => {
                    return Err(RxCapsError::InvalidPdoIndex(InvalidPdoIndex {
                        requested: i,
                        max: NUM_SPR_PDOS,
                    }));
                }
            })
            .map_err(RxCapsError::ExpectedPdo)?;
        }

        // Decode only valid EPR PDOs
        for (i, pdo) in pdos
            .iter_mut()
            .skip(EPR_PDO_START_INDEX)
            .enumerate()
            .take(num_valid_epr_pdos)
        {
            *pdo = T::try_from(match i {
                0 => raw.epr_pdo0(),
                1 => raw.epr_pdo1(),
                2 => raw.epr_pdo2(),
                3 => raw.epr_pdo3(),
                _ => {
                    return Err(RxCapsError::InvalidPdoIndex(InvalidPdoIndex {
                        requested: i,
                        max: NUM_EPR_PDOS,
                    }));
                }
            })
            .map_err(RxCapsError::ExpectedPdo)?;
        }

        Ok(RxCaps {
            num_valid_pdos: raw.num_valid_pdos(),
            num_valid_epr_pdos: raw.num_valid_epr_pdos(),
            last_src_cap_is_epr: raw.last_src_cap_is_epr(),
            pdos,
        })
    }
}

pub type RxSrcCaps = RxCaps<source::Pdo>;
pub type RxSnkCaps = RxCaps<sink::Pdo>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::{
        TEST_SRC_APDO_INVALID_RAW, TEST_SRC_EPR_PDO_FIXED_28V1A5, TEST_SRC_EPR_PDO_FIXED_28V1A5_RAW,
        TEST_SRC_EPR_PDO_FIXED_28V3A, TEST_SRC_EPR_PDO_FIXED_28V3A_RAW, TEST_SRC_EPR_PDO_FIXED_28V5A,
        TEST_SRC_EPR_PDO_FIXED_28V5A_RAW, TEST_SRC_PDO_FIXED_5V1A5, TEST_SRC_PDO_FIXED_5V1A5_RAW,
        TEST_SRC_PDO_FIXED_5V3A, TEST_SRC_PDO_FIXED_5V3A_RAW, TEST_SRC_PDO_FIXED_5V900MA,
        TEST_SRC_PDO_FIXED_5V900MA_RAW, TEST_SRC_PDO_FIXED_9V1500MA, TEST_SRC_PDO_FIXED_9V1500MA_RAW,
        TEST_SRC_PDO_FIXED_9V3000MA, TEST_SRC_PDO_FIXED_9V3000MA_RAW,
    };

    #[test]
    fn test_try_from() {
        let mut buf = [0u8; LEN];
        // 5 SPR PDOs, 3 EPR PDOs, last received is EPR
        buf[0] = 0x5D;

        // Fill 5 SPR PDOs with distinct values
        buf[1..5].copy_from_slice(&TEST_SRC_PDO_FIXED_5V3A_RAW.to_le_bytes());
        buf[5..9].copy_from_slice(&TEST_SRC_PDO_FIXED_5V1A5_RAW.to_le_bytes());
        buf[9..13].copy_from_slice(&TEST_SRC_PDO_FIXED_5V900MA_RAW.to_le_bytes());
        buf[13..17].copy_from_slice(&TEST_SRC_PDO_FIXED_9V1500MA_RAW.to_le_bytes());
        buf[17..21].copy_from_slice(&TEST_SRC_PDO_FIXED_9V3000MA_RAW.to_le_bytes());
        // Make sure we don't attempt to parse beyond the end of valid SPR PDOs
        buf[21..25].copy_from_slice(&TEST_SRC_APDO_INVALID_RAW.to_le_bytes());

        // Fill 3 EPR PDOs
        buf[29..33].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V5A_RAW.to_le_bytes());
        buf[33..37].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V3A_RAW.to_le_bytes());
        buf[37..41].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V1A5_RAW.to_le_bytes());
        // Make sure we don't attempt to parse beyond the end of valid EPR PDOs
        buf[41..45].copy_from_slice(&TEST_SRC_APDO_INVALID_RAW.to_le_bytes());

        let rx_src_caps = RxSrcCaps::try_from(buf).unwrap();
        assert_eq!(rx_src_caps.num_valid_pdos(), 5);
        assert_eq!(rx_src_caps.num_valid_epr_pdos(), 3);
        assert!(rx_src_caps.last_src_cap_is_epr());

        // Verify PDO values are correct (not corrupted by header)
        assert_eq!(*rx_src_caps.get(0).unwrap(), TEST_SRC_PDO_FIXED_5V3A);
        assert_eq!(*rx_src_caps.get(1).unwrap(), TEST_SRC_PDO_FIXED_5V1A5);
        assert_eq!(*rx_src_caps.get(2).unwrap(), TEST_SRC_PDO_FIXED_5V900MA);
        assert_eq!(*rx_src_caps.get(3).unwrap(), TEST_SRC_PDO_FIXED_9V1500MA);
        assert_eq!(*rx_src_caps.get(4).unwrap(), TEST_SRC_PDO_FIXED_9V3000MA);
        assert_eq!(
            *rx_src_caps.get(EPR_PDO_START_INDEX).unwrap(),
            TEST_SRC_EPR_PDO_FIXED_28V5A
        );
        assert_eq!(
            *rx_src_caps.get(EPR_PDO_START_INDEX + 1).unwrap(),
            TEST_SRC_EPR_PDO_FIXED_28V3A
        );
        assert_eq!(
            *rx_src_caps.get(EPR_PDO_START_INDEX + 2).unwrap(),
            TEST_SRC_EPR_PDO_FIXED_28V1A5
        );
    }
}
