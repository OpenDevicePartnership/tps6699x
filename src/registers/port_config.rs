//! Port Configuration register (`0x28`).
//!
//! This register controls various port configuration options.
//!
//! # TODO
//! - Use a units crate for safely expressing values like 50 mV per LSB, etc.

use bitfield::bitfield;
use embedded_usb_pd::pdo::MV50_UNIT;

/// The address of the `Port Configuration` register.
pub const ADDR: u8 = 0x28;

/// The length of the `Port Configuration` register, in bytes.
pub const LEN: usize = 18;

/// SBU Mux Usage field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SbuMuxUsage {
    /// DisplayPort mode enables AUX channel TBT and USB4 modes enable SBRX/TX channel
    DisplayPortAux,
    /// DisplayPort TBT and USB4 modes all enable AUX channel
    AllAux,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for SbuMuxUsage {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0x0 => SbuMuxUsage::DisplayPortAux,
            0x1 => SbuMuxUsage::AllAux,
            x => SbuMuxUsage::Reserved(x),
        }
    }
}

impl From<SbuMuxUsage> for u8 {
    fn from(value: SbuMuxUsage) -> Self {
        match value {
            SbuMuxUsage::DisplayPortAux => 0x0,
            SbuMuxUsage::AllAux => 0x1,
            SbuMuxUsage::Reserved(x) => x,
        }
    }
}

/// SBU Mux Default Setting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SbuMuxDefaultSetting {
    /// Don't change the default setting after booting
    DontChange,
    /// Disable the SBU mux after booting
    DisableSbuMux,
    /// Enable the Px_SBU mux to Px_DBG after booting
    EnableDbg,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for SbuMuxDefaultSetting {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => SbuMuxDefaultSetting::DontChange,
            0x1 => SbuMuxDefaultSetting::DisableSbuMux,
            0x2 => SbuMuxDefaultSetting::EnableDbg,
            x => SbuMuxDefaultSetting::Reserved(x),
        }
    }
}

impl From<SbuMuxDefaultSetting> for u8 {
    fn from(value: SbuMuxDefaultSetting) -> Self {
        match value {
            SbuMuxDefaultSetting::DontChange => 0x0,
            SbuMuxDefaultSetting::DisableSbuMux => 0x1,
            SbuMuxDefaultSetting::EnableDbg => 0x2,
            SbuMuxDefaultSetting::Reserved(x) => x,
        }
    }
}

/// SBU Mux Debug Setting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SbuMuxDebugSetting {
    /// Enable SBU to DBG upon entry to DebugAccessory.SNK
    DebugSnk,
    /// Enable SBU to DBG upon entry to UnorientedDebugAccessory.SRC
    UnorientedDebugSrc,
    /// Enable SBU to DBG upon entry to OrientedDebugAccessory.SRC
    OrientedDebugSrc,
    /// Enable SBU to DBG upon entry to any of the 3 debug accessory states
    AnyDebug,
    /// Never enable SBU to DBG
    NeverEnable,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for SbuMuxDebugSetting {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => SbuMuxDebugSetting::DebugSnk,
            0x1 => SbuMuxDebugSetting::UnorientedDebugSrc,
            0x2 => SbuMuxDebugSetting::OrientedDebugSrc,
            0x3 => SbuMuxDebugSetting::AnyDebug,
            0x4 => SbuMuxDebugSetting::NeverEnable,
            x => SbuMuxDebugSetting::Reserved(x),
        }
    }
}

impl From<SbuMuxDebugSetting> for u8 {
    fn from(value: SbuMuxDebugSetting) -> Self {
        match value {
            SbuMuxDebugSetting::DebugSnk => 0x0,
            SbuMuxDebugSetting::UnorientedDebugSrc => 0x1,
            SbuMuxDebugSetting::OrientedDebugSrc => 0x2,
            SbuMuxDebugSetting::AnyDebug => 0x3,
            SbuMuxDebugSetting::NeverEnable => 0x4,
            SbuMuxDebugSetting::Reserved(x) => x,
        }
    }
}

/// Level shifter direction configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LevelShifterDirectionCfg {
    /// Automatic
    Automatic,
    /// GPIO controlled
    GpioControlled,
    /// I2C controlled
    I2cControlled,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for LevelShifterDirectionCfg {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0x0 => LevelShifterDirectionCfg::Automatic,
            0x1 => LevelShifterDirectionCfg::GpioControlled,
            0x2 => LevelShifterDirectionCfg::I2cControlled,
            x => LevelShifterDirectionCfg::Reserved(x),
        }
    }
}

impl From<LevelShifterDirectionCfg> for u8 {
    fn from(value: LevelShifterDirectionCfg) -> Self {
        match value {
            LevelShifterDirectionCfg::Automatic => 0x0,
            LevelShifterDirectionCfg::GpioControlled => 0x1,
            LevelShifterDirectionCfg::I2cControlled => 0x2,
            LevelShifterDirectionCfg::Reserved(x) => x,
        }
    }
}

/// External DCDC Type field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExternalDcdcType {
    /// Marina Bay
    MarinaBay,
    /// BQ25758
    Bq25758,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for ExternalDcdcType {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ExternalDcdcType::MarinaBay,
            0x1 => ExternalDcdcType::Bq25758,
            x => ExternalDcdcType::Reserved(x),
        }
    }
}

impl From<ExternalDcdcType> for u8 {
    fn from(value: ExternalDcdcType) -> Self {
        match value {
            ExternalDcdcType::MarinaBay => 0x0,
            ExternalDcdcType::Bq25758 => 0x1,
            ExternalDcdcType::Reserved(x) => x,
        }
    }
}

