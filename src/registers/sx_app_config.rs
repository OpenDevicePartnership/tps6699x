//! Set Sx App Config register (`0x20`).
//!
//! Configuration based on system power state. The Host may write the current system power state,
//! and a change in power state triggers a new Application Configuration to be applied.

use bitfield::bitfield;

/// The address of the `Set Sx App Config` register.
pub const ADDR: u8 = 0x20;

/// The length of the `Set Sx App Config` register, in bytes.
pub const LEN: usize = 2;

/// System power state values.
///
/// When a change in power state occurs, a new app config will be applied
/// per the settings in register 0x1F.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemPowerState {
    /// S0 - System fully running
    S0,
    /// S3 - Suspend to RAM
    S3,
    /// S4 - Hibernate
    S4,
    /// S5 - Soft off
    S5,
    /// S0ix - Modern standby / Connected standby
    S0ix,
    /// Reserved value
    Reserved(u8),
}

impl From<u8> for SystemPowerState {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => SystemPowerState::S0,
            0x1 => SystemPowerState::S3,
            0x2 => SystemPowerState::S4,
            0x3 => SystemPowerState::S5,
            0x4 => SystemPowerState::S0ix,
            x => SystemPowerState::Reserved(x),
        }
    }
}

impl From<SystemPowerState> for u8 {
    fn from(value: SystemPowerState) -> Self {
        match value {
            SystemPowerState::S0 => 0x0,
            SystemPowerState::S3 => 0x1,
            SystemPowerState::S4 => 0x2,
            SystemPowerState::S5 => 0x3,
            SystemPowerState::S0ix => 0x4,
            SystemPowerState::Reserved(x) => x,
        }
    }
}

bitfield! {
    /// Raw bytes for the Set Sx App Config register.
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct SxAppConfigRaw([u8]);
    impl Debug;

    /// Current power state (bits 2-0).
    pub u8, sleep_state, set_sleep_state: 2, 0;
}

/// The Set Sx App Config register.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SxAppConfig(SxAppConfigRaw<[u8; LEN]>);

impl SxAppConfig {
    /// The default bytes for the register (S0 state).
    const DEFAULT: [u8; LEN] = [0x00, 0x00];

    /// Get the raw byte representation of the register.
    pub fn as_bytes(&self) -> &[u8; LEN] {
        &self.0 .0
    }

    /// Get the current power state.
    pub fn power_state(&self) -> SystemPowerState {
        self.0.sleep_state().into()
    }

    /// Set the power state and return `self` to chain.
    pub fn set_power_state(&mut self, value: SystemPowerState) -> &mut Self {
        self.0.set_sleep_state(value.into());
        self
    }
}

impl From<[u8; LEN]> for SxAppConfig {
    fn from(value: [u8; LEN]) -> Self {
        SxAppConfig(SxAppConfigRaw(value))
    }
}

impl From<SxAppConfig> for [u8; LEN] {
    fn from(value: SxAppConfig) -> Self {
        value.0 .0
    }
}

impl Default for SxAppConfig {
    fn default() -> Self {
        Self::DEFAULT.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_state_conversions() {
        assert_eq!(SystemPowerState::S0, 0u8.into());
        assert_eq!(SystemPowerState::S3, 1u8.into());
        assert_eq!(SystemPowerState::S4, 2u8.into());
        assert_eq!(SystemPowerState::S5, 3u8.into());
        assert_eq!(SystemPowerState::S0ix, 4u8.into());

        assert_eq!(u8::from(SystemPowerState::S0), 0);
        assert_eq!(u8::from(SystemPowerState::S3), 1);
        assert_eq!(u8::from(SystemPowerState::S4), 2);
        assert_eq!(u8::from(SystemPowerState::S5), 3);
        assert_eq!(u8::from(SystemPowerState::S0ix), 4);
    }

    #[test]
    fn test_default() {
        let config = SxAppConfig::default();
        assert_eq!(config.power_state(), SystemPowerState::S0);
    }

    #[test]
    fn test_set_power_state() {
        let mut config = SxAppConfig::default();
        config.set_power_state(SystemPowerState::S5);
        assert_eq!(config.power_state(), SystemPowerState::S5);
        assert_eq!(config.as_bytes()[0], 0x03);
    }
}
