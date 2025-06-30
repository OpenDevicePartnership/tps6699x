//! Types related to DP status register, 0x58

use bitfield::bitfield;

/// The address of the DP status register.
pub const ADDR: u8 = 0x58;

/// The length of the DP status register, in bytes.
///
/// This exceeds the maximum supported length by the [`device_driver`] crate.
pub const LEN: usize = 38;

bitfield! {
    /// DP status register
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct DpStatusRaw([u8]);
    impl Debug;
    /// DP Detected
    pub u8, dp_detected, set_dp_detected: 0, 0;
    /// DP Mode Active
    pub u8, dp_mode_active, set_dp_mode_active: 1, 1;
    /// DP Status TX
    pub u32, dp_status_tx, set_dp_status_tx: 39, 8;
    /// DP Status RX
    pub u32, dp_status_rx, set_dp_status_rx: 71, 40;
    /// DP Configure Message
    pub u32, dp_configure_message, set_dp_configure_message: 103, 72;
    /// DP Mode Data
    pub u32, dp_mode_data, set_dp_mode_data: 135, 104;
    /// DP Status to Plug
    pub u32, dp_status_to_plug, set_dp_status_to_plug: 167, 136;
    /// DP Status ACK from Plug
    pub u32, dp_status_ack_from_plug, set_dp_status_ack_from_plug: 199, 168;
    /// DP Config to Plug
    pub u32, dp_config_to_plug, set_dp_config_to_plug: 231, 200;
    /// DP Config from Plug
    pub u32, dp_config_from_plug, set_dp_config_from_plug: 263, 232;
    /// DP Mode Data SOPPrime
    pub u32, dp_mode_data_sopprime, set_dp_mode_data_sopprime: 295, 264;
    /// DP Signalling Rate
    pub u8, dp_signalling_rate, set_dp_signalling_rate: 299, 296;
    /// Cable UHBR13.5 Support
    pub u8, cable_uhbr13_5_support, set_cable_uhbr13_5_support: 300, 300;
    /// Cable Active Component
    pub u8, cable_active_component, set_cable_active_component: 302, 301;
    /// DP UFP VDO Version
    pub u8, dp_ufp_vdo_version, set_dp_ufp_vdo_version: 303, 303;
}

/// The actual flags bitfield is generic over the size of the array
/// Provide this type alias for convenience
pub type DpStatus = DpStatusRaw<[u8; LEN]>;