/// TypeC State Machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TypeCStateMachine {
    /// Sink state machine only
    Sink,
    /// Source state machine only
    Source,
    /// DRP state machine
    Drp,
    /// Disabled
    Disabled,
}

impl From<u8> for TypeCStateMachine {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0x0 => TypeCStateMachine::Sink,
            0x1 => TypeCStateMachine::Source,
            0x2 => TypeCStateMachine::Drp,
            _ => TypeCStateMachine::Disabled,
        }
    }
}

impl From<TypeCStateMachine> for u8 {
    fn from(value: TypeCStateMachine) -> Self {
        match value {
            TypeCStateMachine::Sink => 0x0,
            TypeCStateMachine::Source => 0x1,
            TypeCStateMachine::Drp => 0x2,
            TypeCStateMachine::Disabled => 0x3,
        }
    }
}

/// VBUS Sink UVP Trip HV
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusSinkUvpTripHv {
    /// 5%
    Pct5,
    /// 10%
    Pct10,
    /// 15%
    Pct15,
    /// 20%
    Pct20,
    /// 25%
    Pct25,
    /// 30%
    Pct30,
    /// 40%
    Pct40,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for VbusSinkUvpTripHv {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => VbusSinkUvpTripHv::Pct5,
            0x1 => VbusSinkUvpTripHv::Pct10,
            0x2 => VbusSinkUvpTripHv::Pct15,
            0x3 => VbusSinkUvpTripHv::Pct20,
            0x4 => VbusSinkUvpTripHv::Pct25,
            0x5 => VbusSinkUvpTripHv::Pct30,
            0x6 => VbusSinkUvpTripHv::Pct40,
            x => VbusSinkUvpTripHv::Reserved(x),
        }
    }
}

impl From<VbusSinkUvpTripHv> for u8 {
    fn from(value: VbusSinkUvpTripHv) -> Self {
        match value {
            VbusSinkUvpTripHv::Pct5 => 0x0,
            VbusSinkUvpTripHv::Pct10 => 0x1,
            VbusSinkUvpTripHv::Pct15 => 0x2,
            VbusSinkUvpTripHv::Pct20 => 0x3,
            VbusSinkUvpTripHv::Pct25 => 0x4,
            VbusSinkUvpTripHv::Pct30 => 0x5,
            VbusSinkUvpTripHv::Pct40 => 0x6,
            VbusSinkUvpTripHv::Reserved(x) => x,
        }
    }
}

/// OVP for PP5V
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvpForPp5v {
    /// 5.25V
    V5_25,
    /// 5.5V
    V5_5,
    /// 5.8V
    V5_8,
    /// 6.1V
    V6_1,
}

impl From<u8> for OvpForPp5v {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0x0 => OvpForPp5v::V5_25,
            0x1 => OvpForPp5v::V5_5,
            0x2 => OvpForPp5v::V5_8,
            _ => OvpForPp5v::V6_1,
        }
    }
}

impl From<OvpForPp5v> for u8 {
    fn from(value: OvpForPp5v) -> Self {
        match value {
            OvpForPp5v::V5_25 => 0x0,
            OvpForPp5v::V5_5 => 0x1,
            OvpForPp5v::V5_8 => 0x2,
            OvpForPp5v::V6_1 => 0x3,
        }
    }
}

/// VBUS OVP Usage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusOvpUsage {
    /// 100%
    Pct100,
    /// 105%
    Pct105,
    /// 111%
    Pct111,
    /// 114%
    Pct114,
}

impl From<u8> for VbusOvpUsage {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0x0 => VbusOvpUsage::Pct100,
            0x1 => VbusOvpUsage::Pct105,
            0x2 => VbusOvpUsage::Pct111,
            _ => VbusOvpUsage::Pct114,
        }
    }
}

impl From<VbusOvpUsage> for u8 {
    fn from(value: VbusOvpUsage) -> Self {
        match value {
            VbusOvpUsage::Pct100 => 0x0,
            VbusOvpUsage::Pct105 => 0x1,
            VbusOvpUsage::Pct111 => 0x2,
            VbusOvpUsage::Pct114 => 0x3,
        }
    }
}

/// USB3 Rate configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb3Rate {
    /// Not supported
    NotSupported,
    /// Gen1
    Gen1,
    /// Gen2
    Gen2,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for Usb3Rate {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0x0 => Usb3Rate::NotSupported,
            0x1 => Usb3Rate::Gen1,
            0x2 => Usb3Rate::Gen2,
            x => Usb3Rate::Reserved(x),
        }
    }
}

impl From<Usb3Rate> for u8 {
    fn from(value: Usb3Rate) -> Self {
        match value {
            Usb3Rate::NotSupported => 0x0,
            Usb3Rate::Gen1 => 0x1,
            Usb3Rate::Gen2 => 0x2,
            Usb3Rate::Reserved(x) => x,
        }
    }
}

/// TypeC Support Options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TypeCSupportOptions {
    /// No Type-C optional states are supported
    None,
    /// Try.SRC state is supported as a DRP
    TrySrcDrp,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for TypeCSupportOptions {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0x0 => TypeCSupportOptions::None,
            0x1 => TypeCSupportOptions::TrySrcDrp,
            x => TypeCSupportOptions::Reserved(x),
        }
    }
}

impl From<TypeCSupportOptions> for u8 {
    fn from(value: TypeCSupportOptions) -> Self {
        match value {
            TypeCSupportOptions::None => 0x0,
            TypeCSupportOptions::TrySrcDrp => 0x1,
            TypeCSupportOptions::Reserved(x) => x,
        }
    }
}

/// Crossbar Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrossbarType {
    /// Type 1
    Type1,
    /// Type 2
    Type2,
}

impl From<bool> for CrossbarType {
    fn from(value: bool) -> Self {
        if value {
            CrossbarType::Type2
        } else {
            CrossbarType::Type1
        }
    }
}

