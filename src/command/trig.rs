//! The `Trig` command.

use bincode::enc::Encoder;
use bincode::error::EncodeError;
use bincode::Encode;

/// The length of the arguments for the `Trig` command.
#[allow(dead_code)]
pub(crate) const ARGS_LEN: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Cmd {
    FaultInputPort1 = 0x21,
    FaultInputPort2 = 0x22,
    RetimerForcePwr = 0x2A,
    RetimerHighCurrentContract = 0x2F,
    I3cMasterIrq = 0x38,
    Mreset = 0x45,
}

impl Encode for Cmd {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let val = *self as u8;
        Encode::encode(&val, encoder)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Edge {
    /// Trig Vgpio falling edge
    Falling = 0,
    /// Trig Vgpio rising edge
    Rising = 1,
}

impl Encode for Edge {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let val = *self as u8;
        Encode::encode(&val, encoder)
    }
}

#[derive(Debug, Encode, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args {
    pub edge: Edge,
    pub cmd: Cmd,
}
