use device_driver;
use embedded_usb_pd::type_c::ConnectionState;
use embedded_usb_pd::{type_c, PdError};

use crate::Mode;

pub mod autonegotiate_sink;
pub mod boot_flags;
pub mod dp_status;
pub mod port_config;
pub mod rx_other_vdm;

device_driver::create_device!(
    device_name: Registers,
    manifest: "device.yaml"
);

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