impl From<CrossbarType> for bool {
    fn from(value: CrossbarType) -> Self {
        matches!(value, CrossbarType::Type2)
    }
}

bitfield! {
    /// Raw bytes for the Port Configuration register.
    ///
    /// Each field corresponds to a bit or group of bits in the register.
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct PortConfigRaw([u8]);
    impl Debug;

    /// Flip PX_SBTX/RX
    pub bool, flip_crossbar_sbtx_setting, set_flip_crossbar_sbtx_setting: 141;
    /// Flip Px_AUX
    pub bool, flip_crossbar_aux_setting, set_flip_crossbar_aux_setting: 140;
    /// Flip Px_DBG
    pub bool, flip_crossbar_dbg_setting, set_flip_crossbar_dbg_setting: 139;
    /// EPR Supported as Sink
    pub bool, disable_sourcing_in_dbm, set_disable_sourcing_in_dbm: 138;
    /// EPR Supported as Sink
    pub bool, epr_supported_as_sink, set_epr_supported_as_sink: 137;
    /// EPR Supported as Source
    pub bool, epr_supported_as_source, set_epr_supported_as_source: 136;
    /// Sbu Mux Usage
    pub u8, sbu_mux_usage, set_sbu_mux_usage: 135, 134;
    /// SBU Mux Default Setting
    pub u8, sbu_mux_default_setting, set_sbu_mux_default_setting: 133, 131;
    /// SBU Mux Debug Setting
    pub u8, sbu_mux_debug_setting, set_sbu_mux_debug_setting: 130, 128;
    /// Level shifter direction configuration
    pub u8, level_shifter_direction_cfg, set_level_shifter_direction_cfg: 127, 126;
    /// EnableInternalLevelShifter
    pub bool, enable_internal_level_shifter, set_enable_internal_level_shifter: 125;
    /// Enable Internal Auxbiasing
    pub bool, enable_internal_auxbiasing, set_enable_internal_auxbiasing: 124;
    /// Threshold voltage to trigger the GREATER_THAN_THRESHOLD_VOLTAGE GPIO Event (50mV per LSB)
    pub u16, gt_threshold_voltage, set_gt_threshold_voltage: 95, 80;
    /// Enables support for GPIO10 to I2C IRQ
    pub bool, sink_mode_i2c_irq_config, set_sink_mode_i2c_irq_config: 72;
    /// External DCDC Type
    pub u8, external_dcdc_type, set_external_dcdc_type: 71, 64;
    /// VBUS For Valid PPS Status
    pub u16, vbus_for_valid_pps_status, set_vbus_for_valid_pps_status: 63, 48;
    /// APDO VBUS Uvp TripPoint Offset
    pub u16, apdo_vbus_uvp_trip_point_offset, set_apdo_vbus_uvp_trip_point_offset: 47, 32;
    /// APDO ILIM Over Shoot
    pub u8, apdo_ilim_over_shoot, set_apdo_ilim_over_shoot: 30, 29;
    /// APDO VBUS UVP Threshold
    pub u8, apdo_vbus_uvp_threshold, set_apdo_vbus_uvp_threshold: 28, 27;
    /// VBUS Sink UVP Trip HV
    pub u8, vbus_sink_uvp_trip_hv, set_vbus_sink_uvp_trip_hv: 26, 24;
    /// Remove Safe-State Between USB3-to-DP Transition
    pub bool, remove_safe_state_between_usb3_to_dp_transition, set_remove_safe_state_between_usb3_to_dp_transition: 23;
    /// Crossbar Config Type 1 Extended
    pub bool, crossbar_config_type1_extended, set_crossbar_config_type1_extended: 22;
    /// OVP for PP5V
    pub u8, ovp_for_pp5v, set_ovp_for_pp5v: 21, 20;
    /// VBUS OVP Usage
    pub u8, vbus_ovp_usage, set_vbus_ovp_usage: 17, 16;
    /// Crossbar I2C Controller enable
    pub bool, crossbar_i2c_controller_enable, set_crossbar_i2c_controller_enable: 15;
    /// USB3 Rate
    pub u8, usb3_rate, set_usb3_rate: 14, 13;
    /// DebugAccessory Support
    pub bool, debug_accessory_support, set_debug_accessory_support: 12;
    /// USB Communication Capable
    pub bool, usb_communication_capable, set_usb_communication_capable: 11;
    /// Disable PD
    pub bool, disable_pd, set_disable_pd: 10;
    /// TypeC Support Options
    pub u8, typec_support_options, set_typec_support_options: 9, 8;
    /// PP_EXT ActiveLow
    pub bool, pp_ext_active_low, set_pp_ext_active_low: 7;
    /// Crossbar Type
    pub bool, crossbar_type, set_crossbar_type: 2;
    /// Type-C State machine
    pub u8, typec_state_machine, set_typec_state_machine: 1, 0;
}

/// The port configuration register.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PortConfig(PortConfigRaw<[u8; LEN]>);

impl PortConfig {
    /// The default bytes for the port configuration register.
    const DEFAULT: [u8; LEN] = [0u8; LEN];

    /// Get the raw byte representation of the port configuration register.
    pub fn as_bytes(&self) -> &[u8; LEN] {
        &self.0 .0
    }

    /// Configuration for flipping PX_SBTX/RX.
    pub fn flip_crossbar_sbtx_setting(&self) -> bool {
        self.0.flip_crossbar_sbtx_setting()
    }
    /// Set [`Self::flip_crossbar_sbtx_setting`] and return `self` to chain.
    pub fn set_flip_crossbar_sbtx_setting(&mut self, value: bool) -> &mut Self {
        self.0.set_flip_crossbar_sbtx_setting(value);
        self
    }

