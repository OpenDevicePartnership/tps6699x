//! Rx Other Vdm register (`0x61`).
//!
//! This register receives the user svid other vdm.
//!

use bitfield::bitfield;

/// The address of the `Rx Other Vdm` register.
pub const ADDR: u8 = 0x61;

/// The length of the `Rx Other Vdm` register, in bytes.
pub const LEN: usize = 29;

bitfield! {
    /// Raw bytes for the Received User SVID Other VDM register.
    ///
    /// Each field corresponds to a bit or group of bits in the register.
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct RxOtherVdmRaw([u8]);
    impl Debug;

    /// Rx Vdm data object 7
    pub u32, vdo7, set_vdo7: 231, 200;
    /// Rx Vdm data object 6
    pub u32, vdo6, set_vdo6: 199, 168;
    /// Rx Vdm data object 5
    pub u32, vdo5, set_vdo5: 167, 136;
    /// Rx Vdm data object 4
    pub u32, vdo4, set_vdo4: 135, 104;
    /// Rx Vdm data object 3
    pub u32, vdo3, set_vdo3: 103, 72;
    /// Rx Vdm data object 2
    pub u32, vdo2, set_vdo2: 71, 40;
    /// Rx Vdm data object 1
    pub u32, vdo1, set_vdo1: 39, 8;
    /// Sequence Number
    pub u8, sequence_number, set_sequence_number: 7,5;
    /// Frame Type of the message in the register
    pub u8, sop_type, set_sop_type: 4, 3;
    /// Number of valid VDOs
    pub u8, num_of_valid_vdos, set_num_of_valid_vdos: 2, 0;
}

/// The port configuration register.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RxOtherVdm(RxOtherVdmRaw<[u8; LEN]>);

impl RxOtherVdm {
    /// The default bytes for the rx other vdm register.
    const DEFAULT: [u8; LEN] = [0u8; LEN];

    /// Get the raw byte representation of the port configuration register.
    pub fn as_bytes(&self) -> &[u8; LEN] {
        &self.0 .0
    }

    /*
    pub fn typec_state_machine(&self) -> TypeCStateMachine {
        self.0.typec_state_machine().into()
    }

    pub fn set_typec_state_machine(&mut self, value: TypeCStateMachine) -> &mut Self {
        self.0.set_typec_state_machine(value.into());
        self
    }
    */
}

impl From<[u8; LEN]> for RxOtherVdm {
    fn from(value: [u8; LEN]) -> Self {
        RxOtherVdm(RxOtherVdmRaw(value))
    }
}

impl From<RxOtherVdm> for [u8; LEN] {
    fn from(value: RxOtherVdm) -> Self {
        value.0 .0
    }
}

impl Default for RxOtherVdm {
    fn default() -> Self {
        Self::DEFAULT.into()
    }
}
