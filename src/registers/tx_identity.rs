//! Tx Identity register (`0x47`).
//!
//! This register's size exceeds the maximum supported length by the [`device_driver`] crate.
//!
//! Data to use for Discover Identity ACK. This data is sent in the response to Discover Identity
//! REQ message. Initialized by Application Customization. The PD controller is not designed to
//! transmit Product Type VDO as part of this ACK message, that is only required for Alternate
//! Mode Adaptors (AMA), VCONN Powered Devices (VPD), and cables.
//!
//! Writes to this register have no immediate effect. PD Controller will update and use the
//! contents of this register each time a Discover Identity SVDM is received before generating
//! the Discover Identity SVDM ACK message.

use bitfield::bitfield;

/// The address of the `Tx Identity` register.
pub const ADDR: u8 = 0x47;

/// The length of the `Tx Identity` register, in bytes.
///
/// This exceeds the maximum supported length by the [`device_driver`] crate.
pub const LEN: usize = 25;

/// Product Type for Downstream Facing Port (DFP) as defined in USB PD specification (bits 33-31).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ProductTypeDfp {
    /// Undefined DFP product type
    UndefinedDfp = 0x0,
    /// PD USB Hub
    PdUsbHub = 0x1,
    /// PD USB Host
    PdUsbHost = 0x2,
    /// Power Brick
    PowerBrick = 0x3,
    /// Alternate Mode Controller (AMC)
    Amc = 0x4,
    /// Reserved value
    Reserved(u8),
}

impl From<u8> for ProductTypeDfp {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => ProductTypeDfp::UndefinedDfp,
            0x1 => ProductTypeDfp::PdUsbHub,
            0x2 => ProductTypeDfp::PdUsbHost,
            0x3 => ProductTypeDfp::PowerBrick,
            0x4 => ProductTypeDfp::Amc,
            x => ProductTypeDfp::Reserved(x),
        }
    }
}

impl From<ProductTypeDfp> for u8 {
    fn from(value: ProductTypeDfp) -> Self {
        match value {
            ProductTypeDfp::UndefinedDfp => 0x0,
            ProductTypeDfp::PdUsbHub => 0x1,
            ProductTypeDfp::PdUsbHost => 0x2,
            ProductTypeDfp::PowerBrick => 0x3,
            ProductTypeDfp::Amc => 0x4,
            ProductTypeDfp::Reserved(x) => x,
        }
    }
}

/// Product Type for Upstream Facing Port (UFP) as defined in USB PD specification (bits 37-35).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ProductTypeUfp {
    /// Undefined UFP product type
    UndefinedUfp = 0x0,
    /// PD USB Hub
    PdUsbHub = 0x1,
    /// PD USB Peripheral
    PdUsbPeripheral = 0x2,
    /// Passive Cable (PSD - Passive SuperSpeed Device)
    Psd = 0x3,
    /// Reserved value
    Reserved(u8),
}

impl From<u8> for ProductTypeUfp {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => ProductTypeUfp::UndefinedUfp,
            0x1 => ProductTypeUfp::PdUsbHub,
            0x2 => ProductTypeUfp::PdUsbPeripheral,
            0x3 => ProductTypeUfp::Psd,
            x => ProductTypeUfp::Reserved(x),
        }
    }
}

impl From<ProductTypeUfp> for u8 {
    fn from(value: ProductTypeUfp) -> Self {
        match value {
            ProductTypeUfp::UndefinedUfp => 0x0,
            ProductTypeUfp::PdUsbHub => 0x1,
            ProductTypeUfp::PdUsbPeripheral => 0x2,
            ProductTypeUfp::Psd => 0x3,
            ProductTypeUfp::Reserved(x) => x,
        }
    }
}