    /// Configuration for flipping Px_AUX.
    pub fn flip_crossbar_aux_setting(&self) -> bool {
        self.0.flip_crossbar_aux_setting()
    }
    /// Set [`Self::flip_crossbar_aux_setting`] and return `self` to chain.
    pub fn set_flip_crossbar_aux_setting(&mut self, value: bool) -> &mut Self {
        self.0.set_flip_crossbar_aux_setting(value);
        self
    }

    /// Configuration for flipping Px_DBG.
    pub fn flip_crossbar_dbg_setting(&self) -> bool {
        self.0.flip_crossbar_dbg_setting()
    }
    /// Set [`Self::flip_crossbar_dbg_setting`] and return `self` to chain.
    pub fn set_flip_crossbar_dbg_setting(&mut self, value: bool) -> &mut Self {
        self.0.set_flip_crossbar_dbg_setting(value);
        self
    }

    /// EPR Supported as Sink.
    pub fn disable_sourcing_in_dbm(&self) -> bool {
        self.0.disable_sourcing_in_dbm()
    }
    /// Set [`Self::disable_sourcing_in_dbm`] and return `self` to chain.
    pub fn set_disable_sourcing_in_dbm(&mut self, value: bool) -> &mut Self {
        self.0.set_disable_sourcing_in_dbm(value);
        self
    }

    /// EPR Supported as Sink.
    pub fn epr_supported_as_sink(&self) -> bool {
        self.0.epr_supported_as_sink()
    }
    /// Set [`Self::epr_supported_as_sink`] and return `self` to chain.
    pub fn set_epr_supported_as_sink(&mut self, value: bool) -> &mut Self {
        self.0.set_epr_supported_as_sink(value);
        self
    }

    /// EPR Supported as Source.
    pub fn epr_supported_as_source(&self) -> bool {
        self.0.epr_supported_as_source()
    }
    /// Set [`Self::epr_supported_as_source`] and return `self` to chain.
    pub fn set_epr_supported_as_source(&mut self, value: bool) -> &mut Self {
        self.0.set_epr_supported_as_source(value);
        self
    }

    /// SBU Mux Usage field.
    pub fn sbu_mux_usage(&self) -> SbuMuxUsage {
        self.0.sbu_mux_usage().into()
    }
    /// Set [`Self::sbu_mux_usage`] and return `self` to chain.
    pub fn set_sbu_mux_usage(&mut self, value: SbuMuxUsage) -> &mut Self {
        self.0.set_sbu_mux_usage(value.into());
        self
    }

    /// SBU Mux Default Setting field.
    pub fn sbu_mux_default_setting(&self) -> SbuMuxDefaultSetting {
        self.0.sbu_mux_default_setting().into()
    }
    /// Set [`Self::sbu_mux_default_setting`] and return `self` to chain.
    pub fn set_sbu_mux_default_setting(&mut self, value: SbuMuxDefaultSetting) -> &mut Self {
        self.0.set_sbu_mux_default_setting(value.into());
        self
    }

    /// SBU Mux Debug Setting field.
    pub fn sbu_mux_debug_setting(&self) -> SbuMuxDebugSetting {
        self.0.sbu_mux_debug_setting().into()
    }
    /// Set [`Self::sbu_mux_debug_setting`] and return `self` to chain.
    pub fn set_sbu_mux_debug_setting(&mut self, value: SbuMuxDebugSetting) -> &mut Self {
        self.0.set_sbu_mux_debug_setting(value.into());
        self
    }

    /// Level shifter direction configuration.
    pub fn level_shifter_direction_cfg(&self) -> LevelShifterDirectionCfg {
        self.0.level_shifter_direction_cfg().into()
    }
    /// Set [`Self::level_shifter_direction_cfg`] and return `self` to chain.
    pub fn set_level_shifter_direction_cfg(&mut self, value: LevelShifterDirectionCfg) -> &mut Self {
        self.0.set_level_shifter_direction_cfg(value.into());
        self
    }

    /// Enable internal level shifter.
    pub fn enable_internal_level_shifter(&self) -> bool {
        self.0.enable_internal_level_shifter()
    }
    /// Set [`Self::enable_internal_level_shifter`] and return `self` to chain.
    pub fn set_enable_internal_level_shifter(&mut self, value: bool) -> &mut Self {
        self.0.set_enable_internal_level_shifter(value);
        self
    }

    /// Enable internal auxbiasing.
    pub fn enable_internal_auxbiasing(&self) -> bool {
        self.0.enable_internal_auxbiasing()
    }
    /// Set [`Self::enable_internal_auxbiasing`] and return `self` to chain.
    pub fn set_enable_internal_auxbiasing(&mut self, value: bool) -> &mut Self {
        self.0.set_enable_internal_auxbiasing(value);
        self
    }

    /// Threshold voltage to trigger the GREATER_THAN_THRESHOLD_VOLTAGE GPIO Event in milivolts
    pub fn gt_threshold_voltage(&self) -> u16 {
        self.0.gt_threshold_voltage() * MV50_UNIT
    }
    /// Set [`Self::gt_threshold_voltage`] and return `self` to chain.
    pub fn set_gt_threshold_voltage(&mut self, value: u16) -> &mut Self {
        self.0.set_gt_threshold_voltage(value);
        self
    }

    /// Enables support for GPIO10 to I2C IRQ.
    pub fn sink_mode_i2c_irq_config(&self) -> bool {
        self.0.sink_mode_i2c_irq_config()
    }
    /// Set [`Self::sink_mode_i2c_irq_config`] and return `self` to chain.
    pub fn set_sink_mode_i2c_irq_config(&mut self, value: bool) -> &mut Self {
        self.0.set_sink_mode_i2c_irq_config(value);
        self
    }

