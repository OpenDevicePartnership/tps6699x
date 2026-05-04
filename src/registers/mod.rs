use embedded_usb_pd::type_c::ConnectionState;
use embedded_usb_pd::{type_c, PdError};

use crate::Mode;

pub mod autonegotiate_sink;
pub mod boot_flags;
pub mod dp_status;
pub mod port_config;
pub mod rx_caps;
pub mod rx_other_vdm;
pub mod tx_identity;

// Generated register definitions from device.yaml
// Skip format checking for generated code
#[rustfmt::skip]
#[allow(clippy::unreachable)]
#[allow(clippy::identity_op)]
#[allow(clippy::erasing_op)]
#[allow(clippy::unnecessary_cast)]
#[allow(clippy::new_without_default)]
#[allow(clippy::let_and_return)]
#[allow(clippy::unnecessary_fallible_conversions)]
#[allow(clippy::empty_docs)]
#[allow(unused_imports)]
#[allow(unreachable_patterns)]
mod generated;
pub use generated::*;

/// Command data 1 register
/// This register is 512 bits and exceeds the maximum support by device_driver
pub const REG_DATA1: u8 = 0x09;
// Command data 1 register length
pub const REG_DATA1_LEN: usize = 64;

impl TryFrom<TypecCurrent> for type_c::Current {
    type Error = PdError;

    fn try_from(value: TypecCurrent) -> Result<Self, Self::Error> {
        match value {
            TypecCurrent::UsbDefault => Ok(type_c::Current::UsbDefault),
            TypecCurrent::Current1A5 => Ok(type_c::Current::Current1A5),
            TypecCurrent::Current3A0 => Ok(type_c::Current::Current3A0),
            _ => Err(PdError::InvalidParams),
        }
    }
}

impl From<type_c::Current> for TypecCurrent {
    fn from(value: type_c::Current) -> Self {
        match value {
            type_c::Current::UsbDefault => TypecCurrent::UsbDefault,
            type_c::Current::Current1A5 => TypecCurrent::Current1A5,
            type_c::Current::Current3A0 => TypecCurrent::Current3A0,
        }
    }
}

impl TryFrom<PdCcPullUp> for type_c::Current {
    type Error = PdError;

    fn try_from(value: PdCcPullUp) -> Result<Self, Self::Error> {
        match value {
            PdCcPullUp::UsbDefault => Ok(type_c::Current::UsbDefault),
            PdCcPullUp::Current1A5 => Ok(type_c::Current::Current1A5),
            PdCcPullUp::Current3A0 => Ok(type_c::Current::Current3A0),
            _ => Err(PdError::InvalidParams),
        }
    }
}

impl From<type_c::Current> for PdCcPullUp {
    fn from(value: type_c::Current) -> Self {
        match value {
            type_c::Current::UsbDefault => PdCcPullUp::UsbDefault,
            type_c::Current::Current1A5 => PdCcPullUp::Current1A5,
            type_c::Current::Current3A0 => PdCcPullUp::Current3A0,
        }
    }
}

impl TryFrom<PlugMode> for ConnectionState {
    type Error = PdError;

    fn try_from(value: PlugMode) -> Result<Self, Self::Error> {
        match value {
            PlugMode::Debug => Ok(ConnectionState::DebugAccessory),
            PlugMode::Audio => Ok(ConnectionState::AudioAccessory),
            PlugMode::Connected | PlugMode::ConnectedNoRa => Ok(ConnectionState::Attached),
            _ => Err(PdError::InvalidParams),
        }
    }
}

impl From<Mode> for &str {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Boot => "BOOT",
            Mode::F211 => "F211",
            Mode::App0 => "APP0",
            Mode::App1 => "APP1",
            Mode::Wtpr => "WTPR",
        }
    }
}

impl field_sets::IntEventBus1 {
    /// Create an IntEventBus1 with all bits set to 1
    pub fn all() -> Self {
        field_sets::IntEventBus1::from([0xFF; 11])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_type_c_current() {
        let current: type_c::Current = TypecCurrent::UsbDefault.try_into().unwrap();
        assert_eq!(current, type_c::Current::UsbDefault);

        let current: type_c::Current = TypecCurrent::Current1A5.try_into().unwrap();
        assert_eq!(current, type_c::Current::Current1A5);

        let current: type_c::Current = TypecCurrent::Current3A0.try_into().unwrap();
        assert_eq!(current, type_c::Current::Current3A0);

        let current: TypecCurrent = type_c::Current::UsbDefault.into();
        assert_eq!(current, TypecCurrent::UsbDefault);

        let current: TypecCurrent = type_c::Current::Current1A5.into();
        assert_eq!(current, TypecCurrent::Current1A5);

        let current: TypecCurrent = type_c::Current::Current3A0.into();
        assert_eq!(current, TypecCurrent::Current3A0);
    }

    #[test]
    fn test_convert_pd_cc_pull_up() {
        let current: type_c::Current = PdCcPullUp::UsbDefault.try_into().unwrap();
        assert_eq!(current, type_c::Current::UsbDefault);

        let current: type_c::Current = PdCcPullUp::Current1A5.try_into().unwrap();
        assert_eq!(current, type_c::Current::Current1A5);

        let current: type_c::Current = PdCcPullUp::Current3A0.try_into().unwrap();
        assert_eq!(current, type_c::Current::Current3A0);

        let result: Result<type_c::Current, _> = PdCcPullUp::NoPull.try_into();
        assert_eq!(result, Err(PdError::InvalidParams));

        let pull_up: PdCcPullUp = type_c::Current::UsbDefault.into();
        assert_eq!(pull_up, PdCcPullUp::UsbDefault);

        let pull_up: PdCcPullUp = type_c::Current::Current1A5.into();
        assert_eq!(pull_up, PdCcPullUp::Current1A5);

        let pull_up: PdCcPullUp = type_c::Current::Current3A0.into();
        assert_eq!(pull_up, PdCcPullUp::Current3A0);
    }

    #[test]
    fn test_convert_plug_mode_to_connection_state() {
        let state: ConnectionState = PlugMode::Debug.try_into().unwrap();
        assert!(matches!(state, ConnectionState::DebugAccessory));

        let state: ConnectionState = PlugMode::Audio.try_into().unwrap();
        assert!(matches!(state, ConnectionState::AudioAccessory));

        let state: ConnectionState = PlugMode::Connected.try_into().unwrap();
        assert!(matches!(state, ConnectionState::Attached));

        let state: ConnectionState = PlugMode::ConnectedNoRa.try_into().unwrap();
        assert!(matches!(state, ConnectionState::Attached));

        let result: Result<ConnectionState, _> = PlugMode::NotConnected.try_into();
        assert_eq!(result.unwrap_err(), PdError::InvalidParams);

        let result: Result<ConnectionState, _> = PlugMode::Disabled.try_into();
        assert_eq!(result.unwrap_err(), PdError::InvalidParams);

        let result: Result<ConnectionState, _> = PlugMode::RaDetected.try_into();
        assert_eq!(result.unwrap_err(), PdError::InvalidParams);
    }

    #[test]
    fn test_convert_mode_to_str() {
        let s: &str = crate::Mode::Boot.into();
        assert_eq!(s, "BOOT");

        let s: &str = crate::Mode::F211.into();
        assert_eq!(s, "F211");

        let s: &str = crate::Mode::App0.into();
        assert_eq!(s, "APP0");

        let s: &str = crate::Mode::App1.into();
        assert_eq!(s, "APP1");

        let s: &str = crate::Mode::Wtpr.into();
        assert_eq!(s, "WTPR");
    }
}