bitfield! {
    /// Tx Identity register, bits 0-199
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct TxIdentityRaw([u8]);
    impl Debug;

    /// Number of valid VDOs in this register (bits 2-0)
    pub u8, number_valid_vdos, set_number_valid_vdos: 2, 0;
    /// Vendor ID as defined in USB PD specification (bits 23-8).
    /// This value is also used to populate VID in TX_MIDB_SOP, TX_SCEDB, and TX_SKEDB registers.
    pub u16, vendor_id, set_vendor_id: 23, 8;
    /// Product Type DFP as defined in USB PD specification (bits 33-31)
    pub u8, product_type_dfp, set_product_type_dfp: 33, 31;
    /// Assert this bit if Alternate Modes are supported (bit 34)
    pub bool, modal_operation_supported, set_modal_operation_supported: 34;
    /// Product Type UFP as defined in USB PD specification (bits 37-35)
    pub u8, product_type_ufp, set_product_type_ufp: 37, 35;
    /// Assert if USB communications capable as a device (bit 38)
    pub bool, usb_communication_capable_as_device, set_usb_communication_capable_as_device: 38;
    /// Assert if USB communications capable as a host (bit 39)
    pub bool, usb_communication_capable_as_host, set_usb_communication_capable_as_host: 39;
    /// 32-bit XID assigned by USB-IF (bits 71-40).
    /// This value is also used to populate XID in TX_SCEDB and TX_SKEDB registers.
    pub u32, certification_test_id, set_certification_test_id: 71, 40;
    /// FW version for the PD controller (read-only) (bits 87-72)
    pub u16, bcd_device, set_bcd_device: 87, 72;
    /// Product ID used to populate PID in other registers (bits 103-88).
    /// This value is also used to populate PID in TX_MIDB_SOP, TX_SCEDB, and TX_SKEDB registers.
    pub u16, usb_product_id, set_usb_product_id: 103, 88;
    /// UFP1 VDO (bits 135-104)
    pub u32, ufp1_vdo, set_ufp1_vdo: 135, 104;
    /// DFP1 VDO (bits 199-168)
    pub u32, dfp1_vdo, set_dfp1_vdo: 199, 168;
}

/// High-level wrapper around [`TxIdentityRaw`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TxIdentity(TxIdentityRaw<[u8; LEN]>);

impl TxIdentity {
    /// The default bytes for the Tx Identity register.
    const DEFAULT: [u8; LEN] = [
        0x06, // Byte 0: Number Valid VDOs = 6
        0x51, // Byte 1: Vendor ID lower byte (0x451 & 0xFF)
        0x04, // Byte 2: Vendor ID upper byte (0x451 >> 8)
        0x00, // Byte 3: Reserved
        0xD5, // Byte 4: USB comm host(1) + USB comm device(1) + UFP type(010) + Modal op(1) + DFP type(010)
        0x51, // Byte 5: Certification Test ID byte 0 (0x451 & 0xFF)
        0x04, // Byte 6: Certification Test ID byte 1 (0x451 >> 8)
        0x00, // Byte 7: Certification Test ID byte 2
        0x00, // Byte 8: Certification Test ID byte 3
        0x00, // Byte 9: BCD Device lower byte
        0x00, // Byte 10: BCD Device upper byte
        0x00, // Byte 11: USB Product ID lower byte
        0x00, // Byte 12: USB Product ID upper byte
        0x00, // Byte 13: UFP1 VDO byte 0
        0x00, // Byte 14: UFP1 VDO byte 1
        0x00, // Byte 15: UFP1 VDO byte 2
        0x00, // Byte 16: UFP1 VDO byte 3
        0x00, // Byte 17: Reserved byte 0
        0x00, // Byte 18: Reserved byte 1
        0x00, // Byte 19: Reserved byte 2
        0x00, // Byte 20: Reserved byte 3
        0x00, // Byte 21: DFP1 VDO byte 0
        0x00, // Byte 22: DFP1 VDO byte 1
        0x00, // Byte 23: DFP1 VDO byte 2
        0x00, // Byte 24: DFP1 VDO byte 3
    ];