    /// External DCDC Type field.
    pub fn external_dcdc_type(&self) -> ExternalDcdcType {
        self.0.external_dcdc_type().into()
    }
    /// Set [`Self::external_dcdc_type`] and return `self` to chain.
    pub fn set_external_dcdc_type(&mut self, value: ExternalDcdcType) -> &mut Self {
        self.0.set_external_dcdc_type(value.into());
        self
    }

    /// VBUS For Valid PPS Status.
    pub fn vbus_for_valid_pps_status(&self) -> u16 {
        self.0.vbus_for_valid_pps_status()
    }
    /// Set [`Self::vbus_for_valid_pps_status`] and return `self` to chain.
    pub fn set_vbus_for_valid_pps_status(&mut self, value: u16) -> &mut Self {
        self.0.set_vbus_for_valid_pps_status(value);
        self
    }

    /// APDO VBUS Uvp TripPoint Offset.
    pub fn apdo_vbus_uvp_trip_point_offset(&self) -> u16 {
        self.0.apdo_vbus_uvp_trip_point_offset()
    }
    /// Set [`Self::apdo_vbus_uvp_trip_point_offset`] and return `self` to chain.
    pub fn set_apdo_vbus_uvp_trip_point_offset(&mut self, value: u16) -> &mut Self {
        self.0.set_apdo_vbus_uvp_trip_point_offset(value);
        self
    }

    /// APDO ILIM Over Shoot.
    pub fn apdo_ilim_over_shoot(&self) -> u8 {
        self.0.apdo_ilim_over_shoot()
    }
    /// Set [`Self::apdo_ilim_over_shoot`] and return `self` to chain.
    pub fn set_apdo_ilim_over_shoot(&mut self, value: u8) -> &mut Self {
        self.0.set_apdo_ilim_over_shoot(value);
        self
    }

    /// APDO VBUS UVP Threshold.
    pub fn apdo_vbus_uvp_threshold(&self) -> u8 {
        self.0.apdo_vbus_uvp_threshold()
    }
    /// Set [`Self::apdo_vbus_uvp_threshold`] and return `self` to chain.
    pub fn set_apdo_vbus_uvp_threshold(&mut self, value: u8) -> &mut Self {
        self.0.set_apdo_vbus_uvp_threshold(value);
        self
    }

    /// VBUS Sink UVP Trip HV.
    pub fn vbus_sink_uvp_trip_hv(&self) -> VbusSinkUvpTripHv {
        self.0.vbus_sink_uvp_trip_hv().into()
    }
    /// Set [`Self::vbus_sink_uvp_trip_hv`] and return `self` to chain.
    pub fn set_vbus_sink_uvp_trip_hv(&mut self, value: VbusSinkUvpTripHv) -> &mut Self {
        self.0.set_vbus_sink_uvp_trip_hv(value.into());
        self
    }

    /// Remove Safe-State Between USB3-to-DP Transition.
    pub fn remove_safe_state_between_usb3_to_dp_transition(&self) -> bool {
        self.0.remove_safe_state_between_usb3_to_dp_transition()
    }
    /// Set [`Self::remove_safe_state_between_usb3_to_dp_transition`] and return `self` to chain.
    pub fn set_remove_safe_state_between_usb3_to_dp_transition(&mut self, value: bool) -> &mut Self {
        self.0.set_remove_safe_state_between_usb3_to_dp_transition(value);
        self
    }

    /// Crossbar Config Type 1 Extended.
    pub fn crossbar_config_type1_extended(&self) -> bool {
        self.0.crossbar_config_type1_extended()
    }
    /// Set [`Self::crossbar_config_type1_extended`] and return `self` to chain.
    pub fn set_crossbar_config_type1_extended(&mut self, value: bool) -> &mut Self {
        self.0.set_crossbar_config_type1_extended(value);
        self
    }

    /// OVP for PP5V.
    pub fn ovp_for_pp5v(&self) -> OvpForPp5v {
        self.0.ovp_for_pp5v().into()
    }
    /// Set [`Self::ovp_for_pp5v`] and return `self` to chain.
    pub fn set_ovp_for_pp5v(&mut self, value: OvpForPp5v) -> &mut Self {
        self.0.set_ovp_for_pp5v(value.into());
        self
    }

    /// VBUS OVP Usage.
    pub fn vbus_ovp_usage(&self) -> VbusOvpUsage {
        self.0.vbus_ovp_usage().into()
    }
    /// Set [`Self::vbus_ovp_usage`] and return `self` to chain.
    pub fn set_vbus_ovp_usage(&mut self, value: VbusOvpUsage) -> &mut Self {
        self.0.set_vbus_ovp_usage(value.into());
        self
    }

    /// Crossbar I2C Controller enable.
    pub fn crossbar_i2c_controller_enable(&self) -> bool {
        self.0.crossbar_i2c_controller_enable()
    }
    /// Set [`Self::crossbar_i2c_controller_enable`] and return `self` to chain.
    pub fn set_crossbar_i2c_controller_enable(&mut self, value: bool) -> &mut Self {
        self.0.set_crossbar_i2c_controller_enable(value);
        self
    }

    /// USB3 Rate.
    pub fn usb3_rate(&self) -> Usb3Rate {
        self.0.usb3_rate().into()
    }
    /// Set [`Self::usb3_rate`] and return `self` to chain.
    pub fn set_usb3_rate(&mut self, value: Usb3Rate) -> &mut Self {
        self.0.set_usb3_rate(value.into());
        self
    }

    /// DebugAccessory Support.
    pub fn debug_accessory_support(&self) -> bool {
        self.0.debug_accessory_support()
    }

