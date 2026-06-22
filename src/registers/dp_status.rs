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

#[cfg(test)]
mod tests {
    use super::{DpStatus, DpStatusRaw, LEN};

    #[test]
    fn test_dp_status_nonzero_roundtrip() {
        const EXPECTED: [u8; LEN] = [
            0x03, 0xDD, 0xCC, 0xBB, 0xAA, 0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55, 0xCC, 0xBB, 0xAA, 0x99, 0x00,
            0xFF, 0xEE, 0xDD, 0x78, 0x56, 0x34, 0x12, 0xF0, 0xDE, 0xBC, 0x9A, 0xDF, 0x9B, 0x57, 0x13, 0xE0, 0xAC, 0x68,
            0x24, 0xDA,
        ];

        let mut status: DpStatus = DpStatusRaw([0u8; LEN]);
        status.set_dp_detected(1);
        status.set_dp_mode_active(1);
        status.set_dp_status_tx(0xAABBCCDD);
        status.set_dp_status_rx(0x11223344);
        status.set_dp_configure_message(0x55667788);
        status.set_dp_mode_data(0x99AABBCC);
        status.set_dp_status_to_plug(0xDDEEFF00);
        status.set_dp_status_ack_from_plug(0x12345678);
        status.set_dp_config_to_plug(0x9ABCDEF0);
        status.set_dp_config_from_plug(0x13579BDF);
        status.set_dp_mode_data_sopprime(0x2468ACE0);
        status.set_dp_signalling_rate(0xA);
        status.set_cable_uhbr13_5_support(1);
        status.set_cable_active_component(2);
        status.set_dp_ufp_vdo_version(1);

        let bytes = status.0;
        assert_eq!(bytes, EXPECTED);

        let status2: DpStatus = DpStatusRaw(EXPECTED);
        assert_eq!(status2.dp_detected(), 1);
        assert_eq!(status2.dp_mode_active(), 1);
        assert_eq!(status2.dp_status_tx(), 0xAABBCCDD);
        assert_eq!(status2.dp_status_rx(), 0x11223344);
        assert_eq!(status2.dp_configure_message(), 0x55667788);
        assert_eq!(status2.dp_mode_data(), 0x99AABBCC);
        assert_eq!(status2.dp_status_to_plug(), 0xDDEEFF00);
        assert_eq!(status2.dp_status_ack_from_plug(), 0x12345678);
        assert_eq!(status2.dp_config_to_plug(), 0x9ABCDEF0);
        assert_eq!(status2.dp_config_from_plug(), 0x13579BDF);
        assert_eq!(status2.dp_mode_data_sopprime(), 0x2468ACE0);
        assert_eq!(status2.dp_signalling_rate(), 0xA);
        assert_eq!(status2.cable_uhbr13_5_support(), 1);
        assert_eq!(status2.cable_active_component(), 2);
        assert_eq!(status2.dp_ufp_vdo_version(), 1);
    }
}