    /// Get the raw byte representation of the Tx Identity register.
    pub fn as_bytes(&self) -> &[u8; LEN] {
        &self.0 .0
    }

    /// Get number of valid VDOs
    pub fn number_valid_vdos(&self) -> u8 {
        self.0.number_valid_vdos()
    }

    /// Set number of valid VDOs and return `self` to chain.
    pub fn set_number_valid_vdos(&mut self, value: u8) -> &mut Self {
        self.0.set_number_valid_vdos(value);
        self
    }

    /// Get vendor ID
    pub fn vendor_id(&self) -> u16 {
        self.0.vendor_id()
    }

    /// Set vendor ID and return `self` to chain.
    pub fn set_vendor_id(&mut self, value: u16) -> &mut Self {
        self.0.set_vendor_id(value);
        self
    }

    /// Get product type DFP
    pub fn product_type_dfp(&self) -> ProductTypeDfp {
        self.0.product_type_dfp().into()
    }

    /// Set product type DFP and return `self` to chain.
    pub fn set_product_type_dfp(&mut self, value: ProductTypeDfp) -> &mut Self {
        self.0.set_product_type_dfp(value.into());
        self
    }

    /// Get modal operation supported flag
    pub fn modal_operation_supported(&self) -> bool {
        self.0.modal_operation_supported()
    }

    /// Set modal operation supported flag and return `self` to chain.
    pub fn set_modal_operation_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_modal_operation_supported(value);
        self
    }

    /// Get product type UFP
    pub fn product_type_ufp(&self) -> ProductTypeUfp {
        self.0.product_type_ufp().into()
    }

    /// Set product type UFP and return `self` to chain.
    pub fn set_product_type_ufp(&mut self, value: ProductTypeUfp) -> &mut Self {
        self.0.set_product_type_ufp(value.into());
        self
    }

    /// Get USB communication capable as device flag
    pub fn usb_communication_capable_as_device(&self) -> bool {
        self.0.usb_communication_capable_as_device()
    }

    /// Set USB communication capable as device flag and return `self` to chain.
    pub fn set_usb_communication_capable_as_device(&mut self, value: bool) -> &mut Self {
        self.0.set_usb_communication_capable_as_device(value);
        self
    }

    /// Get USB communication capable as host flag
    pub fn usb_communication_capable_as_host(&self) -> bool {
        self.0.usb_communication_capable_as_host()
    }

    /// Set USB communication capable as host flag and return `self` to chain.
    pub fn set_usb_communication_capable_as_host(&mut self, value: bool) -> &mut Self {
        self.0.set_usb_communication_capable_as_host(value);
        self
    }

    /// Get certification test ID
    pub fn certification_test_id(&self) -> u32 {
        self.0.certification_test_id()
    }

    /// Set certification test ID and return `self` to chain.
    pub fn set_certification_test_id(&mut self, value: u32) -> &mut Self {
        self.0.set_certification_test_id(value);
        self
    }

    /// Get BCD device
    pub fn bcd_device(&self) -> u16 {
        self.0.bcd_device()
    }

    /// Set BCD device and return `self` to chain.
    pub fn set_bcd_device(&mut self, value: u16) -> &mut Self {
        self.0.set_bcd_device(value);
        self
    }

    /// Get USB product ID
    pub fn usb_product_id(&self) -> u16 {
        self.0.usb_product_id()
    }

    /// Set USB product ID and return `self` to chain.
    pub fn set_usb_product_id(&mut self, value: u16) -> &mut Self {
        self.0.set_usb_product_id(value);
        self
    }

    /// Get UFP1 VDO
    pub fn ufp1_vdo(&self) -> u32 {
        self.0.ufp1_vdo()
    }

    /// Set UFP1 VDO and return `self` to chain.
    pub fn set_ufp1_vdo(&mut self, value: u32) -> &mut Self {
        self.0.set_ufp1_vdo(value);
        self
    }

    /// Get DFP1 VDO
    pub fn dfp1_vdo(&self) -> u32 {
        self.0.dfp1_vdo()
    }

    /// Set DFP1 VDO and return `self` to chain.
    pub fn set_dfp1_vdo(&mut self, value: u32) -> &mut Self {
        self.0.set_dfp1_vdo(value);
        self
    }
}

