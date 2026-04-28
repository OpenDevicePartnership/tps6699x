//! Discovered SVIDs register (`0x21`).
//!
//! This register's size exceeds the maximum supported length by the [`device_driver`] crate.
//!
//! This register contains the SVID information returned from `Discover SVIDs REQ` messages.

use bitfield::bitfield;
use embedded_usb_pd::vdm::structured::Svid;

/// The address of the `Discovered SVIDs` register.
pub const ADDR: u8 = 0x21;

/// The length of the `Discovered SVIDs` register, in bytes.
///
/// This exceeds the maximum supported length by the [`device_driver`] crate.
pub const LEN: usize = 264 / 8;

bitfield! {
    /// Discovered SVIDs register.
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct Raw([u8]);
    impl Debug;

    /// Number of SVIDs discovered on SOP.
    pub u8, number_sop_svids, set_number_sop_svids: 3, 0;
    /// Number of SVIDs discovered on SOP'.
    pub u8, number_sop_prime_svids, set_number_sop_prime_svids: 7, 4;

    /// First SVID supported by SOP port partner.
    pub u16, svid_sop0, set_svid_sop0: 23, 8;

    /// Second SVID supported by SOP port partner.
    pub u16, svid_sop1, set_svid_sop1: 39, 24;

    /// Third SVID supported by SOP port partner.
    pub u16, svid_sop2, set_svid_sop2: 55, 40;

    /// Fourth SVID supported by SOP port partner.
    pub u16, svid_sop3, set_svid_sop3: 71, 56;

    /// Fifth SVID supported by SOP port partner.
    pub u16, svid_sop4, set_svid_sop4: 87, 72;

    /// Sixth SVID supported by SOP port partner.
    pub u16, svid_sop5, set_svid_sop5: 103, 88;

    /// Seventh SVID supported by SOP port partner.
    pub u16, svid_sop6, set_svid_sop6: 119, 104;

    /// Eighth SVID supported by SOP port partner.
    pub u16, svid_sop7, set_svid_sop7: 135, 120;

    /// First SVID supported by SOP' cable plug
    pub u16, svid_sop_prime0, set_svid_sop_prime0: 151, 136;

    /// Second SVID supported by SOP' cable plug
    pub u16, svid_sop_prime1, set_svid_sop_prime1: 167, 152;

    /// Third SVID supported by SOP' cable plug
    pub u16, svid_sop_prime2, set_svid_sop_prime2: 183, 168;

    /// Fourth SVID supported by SOP' cable plug
    pub u16, svid_sop_prime3, set_svid_sop_prime3: 199, 184;

    /// Fifth SVID supported by SOP' cable plug
    pub u16, svid_sop_prime4, set_svid_sop_prime4: 215, 200;

    /// Sixth SVID supported by SOP' cable plug
    pub u16, svid_sop_prime5, set_svid_sop_prime5: 231, 216;

    /// Seventh SVID supported by SOP' cable plug
    pub u16, svid_sop_prime6, set_svid_sop_prime6: 247, 232;

    /// Eighth SVID supported by SOP' cable plug
    pub u16, svid_sop_prime7, set_svid_sop_prime7: 263, 248;
}

/// Discovered SVIDs register, containing the SVID information returned from `Discover SVIDs REQ` messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DiscoveredSvids(Raw<[u8; LEN]>);

impl DiscoveredSvids {
    pub const DEFAULT: Self = Self(Raw([0; LEN]));

    /// Returns the number of SVIDs discovered on the SOP port partner.
    pub fn number_sop_svids(&self) -> usize {
        self.0.number_sop_svids() as usize
    }

    /// Returns an iterator over the SVIDs discovered on the SOP port partner.
    ///
    /// This will return at most 8 SVIDs, even if [`Self::number_sop_svids`] returns
    /// a larger number. Neither the data sheet nor the USB PD specification explicitly
    /// state that there can be at most 8 SVIDs, but the register only has space
    /// for 8.
    pub fn svid_sop(&self) -> impl ExactSizeIterator<Item = Svid> {
        [
            self.0.svid_sop0(),
            self.0.svid_sop1(),
            self.0.svid_sop2(),
            self.0.svid_sop3(),
            self.0.svid_sop4(),
            self.0.svid_sop5(),
            self.0.svid_sop6(),
            self.0.svid_sop7(),
        ]
        .into_iter()
        .take(self.number_sop_svids())
        .map(Svid)
    }

    /// Returns the number of SVIDs discovered on the SOP' cable plug.
    pub fn number_sop_prime_svids(&self) -> usize {
        self.0.number_sop_prime_svids() as usize
    }

    /// Returns an iterator over the SVIDs discovered on the SOP' cable plug.
    ///
    /// This will return at most 8 SVIDs, even if [`Self::number_sop_prime_svids`]
    /// returns a larger number. Neither the data sheet nor the USB PD specification
    /// explicitly state that there can be at most 8 SVIDs, but the register only
    /// has space for 8.
    pub fn svid_sop_prime(&self) -> impl ExactSizeIterator<Item = Svid> {
        [
            self.0.svid_sop_prime0(),
            self.0.svid_sop_prime1(),
            self.0.svid_sop_prime2(),
            self.0.svid_sop_prime3(),
            self.0.svid_sop_prime4(),
            self.0.svid_sop_prime5(),
            self.0.svid_sop_prime6(),
            self.0.svid_sop_prime7(),
        ]
        .into_iter()
        .take(self.number_sop_prime_svids())
        .map(Svid)
    }
}

impl Default for DiscoveredSvids {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<[u8; LEN]> for DiscoveredSvids {
    fn from(raw: [u8; LEN]) -> Self {
        Self(Raw(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    #[test]
    fn impl_iterators() {
        let mut reg = DiscoveredSvids::default();

        reg.0.set_number_sop_svids(3);
        reg.0.set_svid_sop0(0x1234);
        reg.0.set_svid_sop1(0x5678);
        reg.0.set_svid_sop2(0x9ABC);

        reg.0.set_number_sop_prime_svids(2);
        reg.0.set_svid_sop_prime0(0xDEF0);
        reg.0.set_svid_sop_prime1(0xFFFF);

        assert_eq!(reg.number_sop_svids(), 3);
        assert_eq!(
            reg.svid_sop().collect::<Vec<_>>(),
            vec![Svid(0x1234), Svid(0x5678), Svid(0x9ABC)]
        );

        assert_eq!(reg.number_sop_prime_svids(), 2);
        assert_eq!(
            reg.svid_sop_prime().collect::<Vec<_>>(),
            vec![Svid(0xDEF0), Svid(0xFFFF)]
        );
    }

    #[test]
    fn default_has_no_svids() {
        let reg = DiscoveredSvids::default();
        assert_eq!(reg.number_sop_svids(), 0);
        assert_eq!(reg.svid_sop().len(), 0);

        assert_eq!(reg.number_sop_prime_svids(), 0);
        assert_eq!(reg.svid_sop_prime().len(), 0);
    }

    /// There's only space fo r8 SVIDs in the register but the number of SVIDs could
    /// be larger, so the iterators should clamp to 8.
    #[test]
    fn iterators_clamp_to_8() {
        let mut reg = DiscoveredSvids::default();

        reg.0.set_number_sop_svids(9);
        reg.0.set_number_sop_prime_svids(10);
        assert_eq!(reg.number_sop_svids(), 9);
        assert_eq!(reg.number_sop_prime_svids(), 10);
        assert_eq!(reg.svid_sop().len(), 8);
        assert_eq!(reg.svid_sop_prime().len(), 8);
    }
}
