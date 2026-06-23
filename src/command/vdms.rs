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
