//! Autonegotiate Sink register (`0x37`).
//!
//! This register's size exceeds the maximum supported length by the [`device_driver`] crate.
//!
//! # TODO
//! - Use a units crate for safely expressing values like 20 mV per LSB, etc.

use bitfield::bitfield;
use embedded_usb_pd::pdo::{MA10_UNIT, MA50_UNIT, MV20_UNIT, MV50_UNIT, MW250_UNIT};

/// The address of the `Autonegotiate Sink` register.
pub const ADDR: u8 = 0x37;

/// The length of the `Autonegotiate Sink` register, in bytes.
///
/// This exceeds the maximum supported length by the [`device_driver`] crate.
pub const LEN: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoNegRdoPriority {
    HigherVoltage = 0x0,
    LowerVoltage = 0x1,
}

impl From<bool> for AutoNegRdoPriority {
    fn from(value: bool) -> Self {
        if value {
            AutoNegRdoPriority::LowerVoltage
        } else {
            AutoNegRdoPriority::HigherVoltage
        }
    }
}

impl From<AutoNegRdoPriority> for bool {
    fn from(value: AutoNegRdoPriority) -> Self {
        match value {
            AutoNegRdoPriority::HigherVoltage => false,
            AutoNegRdoPriority::LowerVoltage => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoComputeSinkMinPower {
    ProvidedByHost = 0x0,
    ComputedByPdController = 0x1,
}

impl From<bool> for AutoComputeSinkMinPower {
    fn from(value: bool) -> Self {
        if value {
            AutoComputeSinkMinPower::ComputedByPdController
        } else {
            AutoComputeSinkMinPower::ProvidedByHost
        }
    }
}

impl From<AutoComputeSinkMinPower> for bool {
    fn from(value: AutoComputeSinkMinPower) -> Self {
        match value {
            AutoComputeSinkMinPower::ProvidedByHost => false,
            AutoComputeSinkMinPower::ComputedByPdController => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoCapabilityMismatch {
    Enabled = 0x0,
    Disabled = 0x1,
}

impl From<bool> for NoCapabilityMismatch {
    fn from(value: bool) -> Self {
        if value {
            NoCapabilityMismatch::Disabled
        } else {
            NoCapabilityMismatch::Enabled
        }
    }
}

impl From<NoCapabilityMismatch> for bool {
    fn from(value: NoCapabilityMismatch) -> Self {
        match value {
            NoCapabilityMismatch::Enabled => false,
            NoCapabilityMismatch::Disabled => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoComputeSinkMinVoltage {
    ProvidedByHost = 0x0,
    ComputedByPdController = 0x1,
}

impl From<bool> for AutoComputeSinkMinVoltage {
    fn from(value: bool) -> Self {
        if value {
            AutoComputeSinkMinVoltage::ComputedByPdController
        } else {
            AutoComputeSinkMinVoltage::ProvidedByHost
        }
    }
}

impl From<AutoComputeSinkMinVoltage> for bool {
    fn from(value: AutoComputeSinkMinVoltage) -> Self {
        match value {
            AutoComputeSinkMinVoltage::ProvidedByHost => false,
            AutoComputeSinkMinVoltage::ComputedByPdController => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoComputeSinkMaxVoltage {
    ProvidedByHost = 0x0,
    ComputedByPdController = 0x1,
}

impl From<bool> for AutoComputeSinkMaxVoltage {
    fn from(value: bool) -> Self {
        if value {
            AutoComputeSinkMaxVoltage::ComputedByPdController
        } else {
            AutoComputeSinkMaxVoltage::ProvidedByHost
        }
    }
}

impl From<AutoComputeSinkMaxVoltage> for bool {
    fn from(value: AutoComputeSinkMaxVoltage) -> Self {
        match value {
            AutoComputeSinkMaxVoltage::ProvidedByHost => false,
            AutoComputeSinkMaxVoltage::ComputedByPdController => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PpsRequestInterval {
    EightSeconds = 0x0,
    FourSeconds = 0x1,
    TwoSeconds = 0x2,
    OneSecond = 0x3,
}

impl From<u8> for PpsRequestInterval {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0x0 => PpsRequestInterval::EightSeconds,
            0x1 => PpsRequestInterval::FourSeconds,
            0x2 => PpsRequestInterval::TwoSeconds,
            0x3 => PpsRequestInterval::OneSecond,
            _ => unreachable!("Masked value should always be in range 0-3"),
        }
    }
}

impl From<PpsRequestInterval> for u8 {
    fn from(value: PpsRequestInterval) -> Self {
        value as u8
    }
}

bitfield! {
    /// Raw bytes for the Autonegotiate Sink register.
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct AutonegotiateSinkRaw([u8]);
    impl Debug;

    /// Configuration for tie-breaker in PDO selection.
    pub bool, auto_neg_rdo_priority, set_auto_neg_rdo_priority: 0;

    /// Value used for the NoUSBSusp Flag in the RDO.
    pub bool, no_usb_suspend, set_no_usb_suspend: 1;

    /// How to compute the minimum power that the sink requires.
    pub bool, auto_compute_sink_min_power, set_auto_compute_sink_min_power: 2;

    /// Configuration for capability mismatch in RDO.
    pub bool, no_capability_mismatch, set_no_capability_mismatch: 3;

    /// How to compute the minimum voltage that the sink requires.
    pub bool, auto_compute_sink_min_voltage, set_auto_compute_sink_min_voltage: 4;

    /// How to compute the maximum voltage that the sink requires.
    pub bool, auto_compute_sink_max_voltage, set_auto_compute_sink_max_voltage: 5;

    /// Sink path and capability mismatch setting.
    pub bool, auto_disable_sink_upon_capability_mismatch, set_auto_disable_sink_upon_capability_mismatch: 6;

    /// Disable the sinking path during contract negotiation, and enable it back after receiving PS-RDY from SRC.
    pub bool, auto_disable_input_for_sink_standby, set_auto_disable_input_for_sink_standby: 7;

    /// Disable the sinking path during contract negotiation, and enable it back after receiving PS-RDY from SRC when in DBM.
    pub bool, auto_disable_input_for_sink_standby_in_dbm, set_auto_disable_input_for_sink_standby_in_dbm: 8;

    /// Automatically re-enable the sink path for Sink Standby when PS-RDY is received.
    pub bool, auto_enable_standby_srdy, set_auto_enable_standby_srdy: 9;

    /// Keep the sink path disabled until final PD contract is set.
    pub bool, auto_enable_input_after_snk_ready_in_dbm, set_auto_enable_input_after_snk_ready_in_dbm: 10;

    /// Maximum current to request. 10 mA per LSB.
    pub u16, auto_neg_max_current, set_auto_neg_max_current: 21, 12;

    /// Minimum operating power required by the sink. 250 mW per LSB.
    pub u16, auto_neg_sink_min_required_power, set_auto_neg_sink_min_required_power: 31, 22;

    /// Maximum voltage to request. 50 mV per LSB.
    pub u16, auto_neg_max_voltage, set_auto_neg_max_voltage: 41, 32;

    /// Minimum voltage to request. 50 mV per LSB.
    pub u16, auto_neg_min_voltage, set_auto_neg_min_voltage: 51, 42;

    /// Capabilities mismatch power threshold. 250 mW per LSB.
    pub u16, auto_neg_capabilities_mismatch_power, set_auto_neg_capabilities_mismatch_power: 61, 52;

    /// Enable Sink PPS mode.
    pub bool, pps_enable_sink_mode, set_pps_enable_sink_mode: 64;

    /// Sink PPS request interval.
    pub u8, pps_request_interval, set_pps_request_interval: 66, 65;

    /// Selection for CV or CC mode.
    pub bool, pps_source_operating_mode, set_pps_source_operating_mode: 67;

    /// Select only a source with full voltage range.
    pub bool, pps_required_full_voltage_range, set_pps_required_full_voltage_range: 68;

    /// Sink path handling during supply type transition.
    pub bool, pps_disable_sink_upon_non_apdo_contract, set_pps_disable_sink_upon_non_apdo_contract: 69;

    /// Enable Sink Path above PPS Sink Voltage threshold.
    pub bool, enable_sink_path_above_pps_sink_voltage_threshold, set_enable_sink_path_above_pps_sink_voltage_threshold: 83;

    /// Disable Sink Path if not in PPS contract.
    pub bool, disable_sink_if_not_in_pps_contract, set_disable_sink_if_not_in_pps_contract: 84;

    /// Operation current in Sink PPS mode.
    pub u16, pps_sink_voltage_threshold, set_pps_sink_voltage_threshold: 95, 85;

    /// Operation current in Sink PPS mode. 50 mA per LSB.
    pub u8, pps_operating_current, set_pps_operating_current: 102, 96;

    /// Desired output voltage. 20 mV per LSB.
    pub u16, pps_output_voltage, set_pps_output_voltage: 115, 105;

    /// Enable Sink EPR AVS mode.
    pub bool, epr_avs_enable_sink_mode, set_epr_avs_enable_sink_mode: 128;

    /// AVS operating current.
    pub u8, epr_avs_operating_current, set_epr_avs_operating_current: 166, 160;

    /// AVS operating voltage.
    pub u16, epr_avs_output_voltage, set_epr_avs_output_voltage: 180, 169;
}

/// The autonegotiate sink register.
#[derive(Debug, Clone)]
pub struct AutonegotiateSink(AutonegotiateSinkRaw<[u8; LEN]>);

impl AutonegotiateSink {
    /// The default bytes for the autonegotiate sink register.
    const DEFAULT: [u8; LEN] = [
        0xBE, 0x40, 0x1F, 0x0F, 0x90, 0x91, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    /// Get the raw byte representation of the autonegotiate sink register.
    pub fn as_bytes(&self) -> &[u8; LEN] {
        &self.0 .0
    }

    /// Configuration for tie-breaker in PDO selection.
    pub fn auto_neg_rdo_priority(&self) -> AutoNegRdoPriority {
        self.0.auto_neg_rdo_priority().into()
    }

    /// Set [`Self::auto_neg_rdo_priority`] and return `self` to chain.
    pub fn set_auto_neg_rdo_priority(&mut self, value: AutoNegRdoPriority) -> &mut Self {
        self.0.set_auto_neg_rdo_priority(value.into());
        self
    }

    /// Value used for the NoUSBSusp Flag in the RDO.
    pub fn no_usb_suspend(&self) -> bool {
        self.0.no_usb_suspend()
    }

    /// Set [`Self::no_usb_suspend`] and return `self` to chain.
    pub fn set_no_usb_suspend(&mut self, value: bool) -> &mut Self {
        self.0.set_no_usb_suspend(value);
        self
    }

    /// How to compute the minimum power that the sink requires.
    pub fn auto_compute_sink_min_power(&self) -> AutoComputeSinkMinPower {
        self.0.auto_compute_sink_min_power().into()
    }

    /// Set [`Self::auto_compute_sink_min_power`] and return `self` to chain.
    pub fn set_auto_compute_sink_min_power(&mut self, value: AutoComputeSinkMinPower) -> &mut Self {
        self.0.set_auto_compute_sink_min_power(value.into());
        self
    }

    /// Configuration for capability mismatch in RDO.
    pub fn no_capability_mismatch(&self) -> NoCapabilityMismatch {
        self.0.no_capability_mismatch().into()
    }

    /// Set [`Self::no_capability_mismatch`] and return `self` to chain.
    pub fn set_no_capability_mismatch(&mut self, value: NoCapabilityMismatch) -> &mut Self {
        self.0.set_no_capability_mismatch(value.into());
        self
    }

    /// How to compute the minimum voltage that the sink requires.
    pub fn auto_compute_sink_min_voltage(&self) -> AutoComputeSinkMinVoltage {
        self.0.auto_compute_sink_min_voltage().into()
    }

    /// Set [`Self::auto_compute_sink_min_voltage`] and return `self` to chain.
    pub fn set_auto_compute_sink_min_voltage(&mut self, value: AutoComputeSinkMinVoltage) -> &mut Self {
        self.0.set_auto_compute_sink_min_voltage(value.into());
        self
    }

    /// How to compute the maximum voltage that the sink requires.
    pub fn auto_compute_sink_max_voltage(&self) -> AutoComputeSinkMaxVoltage {
        self.0.auto_compute_sink_max_voltage().into()
    }

    /// Set [`Self::auto_compute_sink_max_voltage`] and return `self` to chain.
    pub fn set_auto_compute_sink_max_voltage(&mut self, value: AutoComputeSinkMaxVoltage) -> &mut Self {
        self.0.set_auto_compute_sink_max_voltage(value.into());
        self
    }

    /// Sink path and capability mismatch setting.
    pub fn auto_disable_sink_upon_capability_mismatch(&self) -> bool {
        self.0.auto_disable_sink_upon_capability_mismatch()
    }

    /// Set [`Self::auto_disable_sink_upon_capability_mismatch`] and return `self` to chain.
    pub fn set_auto_disable_sink_upon_capability_mismatch(&mut self, value: bool) -> &mut Self {
        self.0.set_auto_disable_sink_upon_capability_mismatch(value);
        self
    }

    /// Disable the sinking path during contract negotiation, and enable it back after receiving PS-RDY from SRC.
    pub fn auto_disable_input_for_sink_standby(&self) -> bool {
        self.0.auto_disable_input_for_sink_standby()
    }

    /// Set [`Self::auto_disable_input_for_sink_standby`] and return `self` to chain.
    pub fn set_auto_disable_input_for_sink_standby(&mut self, value: bool) -> &mut Self {
        self.0.set_auto_disable_input_for_sink_standby(value);
        self
    }

    /// Disable the sinking path during contract negotiation, and enable it back after receiving PS-RDY from SRC when in DBM.
    pub fn auto_disable_input_for_sink_standby_in_dbm(&self) -> bool {
        self.0.auto_disable_input_for_sink_standby_in_dbm()
    }

    /// Set [`Self::auto_disable_input_for_sink_standby_in_dbm`] and return `self` to chain.
    pub fn set_auto_disable_input_for_sink_standby_in_dbm(&mut self, value: bool) -> &mut Self {
        self.0.set_auto_disable_input_for_sink_standby_in_dbm(value);
        self
    }

    /// Automatically re-enable the sink path for Sink Standby when PS-RDY is received.
    pub fn auto_enable_standby_srdy(&self) -> bool {
        self.0.auto_enable_standby_srdy()
    }

    /// Set [`Self::auto_enable_standby_srdy`] and return `self` to chain.
    pub fn set_auto_enable_standby_srdy(&mut self, value: bool) -> &mut Self {
        self.0.set_auto_enable_standby_srdy(value);
        self
    }

    /// Keep the sink path disabled until final PD contract is set.
    pub fn auto_enable_input_after_snk_ready_in_dbm(&self) -> bool {
        self.0.auto_enable_input_after_snk_ready_in_dbm()
    }

    /// Set [`Self::auto_enable_input_after_snk_ready_in_dbm`] and return `self` to chain.
    pub fn set_auto_enable_input_after_snk_ready_in_dbm(&mut self, value: bool) -> &mut Self {
        self.0.set_auto_enable_input_after_snk_ready_in_dbm(value);
        self
    }

    /// Maximum current to request in mA.
    pub fn auto_neg_max_current(&self) -> u16 {
        self.0.auto_neg_max_current() * MA10_UNIT
    }

    /// Set [`Self::auto_neg_max_current`] and return `self` to chain.
    pub fn set_auto_neg_max_current(&mut self, value: u16) -> &mut Self {
        self.0.set_auto_neg_max_current(value);
        self
    }

    /// Minimum operating power required by the sink in mW.
    pub fn auto_neg_sink_min_required_power(&self) -> u32 {
        self.0.auto_neg_sink_min_required_power() as u32 * MW250_UNIT
    }

    /// Set [`Self::auto_neg_sink_min_required_power`] and return `self` to chain.
    pub fn set_auto_neg_sink_min_required_power(&mut self, value_mw: u32) -> &mut Self {
        self.0
            .set_auto_neg_sink_min_required_power((value_mw / MW250_UNIT) as u16);
        self
    }

    /// Maximum voltage to request in mV.
    pub fn auto_neg_max_voltage(&self) -> u16 {
        self.0.auto_neg_max_voltage() * MV50_UNIT
    }

    /// Set [`Self::auto_neg_max_voltage`] and return `self` to chain.
    pub fn set_auto_neg_max_voltage(&mut self, value_mv: u16) -> &mut Self {
        self.0.set_auto_neg_max_voltage(value_mv / MV50_UNIT);
        self
    }

    /// Minimum voltage to request in mV.
    pub fn auto_neg_min_voltage(&self) -> u16 {
        self.0.auto_neg_min_voltage() * MV50_UNIT
    }

    /// Set [`Self::auto_neg_min_voltage`] and return `self` to chain.
    pub fn set_auto_neg_min_voltage(&mut self, value_mv: u16) -> &mut Self {
        self.0.set_auto_neg_min_voltage(value_mv / MV50_UNIT);
        self
    }

    /// Capabilities mismatch power threshold.
    pub fn auto_neg_capabilities_mismatch_power(&self) -> u32 {
        self.0.auto_neg_capabilities_mismatch_power() as u32 * MW250_UNIT
    }

    /// Set [`Self::auto_neg_capabilities_mismatch_power`] and return `self` to chain.
    pub fn set_auto_neg_capabilities_mismatch_power(&mut self, value_mw: u32) -> &mut Self {
        self.0
            .set_auto_neg_capabilities_mismatch_power((value_mw / MW250_UNIT) as u16);
        self
    }

    /// Enable Sink PPS mode.
    pub fn pps_enable_sink_mode(&self) -> bool {
        self.0.pps_enable_sink_mode()
    }

    /// Set [`Self::pps_enable_sink_mode`] and return `self` to chain.
    pub fn set_pps_enable_sink_mode(&mut self, value: bool) -> &mut Self {
        self.0.set_pps_enable_sink_mode(value);
        self
    }

    /// Sink PPS request interval.
    pub fn pps_request_interval(&self) -> PpsRequestInterval {
        self.0.pps_request_interval().into()
    }

    /// Set [`Self::pps_request_interval`] and return `self` to chain.
    pub fn set_pps_request_interval(&mut self, value: PpsRequestInterval) -> &mut Self {
        self.0.set_pps_request_interval(value.into());
        self
    }

    /// Selection for CV or CC mode.
    pub fn pps_source_operating_mode(&self) -> bool {
        self.0.pps_source_operating_mode()
    }

    /// Set [`Self::pps_source_operating_mode`] and return `self` to chain.
    pub fn set_pps_source_operating_mode(&mut self, value: bool) -> &mut Self {
        self.0.set_pps_source_operating_mode(value);
        self
    }

    /// Select only a source with full voltage range.
    pub fn pps_required_full_voltage_range(&self) -> bool {
        self.0.pps_required_full_voltage_range()
    }

    /// Set [`Self::pps_required_full_voltage_range`] and return `self` to chain.
    pub fn set_pps_required_full_voltage_range(&mut self, value: bool) -> &mut Self {
        self.0.set_pps_required_full_voltage_range(value);
        self
    }

    /// Sink path handling during supply type transition.
    pub fn pps_disable_sink_upon_non_apdo_contract(&self) -> bool {
        self.0.pps_disable_sink_upon_non_apdo_contract()
    }

    /// Set [`Self::pps_disable_sink_upon_non_apdo_contract`] and return `self` to chain.
    pub fn set_pps_disable_sink_upon_non_apdo_contract(&mut self, value: bool) -> &mut Self {
        self.0.set_pps_disable_sink_upon_non_apdo_contract(value);
        self
    }

    /// Enable Sink Path above PPS Sink Voltage threshold.
    pub fn enable_sink_path_above_pps_sink_voltage_threshold(&self) -> bool {
        self.0.enable_sink_path_above_pps_sink_voltage_threshold()
    }

    /// Set [`Self::enable_sink_path_above_pps_sink_voltage_threshold`] and return `self` to chain.
    pub fn set_enable_sink_path_above_pps_sink_voltage_threshold(&mut self, value: bool) -> &mut Self {
        self.0.set_enable_sink_path_above_pps_sink_voltage_threshold(value);
        self
    }

    /// Disable Sink Path if not in PPS contract.
    pub fn disable_sink_if_not_in_pps_contract(&self) -> bool {
        self.0.disable_sink_if_not_in_pps_contract()
    }

    /// Set [`Self::disable_sink_if_not_in_pps_contract`] and return `self` to chain.
    pub fn set_disable_sink_if_not_in_pps_contract(&mut self, value: bool) -> &mut Self {
        self.0.set_disable_sink_if_not_in_pps_contract(value);
        self
    }

    /// Operation current in Sink PPS mode.
    pub fn pps_sink_voltage_threshold(&self) -> u16 {
        self.0.pps_sink_voltage_threshold()
    }

    /// Set [`Self::pps_sink_voltage_threshold`] and return `self` to chain.
    pub fn set_pps_sink_voltage_threshold(&mut self, value: u16) -> &mut Self {
        self.0.set_pps_sink_voltage_threshold(value);
        self
    }

    /// Operation current in Sink PPS mode in mA.
    pub fn pps_operating_current(&self) -> u16 {
        self.0.pps_operating_current() as u16 * MA50_UNIT
    }

    /// Set [`Self::pps_operating_current`] and return `self` to chain.
    pub fn set_pps_operating_current(&mut self, value: u16) -> &mut Self {
        self.0.set_pps_operating_current((value / MA50_UNIT) as u8);
        self
    }

    /// Desired output voltage in mV.
    pub fn pps_output_voltage(&self) -> u16 {
        self.0.pps_output_voltage() * MV20_UNIT
    }

    /// Set [`Self::pps_output_voltage`] and return `self` to chain.
    pub fn set_pps_output_voltage(&mut self, value_mv: u16) -> &mut Self {
        self.0.set_pps_output_voltage(value_mv / MV20_UNIT);
        self
    }

    /// Enable Sink EPR AVS mode.
    pub fn epr_avs_enable_sink_mode(&self) -> bool {
        self.0.epr_avs_enable_sink_mode()
    }

    /// Set [`Self::epr_avs_enable_sink_mode`] and return `self` to chain.
    pub fn set_epr_avs_enable_sink_mode(&mut self, value: bool) -> &mut Self {
        self.0.set_epr_avs_enable_sink_mode(value);
        self
    }

    /// AVS operating current.
    pub fn epr_avs_operating_current(&self) -> u8 {
        self.0.epr_avs_operating_current()
    }

    /// Set [`Self::epr_avs_operating_current`] and return `self` to chain.
    pub fn set_epr_avs_operating_current(&mut self, value: u8) -> &mut Self {
        self.0.set_epr_avs_operating_current(value);
        self
    }

    /// AVS operating voltage.
    pub fn epr_avs_output_voltage(&self) -> u16 {
        self.0.epr_avs_output_voltage()
    }

    /// Set [`Self::epr_avs_output_voltage`] and return `self` to chain.
    pub fn set_epr_avs_output_voltage(&mut self, value: u16) -> &mut Self {
        self.0.set_epr_avs_output_voltage(value);
        self
    }
}

impl From<[u8; LEN]> for AutonegotiateSink {
    fn from(value: [u8; LEN]) -> Self {
        AutonegotiateSink(AutonegotiateSinkRaw(value))
    }
}

impl From<AutonegotiateSink> for [u8; LEN] {
    fn from(value: AutonegotiateSink) -> Self {
        value.0 .0
    }
}

impl Default for AutonegotiateSink {
    fn default() -> Self {
        Self::DEFAULT.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;
    use std::eprintln;

    #[test]
    fn default_matches_datasheet() {
        let actual = AutonegotiateSink::default();
        eprintln!("{:#X?}", actual);

        assert_eq!(actual.auto_neg_rdo_priority(), AutoNegRdoPriority::HigherVoltage);
        assert!(actual.no_usb_suspend());
        assert_eq!(
            actual.auto_compute_sink_min_power(),
            AutoComputeSinkMinPower::ComputedByPdController
        );
        assert_eq!(actual.no_capability_mismatch(), NoCapabilityMismatch::Disabled);
        assert_eq!(
            actual.auto_compute_sink_min_voltage(),
            AutoComputeSinkMinVoltage::ComputedByPdController
        );
        assert_eq!(
            actual.auto_compute_sink_max_voltage(),
            AutoComputeSinkMaxVoltage::ComputedByPdController
        );
        assert!(!actual.auto_disable_sink_upon_capability_mismatch());
        assert!(actual.auto_disable_input_for_sink_standby());
        assert!(!actual.auto_disable_input_for_sink_standby_in_dbm());
        assert!(!actual.auto_enable_standby_srdy());
        assert!(!actual.auto_enable_input_after_snk_ready_in_dbm());
        assert_eq!(actual.auto_neg_max_current(), 0x1F4 * MA10_UNIT);
        assert_eq!(actual.auto_neg_sink_min_required_power(), 0x3C * MW250_UNIT);
        assert_eq!(actual.auto_neg_max_voltage(), 0x190 * MV50_UNIT);
        assert_eq!(actual.auto_neg_min_voltage(), 0x64 * MV50_UNIT);
        assert_eq!(actual.auto_neg_capabilities_mismatch_power(), 0);
        assert!(!actual.pps_enable_sink_mode());
        assert_eq!(actual.pps_request_interval(), PpsRequestInterval::EightSeconds);
        assert!(!actual.pps_source_operating_mode());
        assert!(!actual.pps_required_full_voltage_range());
        assert!(!actual.pps_disable_sink_upon_non_apdo_contract());
        assert!(!actual.enable_sink_path_above_pps_sink_voltage_threshold());
        assert!(!actual.disable_sink_if_not_in_pps_contract());
        assert_eq!(actual.pps_sink_voltage_threshold(), 0);
        assert_eq!(actual.pps_operating_current(), 0);
        assert_eq!(actual.pps_output_voltage(), 0);
        assert!(!actual.epr_avs_enable_sink_mode());
        assert_eq!(actual.epr_avs_operating_current(), 0);
        assert_eq!(actual.epr_avs_output_voltage(), 0);

        let bytes = actual.as_bytes();
        assert_eq!(bytes, &AutonegotiateSink::DEFAULT);
    }
}
