//! Higher-level wrapper for the rx src/sink caps register.

use embedded_usb_pd::pdo::{self, sink, source, Common};

use crate::registers::rx_caps::{NUM_EPR_PDOS, NUM_SPR_PDOS};

/// Higher-level wrapper for the rx src/sink caps register.
pub struct RxCaps<T: Common> {
    pub(super) spr: heapless::Vec<T, NUM_SPR_PDOS>,
    pub(super) epr: heapless::Vec<T, NUM_EPR_PDOS>,
}

impl<T: Common> RxCaps<T> {
    /// Return a slice of all SPR PDOs.
    pub fn spr_as_slice(&self) -> &[T] {
        self.spr.as_slice()
    }

    /// Return a mutable slice of all SPR PDOs.
    pub fn spr_as_mut_slice(&mut self) -> &mut [T] {
        self.spr.as_mut_slice()
    }

    /// Return a slice of all EPR PDOs.
    pub fn epr_as_slice(&self) -> &[T] {
        self.epr.as_slice()
    }

    /// Return if any PDOs are present.
    pub fn is_empty(&self) -> bool {
        self.spr.len() + self.epr.len() == 0
    }

    /// Return a mutable slice of all EPR PDOs.
    pub fn epr_as_mut_slice(&mut self) -> &mut [T] {
        self.epr.as_mut_slice()
    }

    /// Iterator over all PDOs
    pub fn iter(&self) -> impl Iterator<Item = &'_ T> {
        self.spr.iter().chain(self.epr.iter())
    }

    /// Iterator over all PDOs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut T> {
        self.spr.iter_mut().chain(self.epr.iter_mut())
    }

    /// Iterator over all SPR PDOs
    pub fn spr_iter(&self) -> impl Iterator<Item = &'_ T> {
        self.spr.iter()
    }

    /// Iterator over all SPR PDOs
    pub fn spr_iter_mut(&mut self) -> impl Iterator<Item = &'_ mut T> {
        self.spr.iter_mut()
    }

    /// Iterator over all EPR PDOs
    pub fn epr_iter(&self) -> impl Iterator<Item = &'_ T> {
        self.epr.iter()
    }

    /// Iterator over all EPR PDOs
    pub fn epr_iter_mut(&mut self) -> impl Iterator<Item = &'_ mut T> {
        self.epr.iter_mut()
    }
}

// These are specialized to avoid a duplicate From<RxCaps<pdo::Pdo>> for RxCaps<pdo::Pdo>
impl From<RxCaps<source::Pdo>> for RxCaps<pdo::Pdo> {
    fn from(value: RxCaps<source::Pdo>) -> Self {
        RxCaps {
            spr: value.spr_as_slice().iter().map(|pdo| pdo.clone().into()).collect(),
            epr: value.epr_as_slice().iter().map(|pdo| pdo.clone().into()).collect(),
        }
    }
}

impl From<RxCaps<sink::Pdo>> for RxCaps<pdo::Pdo> {
    fn from(value: RxCaps<sink::Pdo>) -> Self {
        RxCaps {
            spr: value.spr_as_slice().iter().map(|pdo| pdo.clone().into()).collect(),
            epr: value.epr_as_slice().iter().map(|pdo| pdo.clone().into()).collect(),
        }
    }
}

pub type RxSrcCaps = RxCaps<source::Pdo>;
pub type RxSnkCaps = RxCaps<sink::Pdo>;
