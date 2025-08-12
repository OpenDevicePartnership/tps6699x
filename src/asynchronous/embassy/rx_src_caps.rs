//! Higher-level wrapper for the rx src caps register.

use embedded_usb_pd::pdo::source::Pdo;

use crate::registers::rx_src_caps::{NUM_EPR_PDOS, NUM_SPR_PDOS};

/// Higher-level wrapper for the rx src caps register.
pub struct RxSrcCaps {
    pub(super) spr: heapless::Vec<Pdo, NUM_SPR_PDOS>,
    pub(super) epr: heapless::Vec<Pdo, NUM_EPR_PDOS>,
}

impl RxSrcCaps {
    /// Return a slice of all SPR PDOs.
    pub fn spr_as_slice(&self) -> &[Pdo] {
        self.spr.as_slice()
    }

    /// Return a mutable slice of all SPR PDOs.
    pub fn spr_as_mut_slice(&mut self) -> &mut [Pdo] {
        self.spr.as_mut_slice()
    }

    /// Return a slice of all EPR PDOs.
    pub fn epr_as_slice(&self) -> &[Pdo] {
        self.epr.as_slice()
    }

    /// Return if any PDOs are present.
    pub fn is_empty(&self) -> bool {
        self.spr.len() + self.epr.len() == 0
    }

    /// Return a mutable slice of all EPR PDOs.
    pub fn epr_as_mut_slice(&mut self) -> &mut [Pdo] {
        self.epr.as_mut_slice()
    }

    /// Iterator over all PDOs
    pub fn iter(&self) -> impl Iterator<Item = &'_ Pdo> {
        self.spr.iter().chain(self.epr.iter())
    }

    /// Iterator over all PDOs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut Pdo> {
        self.spr.iter_mut().chain(self.epr.iter_mut())
    }

    /// Iterator over all SPR PDOs
    pub fn spr_iter(&self) -> impl Iterator<Item = &'_ Pdo> {
        self.spr.iter()
    }

    /// Iterator over all SPR PDOs
    pub fn spr_iter_mut(&mut self) -> impl Iterator<Item = &'_ mut Pdo> {
        self.spr.iter_mut()
    }

    /// Iterator over all EPR PDOs
    pub fn epr_iter(&self) -> impl Iterator<Item = &'_ Pdo> {
        self.epr.iter()
    }

    /// Iterator over all EPR PDOs
    pub fn epr_iter_mut(&mut self) -> impl Iterator<Item = &'_ mut Pdo> {
        self.epr.iter_mut()
    }
}
