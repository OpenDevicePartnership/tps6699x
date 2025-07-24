use core::ops::{Index, IndexMut};

use bitfield::bitfield;
use embedded_usb_pd::{pdo::source::Pdo, InvalidData};

/// Register address
pub const ADDR: u8 = 0x30;

/// Length of the register in bytes
pub const LEN: usize = 45;

/// Total number of PDOs supported
pub const TOTAL_PDOS: usize = 11;

/// Total length in bytes of the register header
/// [`RxSrcCapsRaw::num_valid_pdos`], [`RxSrcCapsRaw::num_valid_epr_pdos`] and [`RxSrcCapsRaw::last_src_cap_is_epr`]
pub const HEADER_LEN: usize = 1;

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
    ///Last Src Cap Received is EPR
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
    type Error = InvalidData;

    fn try_from(raw: [u8; LEN]) -> Result<Self, Self::Error> {
        let raw = RxSrcCapsRaw(raw);
        Ok(RxSrcCaps {
            num_valid_pdos: raw.num_valid_pdos(),
            num_valid_epr_pdos: raw.num_valid_epr_pdos(),
            last_src_cap_is_epr: raw.last_src_cap_is_epr(),
            pdos: [
                Pdo::try_from(raw.pdo0())?,
                Pdo::try_from(raw.pdo1())?,
                Pdo::try_from(raw.pdo2())?,
                Pdo::try_from(raw.pdo3())?,
                Pdo::try_from(raw.pdo4())?,
                Pdo::try_from(raw.pdo5())?,
                Pdo::try_from(raw.pdo6())?,
                Pdo::try_from(raw.epr_pdo0())?,
                Pdo::try_from(raw.epr_pdo1())?,
                Pdo::try_from(raw.epr_pdo2())?,
                Pdo::try_from(raw.epr_pdo3())?,
            ],
        })
    }
}