impl From<[u8; LEN]> for TxIdentity {
    fn from(value: [u8; LEN]) -> Self {
        TxIdentity(TxIdentityRaw(value))
    }
}

impl From<TxIdentity> for [u8; LEN] {
    fn from(value: TxIdentity) -> Self {
        value.0 .0
    }
}

impl Default for TxIdentity {
    fn default() -> Self {
        Self::DEFAULT.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let default_register = TxIdentity::default();

        // Test Number Valid VDOs (bits 2-0) = 6h
        assert_eq!(default_register.number_valid_vdos(), 6);

        // Test Vendor ID (bits 23-8) = 451h
        assert_eq!(default_register.vendor_id(), 0x451);

        // Test Product Type DFP (bits 33-31) = 2h
        assert_eq!(default_register.product_type_dfp(), ProductTypeDfp::PdUsbHost);

        // Test Modal Operation Supported (bit 34) = 1h
        assert_eq!(default_register.modal_operation_supported(), true);

        // Test Product Type UFP (bits 37-35) = 2h
        assert_eq!(default_register.product_type_ufp(), ProductTypeUfp::PdUsbPeripheral);

        // Test USB Communication Capable as Device (bit 38) = 1h
        assert_eq!(default_register.usb_communication_capable_as_device(), true);

        // Test USB Communication Capable as Host (bit 39) = 1h
        assert_eq!(default_register.usb_communication_capable_as_host(), true);

        // Test Certification Test ID (bits 71-40) = 451h
        assert_eq!(default_register.certification_test_id(), 0x451);

        // Test BCD Device (bits 87-72) = 0h
        assert_eq!(default_register.bcd_device(), 0x0);

        // Test USB Product ID (bits 103-88) = 0h
        assert_eq!(default_register.usb_product_id(), 0x0);

        // Test UFP1 VDO (bits 135-104) = 0h
        assert_eq!(default_register.ufp1_vdo(), 0x0);

        // Test DFP1 VDO (bits 199-168) = 0h
        assert_eq!(default_register.dfp1_vdo(), 0x0);
    }

    #[test]
    fn test_raw_construction_matches_default() {
        // Create a raw register with all zeros
        let mut raw = TxIdentityRaw([0u8; LEN]);

        // Set all fields to their default values individually
        raw.set_number_valid_vdos(6); // VdoCount::Ack6Vdos = 6
        raw.set_vendor_id(0x451); // TI vendor ID
        raw.set_product_type_dfp(2); // ProductTypeDfp::PdUsbHost = 2
        raw.set_modal_operation_supported(true); // Modal operation supported
        raw.set_product_type_ufp(2); // ProductTypeUfp::PdUsbPeripheral = 2
        raw.set_usb_communication_capable_as_device(true); // USB comm capable as device
        raw.set_usb_communication_capable_as_host(true); // USB comm capable as host
        raw.set_certification_test_id(0x451); // Certification test ID
        raw.set_bcd_device(0x0); // BCD device version
        raw.set_usb_product_id(0x0); // USB product ID
        raw.set_ufp1_vdo(0x0); // UFP1 VDO
        raw.set_dfp1_vdo(0x0); // DFP1 VDO

        // Convert to TxIdentity
        let tx_identity = TxIdentity(raw);

        // Verify the raw bytes match the DEFAULT constant
        assert_eq!(tx_identity.as_bytes(), &TxIdentity::DEFAULT);

        // Also verify it matches a default-constructed TxIdentity
        let default_tx_identity = TxIdentity::default();
        assert_eq!(tx_identity.as_bytes(), default_tx_identity.as_bytes());
    }
}
