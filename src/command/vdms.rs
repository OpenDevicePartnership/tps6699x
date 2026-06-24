//! `VDMs`: Repeat transactions on I2C3m under certain conditions.

use bitfield::bitfield;

use crate::error;

pub const INPUT_LEN: usize = 31;
pub const INITIATOR_WAIT_TIME_MS: u8 = 100;
pub const MAX_NUM_DATA_OBJECTS: usize = 7;

#[derive(Debug, Clone, Copy)]
pub enum SopTarget {
    /// SOP'
    Sop,
    /// SOP''
    SopPrime,
    /// SOP'''
    SopDoublePrime,
    /// SOP'_Debug for Source, SOP''_Debug for sink.
    SopDebug,
}
impl From<SopTarget> for u8 {
    fn from(value: SopTarget) -> Self {
        match value {
            SopTarget::Sop => 0,
            SopTarget::SopPrime => 1,
            SopTarget::SopDoublePrime => 2,
            SopTarget::SopDebug => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Version {
    /// VDMs version 1 ignores [`Input::initiator_wait_timer`], always waiting 30ms for a response.
    One,
    /// VDMs version 2 uses [`Input::initiator_wait_timer`].
    Two,
}

impl From<Version> for bool {
    fn from(value: Version) -> Self {
        match value {
            Version::One => false,
            Version::Two => true,
        }
    }
}

bitfield! {
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct InputRaw([u8]);
    impl Debug;

    /// Number of VDOs to transmit
    pub u8, num_vdo, set_num_vdo: 2, 0;
    /// Version of the VDMs command
    pub bool, version, set_version: 3;
    /// SOP Target of the message in the register
    pub u8, sop_target, set_sop_target: 5, 4;
    /// When set, PD will stop sending BUSY response to the last received SVMD command
    pub u8, am_intrusive_mode, set_am_intrusive_mode: 7;
    /// Data object 1
    pub u32, vdo1, set_vdo1: 39, 8;
    /// Data object 2
    pub u32, vdo2, set_vdo2: 71, 40;
    /// Data object 3
    pub u32, vdo3, set_vdo3: 103, 72;
    /// Data object 4
    pub u32, vdo4, set_vdo4: 135, 104;
    /// Data object 5
    pub u32, vdo5, set_vdo5: 167, 136;
    /// Data object 6
    pub u32, vdo6, set_vdo6: 199, 168;
    /// Data object 7
    pub u32, vdo7, set_vdo7: 231, 200;
    /// Initiator or Responder. false: response, true: initiating a VDM
    pub bool, initiator, set_initiator: 232;
    /// Initiator Wait State Timer (in milliseconds) if the Initiator_Responder is set to true
    pub u8, initiator_wait_timer, set_initiator_wait_timer: 247, 240;
}

pub struct Input(InputRaw<[u8; INPUT_LEN]>);
impl Input {
    pub fn new() -> Self {
        Self(InputRaw([0; INPUT_LEN]))
    }

    pub fn as_bytes(&self) -> &[u8; INPUT_LEN] {
        &self.0.0
    }

    pub fn set_num_vdo(&mut self, num: u8) {
        let num = num.min(MAX_NUM_DATA_OBJECTS as u8);
        self.0.set_num_vdo(num);
    }

    pub fn set_version(&mut self, version: Version) {
        self.0.set_version(version.into());
    }

    pub fn set_sop_target(&mut self, sop_target: SopTarget) {
        self.0.set_sop_target(sop_target.into());
    }

    pub fn set_am_intrusive_mode(&mut self, mode: bool) {
        self.0.set_am_intrusive_mode(mode);
    }

    pub fn set_vdo(&mut self, index: usize, vdo: u32) {
        match index {
            0 => self.0.set_vdo1(vdo),
            1 => self.0.set_vdo2(vdo),
            2 => self.0.set_vdo3(vdo),
            3 => self.0.set_vdo4(vdo),
            4 => self.0.set_vdo5(vdo),
            5 => self.0.set_vdo6(vdo),
            6 => self.0.set_vdo7(vdo),
            _ => error!("Index out of bounds for VDOs"),
        }
    }

    pub fn set_initiator(&mut self, initiator: bool) {
        self.0.set_initiator(initiator);
    }

    pub fn set_initiator_wait_timer(&mut self, timer: u8) {
        self.0.set_initiator_wait_timer(timer);
    }
}

impl From<Input> for [u8; INPUT_LEN] {
    fn from(value: Input) -> Self {
        value.0.0
    }
}

impl From<[u8; INPUT_LEN]> for Input {
    fn from(value: [u8; INPUT_LEN]) -> Self {
        Input(InputRaw(value))
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_nonzero_roundtrip() {
        let mut input = Input::new();

        input.set_num_vdo(7);
        input.set_version(Version::Two);
        input.set_sop_target(SopTarget::SopDoublePrime);
        input.set_am_intrusive_mode(true);
        input.set_vdo(0, 0xDEADBEEF);
        input.set_vdo(1, 0xA1B2C3D4);
        input.set_vdo(2, 0x12345678);
        input.set_vdo(3, 0x9ABCDEF0);
        input.set_vdo(4, 0x13579BDF);
        input.set_vdo(5, 0x2468ACE0);
        input.set_vdo(6, 0xFDB97531);
        input.set_initiator(true);
        input.set_initiator_wait_timer(0xAB);

        // Byte 0: num_vdo(2:0)=7, version(3)=1, sop_target(5:4)=2, am_intrusive(7)=1
        const EXPECTED: [u8; INPUT_LEN] = [
            0xAF, 0xEF, 0xBE, 0xAD, 0xDE, 0xD4, 0xC3, 0xB2, 0xA1, 0x78, 0x56, 0x34, 0x12, 0xF0, 0xDE, 0xBC, 0x9A, 0xDF,
            0x9B, 0x57, 0x13, 0xE0, 0xAC, 0x68, 0x24, 0x31, 0x75, 0xB9, 0xFD, 0x01, 0xAB,
        ];
        assert_eq!(input.as_bytes(), &EXPECTED);

        // Reconstruct from bytes and verify all getters
        let input2 = Input::from(EXPECTED);
        assert_eq!(input2.0.num_vdo(), 7);
        assert!(input2.0.version());
        assert_eq!(input2.0.sop_target(), 2);
        assert!(input2.0.am_intrusive_mode());
        assert_eq!(input2.0.vdo1(), 0xDEADBEEF);
        assert_eq!(input2.0.vdo2(), 0xA1B2C3D4);
        assert_eq!(input2.0.vdo3(), 0x12345678);
        assert_eq!(input2.0.vdo4(), 0x9ABCDEF0);
        assert_eq!(input2.0.vdo5(), 0x13579BDF);
        assert_eq!(input2.0.vdo6(), 0x2468ACE0);
        assert_eq!(input2.0.vdo7(), 0xFDB97531);
        assert!(input2.0.initiator());
        assert_eq!(input2.0.initiator_wait_timer(), 0xAB);
    }

    #[test]
    fn test_sop_target_from_conversions() {
        assert_eq!(u8::from(SopTarget::Sop), 0);
        assert_eq!(u8::from(SopTarget::SopPrime), 1);
        assert_eq!(u8::from(SopTarget::SopDoublePrime), 2);
        assert_eq!(u8::from(SopTarget::SopDebug), 3);
    }

    #[test]
    fn test_version_from_conversions() {
        assert!(!bool::from(Version::One));
        assert!(bool::from(Version::Two));
    }
}
