//! Types and functions related to register 0x2D, boot flags
use bitfield::bitfield;

/// The address of the boot flags register.
pub const ADDR: u8 = 0x2D;

/// The length of the boot flags register, in bytes.
///
/// This exceeds the maximum supported length by the [`device_driver`] crate.
pub const LEN: usize = 44;

bitfield! {
    /// Boot flags register, bits 0-383
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct BootFlagsRaw([u8]);
    impl Debug;
    /// Current boot stage
    pub u8, boot_stage, set_boot_stage: 3, 0;
    /// Number of ports
    pub u8, total_num_pps, set_total_num_pps: 65, 64;
    /// 1 if external power path is present
    pub u8, is_ext_pp_present, set_is_ext_pp_present: 66, 66;
    /// Dead battery flag
    pub u8, dead_battery_flag, set_dead_battery_flag: 128, 128;
    /// Dead battery, port B is power provider
    pub u8, db_port_b_power_provider, set_db_port_b_power_provider: 129, 129;
    /// Dead battery, port A is power provider
    pub u8, db_port_a_power_provider, set_db_port_a_power_provider: 130, 130;
    /// Port A sink switch is enabled
    pub u8, port_a_sink_switch, set_port_a_sink_switch: 131, 131;
    /// Port B sink switch is enabled
    pub u8, port_b_sink_switch, set_port_b_sink_switch: 132, 132;
    /// Port A I2C1 Target Address
    pub u8, port_a_i2c1_trgt_addr, set_port_a_i2c1_trgt_addr: 167, 160;
    /// Port B I2C1 Target Address
    pub u8, port_b_i2c1_trgt_addr, set_port_b_i2c1_trgt_addr: 175, 168;
    /// Port A I2C2 Target Address
    pub u8, port_a_i2c2_trgt_addr, set_port_a_i2c2_trgt_addr: 183, 176;
    /// Port B I2C2 Target Address
    pub u8, port_b_i2c2_trgt_addr, set_port_b_i2c2_trgt_addr: 191, 184;
    /// Port A I2C4 Target Address
    pub u8, port_a_i2c4_trgt_addr, set_port_a_i2c4_trgt_addr: 199, 192;
    /// Port B I2C4 Target Address
    pub u8, port_b_i2c4_trgt_addr, set_port_b_i2c4_trgt_addr: 207, 200;
    /// Active Bank the device booted from
    pub u8, active_bank, set_active_bank: 225, 224;
    /// Asserted 1 if Bank 0 has valid Application Code
    pub u8, bank0_valid, set_bank0_valid: 226, 226;
    /// Asserted 1 if Bank 1 has valid Application Code
    pub u8, bank1_valid, set_bank1_valid: 227, 227;
    /// Application Firmware Version in Bank 0
    pub u32, bank0_fw_version, set_bank0_fw_version: 287, 256;
    /// Application Firmware Version in Bank 1
    pub u32, bank1_fw_version, set_bank1_fw_version: 319, 288;
    /// Raw ADCIN Value
    pub u16, adc_in_value, set_adc_in_value: 335, 320;
    /// ADCIN Index
    pub u16, adc_in_index, set_adc_in_index: 351, 336;
}

/// The actual flags bitfield is generic over the size of the array
/// Provide this type alias for convenience
pub type BootFlags = BootFlagsRaw<[u8; LEN]>;

#[cfg(test)]
mod tests {
    use super::{BootFlags, BootFlagsRaw, LEN};

    #[test]
    fn test_boot_flags_nonzero_roundtrip() {
        const EXPECTED: [u8; LEN] = [
            0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F, 0x00,
            0x00, 0x00, 0x20, 0x21, 0x30, 0x31, 0x40, 0x41, 0x00, 0x00, 0x0D, 0x00, 0x00, 0x00, 0x04, 0x03, 0x02, 0x01,
            0x08, 0x07, 0x06, 0x05, 0xCD, 0xAB, 0x34, 0x12,
        ];

        let mut flags: BootFlags = BootFlagsRaw([0u8; LEN]);
        flags.set_boot_stage(0xA);
        flags.set_total_num_pps(2);
        flags.set_is_ext_pp_present(1);
        flags.set_dead_battery_flag(1);
        flags.set_db_port_b_power_provider(1);
        flags.set_db_port_a_power_provider(1);
        flags.set_port_a_sink_switch(1);
        flags.set_port_b_sink_switch(1);
        flags.set_port_a_i2c1_trgt_addr(0x20);
        flags.set_port_b_i2c1_trgt_addr(0x21);
        flags.set_port_a_i2c2_trgt_addr(0x30);
        flags.set_port_b_i2c2_trgt_addr(0x31);
        flags.set_port_a_i2c4_trgt_addr(0x40);
        flags.set_port_b_i2c4_trgt_addr(0x41);
        flags.set_active_bank(1);
        flags.set_bank0_valid(1);
        flags.set_bank1_valid(1);
        flags.set_bank0_fw_version(0x01020304);
        flags.set_bank1_fw_version(0x05060708);
        flags.set_adc_in_value(0xABCD);
        flags.set_adc_in_index(0x1234);

        let bytes = flags.0;
        assert_eq!(bytes, EXPECTED);

        let flags2: BootFlags = BootFlagsRaw(EXPECTED);
        assert_eq!(flags2.boot_stage(), 0xA);
        assert_eq!(flags2.total_num_pps(), 2);
        assert_eq!(flags2.is_ext_pp_present(), 1);
        assert_eq!(flags2.dead_battery_flag(), 1);
        assert_eq!(flags2.db_port_b_power_provider(), 1);
        assert_eq!(flags2.db_port_a_power_provider(), 1);
        assert_eq!(flags2.port_a_sink_switch(), 1);
        assert_eq!(flags2.port_b_sink_switch(), 1);
        assert_eq!(flags2.port_a_i2c1_trgt_addr(), 0x20);
        assert_eq!(flags2.port_b_i2c1_trgt_addr(), 0x21);
        assert_eq!(flags2.port_a_i2c2_trgt_addr(), 0x30);
        assert_eq!(flags2.port_b_i2c2_trgt_addr(), 0x31);
        assert_eq!(flags2.port_a_i2c4_trgt_addr(), 0x40);
        assert_eq!(flags2.port_b_i2c4_trgt_addr(), 0x41);
        assert_eq!(flags2.active_bank(), 1);
        assert_eq!(flags2.bank0_valid(), 1);
        assert_eq!(flags2.bank1_valid(), 1);
        assert_eq!(flags2.bank0_fw_version(), 0x01020304);
        assert_eq!(flags2.bank1_fw_version(), 0x05060708);
        assert_eq!(flags2.adc_in_value(), 0xABCD);
        assert_eq!(flags2.adc_in_index(), 0x1234);
    }
}
