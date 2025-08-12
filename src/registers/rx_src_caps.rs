use core::ops::{Index, IndexMut};

use bitfield::bitfield;
use embedded_usb_pd::pdo::source::Pdo;
use embedded_usb_pd::pdo::ExpectedPdo;

/// Register address
pub const ADDR: u8 = 0x30;

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
    /// Received source capabilities register
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct RxSrcCapsRaw([u8]);
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
pub struct RxSrcCaps {
    /// Number of valid standard PDOs
    num_valid_pdos: u8,
    /// Number of valid EPR PDOs
    num_valid_epr_pdos: u8,
    /// Last source capabilities received is EPR
    last_src_cap_is_epr: bool,
    /// PDOs
    pdos: [Pdo; TOTAL_PDOS],
}

impl RxSrcCaps {
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
}

impl Index<usize> for RxSrcCaps {
    type Output = Pdo;

    fn index(&self, index: usize) -> &Self::Output {
        if index < TOTAL_PDOS {
            &self.pdos[index]
        } else {
            panic!("Index out of bounds: {}", index);
        }
    }
}

impl IndexMut<usize> for RxSrcCaps {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < TOTAL_PDOS {
            &mut self.pdos[index]
        } else {
            panic!("Index out of bounds: {}", index);
        }
    }
}

impl TryFrom<[u8; LEN]> for RxSrcCaps {
    type Error = ExpectedPdo;

    fn try_from(raw: [u8; LEN]) -> Result<Self, Self::Error> {
        let raw = RxSrcCapsRaw(raw);
        let num_valid_pdos = raw.num_valid_pdos() as usize;
        let num_valid_epr_pdos = raw.num_valid_epr_pdos() as usize;

        let mut pdos = [Pdo::default(); TOTAL_PDOS];

        // Decode only valid SPR PDOs
        for (i, pdo) in pdos.iter_mut().enumerate().take(num_valid_pdos) {
            *pdo = Pdo::try_from(match i {
                0 => raw.pdo0(),
                1 => raw.pdo1(),
                2 => raw.pdo2(),
                3 => raw.pdo3(),
                4 => raw.pdo4(),
                5 => raw.pdo5(),
                6 => raw.pdo6(),
                _ => unreachable!(),
            })?;
        }

        // Decode only valid EPR PDOs
        for (i, pdo) in pdos
            .iter_mut()
            .skip(EPR_PDO_START_INDEX)
            .enumerate()
            .take(num_valid_epr_pdos)
        {
            *pdo = Pdo::try_from(match i {
                0 => raw.epr_pdo0(),
                1 => raw.epr_pdo1(),
                2 => raw.epr_pdo2(),
                3 => raw.epr_pdo3(),
                _ => unreachable!(),
            })?;
        }

        Ok(RxSrcCaps {
            num_valid_pdos: raw.num_valid_pdos(),
            num_valid_epr_pdos: raw.num_valid_epr_pdos(),
            last_src_cap_is_epr: raw.last_src_cap_is_epr(),
            pdos,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::{
        TEST_SRC_APDO_INVALID_RAW, TEST_SRC_EPR_PDO_FIXED_28V5A, TEST_SRC_EPR_PDO_FIXED_28V5A_RAW,
        TEST_SRC_PDO_FIXED_5V1A5, TEST_SRC_PDO_FIXED_5V1A5_RAW, TEST_SRC_PDO_FIXED_5V3A, TEST_SRC_PDO_FIXED_5V3A_RAW,
    };

    #[test]
    fn test_try_from() {
        let mut buf = [0u8; LEN];
        // Set header: low 3 bits are SPR PDO count
        buf[0] = 0xa; // 2 SPR PDOs, 1 EPR PDOs
                      // Fill PDOs with test data
                      // SPR PDO 0 - Fixed PDO at 5V, 3A, 100% peak current
        buf[1..5].copy_from_slice(&TEST_SRC_PDO_FIXED_5V3A_RAW.to_le_bytes());
        // SPR PDO 1 - Fixed PDO at 5V, 1.5A, 100% peak current
        buf[5..9].copy_from_slice(&TEST_SRC_PDO_FIXED_5V1A5_RAW.to_le_bytes());
        // Fake SPR, used to test overread
        buf[9..13].copy_from_slice(&TEST_SRC_APDO_INVALID_RAW.to_le_bytes());
        // EPR PDO 0 - Fixed PDO at 28V, 5A, 100% peak current
        buf[29..33].copy_from_slice(&TEST_SRC_EPR_PDO_FIXED_28V5A_RAW.to_le_bytes());
        // Fake EPR, used to test overread
        buf[33..37].copy_from_slice(&TEST_SRC_APDO_INVALID_RAW.to_le_bytes());
        // Fake EPR, used to test overread
        buf[37..41].copy_from_slice(&TEST_SRC_APDO_INVALID_RAW.to_le_bytes());

        let rx_src_caps = RxSrcCaps::try_from(buf).unwrap();
        assert_eq!(rx_src_caps.num_valid_pdos(), 2);
        assert_eq!(rx_src_caps.num_valid_epr_pdos(), 1);
        assert_eq!(rx_src_caps[0], TEST_SRC_PDO_FIXED_5V3A);
        assert_eq!(rx_src_caps[1], TEST_SRC_PDO_FIXED_5V1A5);
        assert_eq!(rx_src_caps[EPR_PDO_START_INDEX], TEST_SRC_EPR_PDO_FIXED_28V5A);
    }
}
