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
            spr: value.spr_as_slice().iter().map(|&pdo| pdo.into()).collect(),
            epr: value.epr_as_slice().iter().map(|&pdo| pdo.into()).collect(),
        }
    }
}

impl From<RxCaps<sink::Pdo>> for RxCaps<pdo::Pdo> {
    fn from(value: RxCaps<sink::Pdo>) -> Self {
        RxCaps {
            spr: value.spr_as_slice().iter().map(|&pdo| pdo.into()).collect(),
            epr: value.epr_as_slice().iter().map(|&pdo| pdo.into()).collect(),
        }
    }
}

pub type RxSrcCaps = RxCaps<source::Pdo>;
pub type RxSnkCaps = RxCaps<sink::Pdo>;

#[cfg(test)]
mod tests {
    use embedded_usb_pd::pdo;
    use embedded_usb_pd::pdo::sink::FixedData as SnkFixedData;
    use embedded_usb_pd::pdo::source::FixedData as SrcFixedData;

    use super::*;

    fn make_src_caps() -> RxCaps<source::Pdo> {
        let spr = source::Pdo::Fixed(SrcFixedData {
            flags: Default::default(),
            voltage_mv: 5000,
            current_ma: 3000,
            peak_current: source::PeakCurrent::Pct100,
        });
        let epr0 = source::Pdo::Fixed(SrcFixedData {
            flags: Default::default(),
            voltage_mv: 28000,
            current_ma: 3000,
            peak_current: source::PeakCurrent::Pct100,
        });
        let epr1 = source::Pdo::Fixed(SrcFixedData {
            flags: Default::default(),
            voltage_mv: 28000,
            current_ma: 5000,
            peak_current: source::PeakCurrent::Pct100,
        });
        RxCaps {
            spr: heapless::Vec::from_iter([spr]),
            epr: heapless::Vec::from_iter([epr0, epr1]),
        }
    }

    fn make_snk_caps() -> RxCaps<sink::Pdo> {
        let spr0 = sink::Pdo::Fixed(SnkFixedData {
            dual_role_power: false,
            higher_capability: false,
            unconstrained_power: false,
            usb_comms_capable: false,
            dual_role_data: false,
            frs_required_current: sink::FrsRequiredCurrent::None,
            voltage_mv: 5000,
            operational_current_ma: 900,
        });
        let spr1 = sink::Pdo::Fixed(SnkFixedData {
            dual_role_power: false,
            higher_capability: false,
            unconstrained_power: false,
            usb_comms_capable: false,
            dual_role_data: false,
            frs_required_current: sink::FrsRequiredCurrent::None,
            voltage_mv: 5000,
            operational_current_ma: 3000,
        });
        let epr = sink::Pdo::Fixed(SnkFixedData {
            dual_role_power: false,
            higher_capability: false,
            unconstrained_power: false,
            usb_comms_capable: false,
            dual_role_data: false,
            frs_required_current: sink::FrsRequiredCurrent::None,
            voltage_mv: 20000,
            operational_current_ma: 3000,
        });
        RxCaps {
            spr: heapless::Vec::from_iter([spr0, spr1]),
            epr: heapless::Vec::from_iter([epr]),
        }
    }

    #[test]
    fn test_from_rx_src_caps_for_pdo() {
        let src = make_src_caps();
        let generic: RxCaps<pdo::Pdo> = src.into();

        assert_eq!(generic.spr.len(), 1);
        assert_eq!(generic.epr.len(), 2);
        assert_eq!(
            generic.spr[0],
            pdo::Pdo::Source(source::Pdo::Fixed(SrcFixedData {
                flags: Default::default(),
                voltage_mv: 5000,
                current_ma: 3000,
                peak_current: source::PeakCurrent::Pct100,
            }))
        );
        assert_eq!(
            generic.epr[0],
            pdo::Pdo::Source(source::Pdo::Fixed(SrcFixedData {
                flags: Default::default(),
                voltage_mv: 28000,
                current_ma: 3000,
                peak_current: source::PeakCurrent::Pct100,
            }))
        );
        assert_eq!(
            generic.epr[1],
            pdo::Pdo::Source(source::Pdo::Fixed(SrcFixedData {
                flags: Default::default(),
                voltage_mv: 28000,
                current_ma: 5000,
                peak_current: source::PeakCurrent::Pct100,
            }))
        );
    }

    #[test]
    fn test_from_rx_snk_caps_for_pdo() {
        let snk = make_snk_caps();
        let generic: RxCaps<pdo::Pdo> = snk.into();

        assert_eq!(generic.spr.len(), 2);
        assert_eq!(generic.epr.len(), 1);
        assert_eq!(
            generic.spr[0],
            pdo::Pdo::Sink(sink::Pdo::Fixed(SnkFixedData {
                dual_role_power: false,
                higher_capability: false,
                unconstrained_power: false,
                usb_comms_capable: false,
                dual_role_data: false,
                frs_required_current: sink::FrsRequiredCurrent::None,
                voltage_mv: 5000,
                operational_current_ma: 900,
            }))
        );
        assert_eq!(
            generic.spr[1],
            pdo::Pdo::Sink(sink::Pdo::Fixed(SnkFixedData {
                dual_role_power: false,
                higher_capability: false,
                unconstrained_power: false,
                usb_comms_capable: false,
                dual_role_data: false,
                frs_required_current: sink::FrsRequiredCurrent::None,
                voltage_mv: 5000,
                operational_current_ma: 3000,
            }))
        );
        assert_eq!(
            generic.epr[0],
            pdo::Pdo::Sink(sink::Pdo::Fixed(SnkFixedData {
                dual_role_power: false,
                higher_capability: false,
                unconstrained_power: false,
                usb_comms_capable: false,
                dual_role_data: false,
                frs_required_current: sink::FrsRequiredCurrent::None,
                voltage_mv: 20000,
                operational_current_ma: 3000,
            }))
        );
    }
}