    /// Set [`Self::debug_accessory_support`] and return `self` to chain.
    pub fn set_debug_accessory_support(&mut self, value: bool) -> &mut Self {
        self.0.set_debug_accessory_support(value);
        self
    }

    /// USB Communication Capable.
    pub fn usb_communication_capable(&self) -> bool {
        self.0.usb_communication_capable()
    }
    /// Set [`Self::usb_communication_capable`] and return `self` to chain.
    pub fn set_usb_communication_capable(&mut self, value: bool) -> &mut Self {
        self.0.set_usb_communication_capable(value);
        self
    }

    /// Disable PD.
    pub fn disable_pd(&self) -> bool {
        self.0.disable_pd()
    }
    /// Set [`Self::disable_pd`] and return `self` to chain.
    pub fn set_disable_pd(&mut self, value: bool) -> &mut Self {
        self.0.set_disable_pd(value);
        self
    }

    /// TypeC Support Options.
    pub fn typec_support_options(&self) -> TypeCSupportOptions {
        self.0.typec_support_options().into()
    }
    /// Set [`Self::typec_support_options`] and return `self` to chain.
    pub fn set_typec_support_options(&mut self, value: TypeCSupportOptions) -> &mut Self {
        self.0.set_typec_support_options(value.into());
        self
    }

    /// PP_EXT ActiveLow.
    pub fn pp_ext_active_low(&self) -> bool {
        self.0.pp_ext_active_low()
    }
    /// Set [`Self::pp_ext_active_low`] and return `self` to chain.
    pub fn set_pp_ext_active_low(&mut self, value: bool) -> &mut Self {
        self.0.set_pp_ext_active_low(value);
        self
    }

    /// Crossbar Type.
    pub fn crossbar_type(&self) -> CrossbarType {
        self.0.crossbar_type().into()
    }
    /// Set [`Self::crossbar_type`] and return `self` to chain.
    pub fn set_crossbar_type(&mut self, value: CrossbarType) -> &mut Self {
        self.0.set_crossbar_type(value.into());
        self
    }

    /// TypeC State machine.
    pub fn typec_state_machine(&self) -> TypeCStateMachine {
        self.0.typec_state_machine().into()
    }
    /// Set [`Self::typec_state_machine`] and return `self` to chain.
    pub fn set_typec_state_machine(&mut self, value: TypeCStateMachine) -> &mut Self {
        self.0.set_typec_state_machine(value.into());
        self
    }
}

impl From<[u8; LEN]> for PortConfig {
    fn from(value: [u8; LEN]) -> Self {
        PortConfig(PortConfigRaw(value))
    }
}

impl From<PortConfig> for [u8; LEN] {
    fn from(value: PortConfig) -> Self {
        value.0 .0
    }
}

impl Default for PortConfig {
    fn default() -> Self {
        Self::DEFAULT.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SbuMuxUsage tests
    #[test]
    fn test_sbu_mux_usage_from_u8() {
        assert_eq!(SbuMuxUsage::from(0x0), SbuMuxUsage::DisplayPortAux);
        assert_eq!(SbuMuxUsage::from(0x1), SbuMuxUsage::AllAux);
        assert_eq!(SbuMuxUsage::from(0x2), SbuMuxUsage::Reserved(0x2));
        assert_eq!(SbuMuxUsage::from(0x3), SbuMuxUsage::Reserved(0x3));
        assert_eq!(SbuMuxUsage::from(0xFF), SbuMuxUsage::Reserved(0x3));
    }

    #[test]
    fn test_sbu_mux_usage_to_u8() {
        assert_eq!(u8::from(SbuMuxUsage::DisplayPortAux), 0x0);
        assert_eq!(u8::from(SbuMuxUsage::AllAux), 0x1);
        assert_eq!(u8::from(SbuMuxUsage::Reserved(0x2)), 0x2);
        assert_eq!(u8::from(SbuMuxUsage::Reserved(0x3)), 0x3);
    }

    // SbuMuxDefaultSetting tests
    #[test]
    fn test_sbu_mux_default_setting_from_u8() {
        assert_eq!(SbuMuxDefaultSetting::from(0x0), SbuMuxDefaultSetting::DontChange);
        assert_eq!(SbuMuxDefaultSetting::from(0x1), SbuMuxDefaultSetting::DisableSbuMux);
        assert_eq!(SbuMuxDefaultSetting::from(0x2), SbuMuxDefaultSetting::EnableDbg);
        assert_eq!(SbuMuxDefaultSetting::from(0x3), SbuMuxDefaultSetting::Reserved(0x3));
        assert_eq!(SbuMuxDefaultSetting::from(0x7), SbuMuxDefaultSetting::Reserved(0x7));
        assert_eq!(SbuMuxDefaultSetting::from(0xFF), SbuMuxDefaultSetting::Reserved(0x7));
    }

    #[test]
    fn test_sbu_mux_default_setting_to_u8() {
        assert_eq!(u8::from(SbuMuxDefaultSetting::DontChange), 0x0);
        assert_eq!(u8::from(SbuMuxDefaultSetting::DisableSbuMux), 0x1);
        assert_eq!(u8::from(SbuMuxDefaultSetting::EnableDbg), 0x2);
        assert_eq!(u8::from(SbuMuxDefaultSetting::Reserved(0x5)), 0x5);
    }

    // SbuMuxDebugSetting tests
    #[test]
    fn test_sbu_mux_debug_setting_from_u8() {
        assert_eq!(SbuMuxDebugSetting::from(0x0), SbuMuxDebugSetting::DebugSnk);
        assert_eq!(SbuMuxDebugSetting::from(0x1), SbuMuxDebugSetting::UnorientedDebugSrc);
        assert_eq!(SbuMuxDebugSetting::from(0x2), SbuMuxDebugSetting::OrientedDebugSrc);
        assert_eq!(SbuMuxDebugSetting::from(0x3), SbuMuxDebugSetting::AnyDebug);
        assert_eq!(SbuMuxDebugSetting::from(0x4), SbuMuxDebugSetting::NeverEnable);
        assert_eq!(SbuMuxDebugSetting::from(0x5), SbuMuxDebugSetting::Reserved(0x5));
        assert_eq!(SbuMuxDebugSetting::from(0xFF), SbuMuxDebugSetting::Reserved(0x7));
    }

    #[test]
    fn test_sbu_mux_debug_setting_to_u8() {
        assert_eq!(u8::from(SbuMuxDebugSetting::DebugSnk), 0x0);
        assert_eq!(u8::from(SbuMuxDebugSetting::UnorientedDebugSrc), 0x1);
        assert_eq!(u8::from(SbuMuxDebugSetting::OrientedDebugSrc), 0x2);
        assert_eq!(u8::from(SbuMuxDebugSetting::AnyDebug), 0x3);
        assert_eq!(u8::from(SbuMuxDebugSetting::NeverEnable), 0x4);
        assert_eq!(u8::from(SbuMuxDebugSetting::Reserved(0x6)), 0x6);
    }

    // LevelShifterDirectionCfg tests
    #[test]
    fn test_level_shifter_direction_cfg_from_u8() {
        assert_eq!(LevelShifterDirectionCfg::from(0x0), LevelShifterDirectionCfg::Automatic);
        assert_eq!(
            LevelShifterDirectionCfg::from(0x1),
            LevelShifterDirectionCfg::GpioControlled
        );
        assert_eq!(
            LevelShifterDirectionCfg::from(0x2),
            LevelShifterDirectionCfg::I2cControlled
        );
        assert_eq!(
            LevelShifterDirectionCfg::from(0x3),
            LevelShifterDirectionCfg::Reserved(0x3)
        );
        assert_eq!(
            LevelShifterDirectionCfg::from(0xFF),
            LevelShifterDirectionCfg::Reserved(0x3)
        );
    }

    #[test]
    fn test_level_shifter_direction_cfg_to_u8() {
        assert_eq!(u8::from(LevelShifterDirectionCfg::Automatic), 0x0);
        assert_eq!(u8::from(LevelShifterDirectionCfg::GpioControlled), 0x1);
        assert_eq!(u8::from(LevelShifterDirectionCfg::I2cControlled), 0x2);
        assert_eq!(u8::from(LevelShifterDirectionCfg::Reserved(0x3)), 0x3);
    }

    // ExternalDcdcType tests
    #[test]
    fn test_external_dcdc_type_from_u8() {
        assert_eq!(ExternalDcdcType::from(0x0), ExternalDcdcType::MarinaBay);
        assert_eq!(ExternalDcdcType::from(0x1), ExternalDcdcType::Bq25758);
        assert_eq!(ExternalDcdcType::from(0x2), ExternalDcdcType::Reserved(0x2));
        assert_eq!(ExternalDcdcType::from(0xFF), ExternalDcdcType::Reserved(0xFF));
    }

    #[test]
    fn test_external_dcdc_type_to_u8() {
        assert_eq!(u8::from(ExternalDcdcType::MarinaBay), 0x0);
        assert_eq!(u8::from(ExternalDcdcType::Bq25758), 0x1);
        assert_eq!(u8::from(ExternalDcdcType::Reserved(0xAB)), 0xAB);
    }

    // TypeCStateMachine tests
    #[test]
    fn test_typec_state_machine_from_u8() {
        assert_eq!(TypeCStateMachine::from(0x0), TypeCStateMachine::Sink);
        assert_eq!(TypeCStateMachine::from(0x1), TypeCStateMachine::Source);
        assert_eq!(TypeCStateMachine::from(0x2), TypeCStateMachine::Drp);
        assert_eq!(TypeCStateMachine::from(0x3), TypeCStateMachine::Disabled);
        assert_eq!(TypeCStateMachine::from(0xFF), TypeCStateMachine::Disabled);
    }

    #[test]
    fn test_typec_state_machine_to_u8() {
        assert_eq!(u8::from(TypeCStateMachine::Sink), 0x0);
        assert_eq!(u8::from(TypeCStateMachine::Source), 0x1);
        assert_eq!(u8::from(TypeCStateMachine::Drp), 0x2);
        assert_eq!(u8::from(TypeCStateMachine::Disabled), 0x3);
    }

    // VbusSinkUvpTripHv tests
    #[test]
    fn test_vbus_sink_uvp_trip_hv_from_u8() {
        assert_eq!(VbusSinkUvpTripHv::from(0x0), VbusSinkUvpTripHv::Pct5);
        assert_eq!(VbusSinkUvpTripHv::from(0x1), VbusSinkUvpTripHv::Pct10);
        assert_eq!(VbusSinkUvpTripHv::from(0x2), VbusSinkUvpTripHv::Pct15);
        assert_eq!(VbusSinkUvpTripHv::from(0x3), VbusSinkUvpTripHv::Pct20);
        assert_eq!(VbusSinkUvpTripHv::from(0x4), VbusSinkUvpTripHv::Pct25);
        assert_eq!(VbusSinkUvpTripHv::from(0x5), VbusSinkUvpTripHv::Pct30);
        assert_eq!(VbusSinkUvpTripHv::from(0x6), VbusSinkUvpTripHv::Pct40);
        assert_eq!(VbusSinkUvpTripHv::from(0x7), VbusSinkUvpTripHv::Reserved(0x7));
        assert_eq!(VbusSinkUvpTripHv::from(0xFF), VbusSinkUvpTripHv::Reserved(0x7));
    }

    #[test]
    fn test_vbus_sink_uvp_trip_hv_to_u8() {
        assert_eq!(u8::from(VbusSinkUvpTripHv::Pct5), 0x0);
        assert_eq!(u8::from(VbusSinkUvpTripHv::Pct10), 0x1);
        assert_eq!(u8::from(VbusSinkUvpTripHv::Pct15), 0x2);
        assert_eq!(u8::from(VbusSinkUvpTripHv::Pct20), 0x3);
        assert_eq!(u8::from(VbusSinkUvpTripHv::Pct25), 0x4);
        assert_eq!(u8::from(VbusSinkUvpTripHv::Pct30), 0x5);
        assert_eq!(u8::from(VbusSinkUvpTripHv::Pct40), 0x6);
        assert_eq!(u8::from(VbusSinkUvpTripHv::Reserved(0x7)), 0x7);
    }

    // OvpForPp5v tests
    #[test]
    fn test_ovp_for_pp5v_from_u8() {
        assert_eq!(OvpForPp5v::from(0x0), OvpForPp5v::V5_25);
        assert_eq!(OvpForPp5v::from(0x1), OvpForPp5v::V5_5);
        assert_eq!(OvpForPp5v::from(0x2), OvpForPp5v::V5_8);
        assert_eq!(OvpForPp5v::from(0x3), OvpForPp5v::V6_1);
        assert_eq!(OvpForPp5v::from(0xFF), OvpForPp5v::V6_1);
    }

    #[test]
    fn test_ovp_for_pp5v_to_u8() {
        assert_eq!(u8::from(OvpForPp5v::V5_25), 0x0);
        assert_eq!(u8::from(OvpForPp5v::V5_5), 0x1);
        assert_eq!(u8::from(OvpForPp5v::V5_8), 0x2);
        assert_eq!(u8::from(OvpForPp5v::V6_1), 0x3);
    }

    // VbusOvpUsage tests
    #[test]
    fn test_vbus_ovp_usage_from_u8() {
        assert_eq!(VbusOvpUsage::from(0x0), VbusOvpUsage::Pct100);
        assert_eq!(VbusOvpUsage::from(0x1), VbusOvpUsage::Pct105);
        assert_eq!(VbusOvpUsage::from(0x2), VbusOvpUsage::Pct111);
        assert_eq!(VbusOvpUsage::from(0x3), VbusOvpUsage::Pct114);
        assert_eq!(VbusOvpUsage::from(0xFF), VbusOvpUsage::Pct114);
    }

    #[test]
    fn test_vbus_ovp_usage_to_u8() {
        assert_eq!(u8::from(VbusOvpUsage::Pct100), 0x0);
        assert_eq!(u8::from(VbusOvpUsage::Pct105), 0x1);
        assert_eq!(u8::from(VbusOvpUsage::Pct111), 0x2);
        assert_eq!(u8::from(VbusOvpUsage::Pct114), 0x3);
    }

    // Usb3Rate tests
    #[test]
    fn test_usb3_rate_from_u8() {
        assert_eq!(Usb3Rate::from(0x0), Usb3Rate::NotSupported);
        assert_eq!(Usb3Rate::from(0x1), Usb3Rate::Gen1);
        assert_eq!(Usb3Rate::from(0x2), Usb3Rate::Gen2);
        assert_eq!(Usb3Rate::from(0x3), Usb3Rate::Reserved(0x3));
        assert_eq!(Usb3Rate::from(0xFF), Usb3Rate::Reserved(0x3));
    }

    #[test]
    fn test_usb3_rate_to_u8() {
        assert_eq!(u8::from(Usb3Rate::NotSupported), 0x0);
        assert_eq!(u8::from(Usb3Rate::Gen1), 0x1);
        assert_eq!(u8::from(Usb3Rate::Gen2), 0x2);
        assert_eq!(u8::from(Usb3Rate::Reserved(0x3)), 0x3);
    }

    // TypeCSupportOptions tests
    #[test]
    fn test_typec_support_options_from_u8() {
        assert_eq!(TypeCSupportOptions::from(0x0), TypeCSupportOptions::None);
        assert_eq!(TypeCSupportOptions::from(0x1), TypeCSupportOptions::TrySrcDrp);
        assert_eq!(TypeCSupportOptions::from(0x2), TypeCSupportOptions::Reserved(0x2));
        assert_eq!(TypeCSupportOptions::from(0x3), TypeCSupportOptions::Reserved(0x3));
        assert_eq!(TypeCSupportOptions::from(0xFF), TypeCSupportOptions::Reserved(0x3));
    }

    #[test]
    fn test_typec_support_options_to_u8() {
        assert_eq!(u8::from(TypeCSupportOptions::None), 0x0);
        assert_eq!(u8::from(TypeCSupportOptions::TrySrcDrp), 0x1);
        assert_eq!(u8::from(TypeCSupportOptions::Reserved(0x2)), 0x2);
    }

    // CrossbarType tests
    #[test]
    fn test_crossbar_type_from_bool() {
        assert_eq!(CrossbarType::from(false), CrossbarType::Type1);
        assert_eq!(CrossbarType::from(true), CrossbarType::Type2);
    }

    #[test]
    fn test_crossbar_type_to_bool() {
        assert_eq!(bool::from(CrossbarType::Type1), false);
        assert_eq!(bool::from(CrossbarType::Type2), true);
    }

    #[test]
    fn test_port_config_default() {
        let config = PortConfig::default();
        assert_eq!(config.as_bytes(), &[0u8; LEN]);
    }

    #[test]
    fn test_port_config_round_trip() {
        let mut bytes = [0u8; LEN];
        bytes[0] = 0x42;
        bytes[17] = 0xFF;
        let config = PortConfig::from(bytes);
        let result: [u8; LEN] = config.into();
        assert_eq!(result, bytes);
    }
}
