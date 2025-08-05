//! Tx Identity register (`0x47`).
//!
//! This register's size exceeds the maximum supported length by the [`device_driver`] crate.

use bitfield::bitfield;

/// The address of the `Tx Identity` register.
pub const ADDR: u8 = 0x47;

/// The length of the `Tx Identity` register, in bytes.
///
/// This exceeds the maximum supported length by the [`device_driver`] crate.
pub const LEN: usize = 49;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TxIdentityVdoCount {
    Nak = 0x0,
    Busy = 0x1,
    NotSupported = 0x2,
    Ack3Vdos = 0x3,
    Ack4Vdos = 0x4,
    Ack5Vdos = 0x5,
    Ack6Vdos = 0x6,
    Reserved(u8),
}

impl From<u8> for TxIdentityVdoCount {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => TxIdentityVdoCount::Nak,
            0x1 => TxIdentityVdoCount::Busy,
            0x2 => TxIdentityVdoCount::NotSupported,
            0x3 => TxIdentityVdoCount::Ack3Vdos,
            0x4 => TxIdentityVdoCount::Ack4Vdos,
            0x5 => TxIdentityVdoCount::Ack5Vdos,
            0x6 => TxIdentityVdoCount::Ack6Vdos,
            x => TxIdentityVdoCount::Reserved(x),
        }
    }
}

impl From<TxIdentityVdoCount> for u8 {
    fn from(value: TxIdentityVdoCount) -> Self {
        match value {
            TxIdentityVdoCount::Nak => 0x0,
            TxIdentityVdoCount::Busy => 0x1,
            TxIdentityVdoCount::NotSupported => 0x2,
            TxIdentityVdoCount::Ack3Vdos => 0x3,
            TxIdentityVdoCount::Ack4Vdos => 0x4,
            TxIdentityVdoCount::Ack5Vdos => 0x5,
            TxIdentityVdoCount::Ack6Vdos => 0x6,
            TxIdentityVdoCount::Reserved(x) => x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TxIdentityProductTypeDfp {
    UndefinedDfp = 0x0,
    PdUsbHub = 0x1,
    PdUsbHost = 0x2,
    PowerBrick = 0x3,
    Amc = 0x4,
    Reserved(u8),
}

impl From<u8> for TxIdentityProductTypeDfp {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => TxIdentityProductTypeDfp::UndefinedDfp,
            0x1 => TxIdentityProductTypeDfp::PdUsbHub,
            0x2 => TxIdentityProductTypeDfp::PdUsbHost,
            0x3 => TxIdentityProductTypeDfp::PowerBrick,
            0x4 => TxIdentityProductTypeDfp::Amc,
            x => TxIdentityProductTypeDfp::Reserved(x),
        }
    }
}

impl From<TxIdentityProductTypeDfp> for u8 {
    fn from(value: TxIdentityProductTypeDfp) -> Self {
        match value {
            TxIdentityProductTypeDfp::UndefinedDfp => 0x0,
            TxIdentityProductTypeDfp::PdUsbHub => 0x1,
            TxIdentityProductTypeDfp::PdUsbHost => 0x2,
            TxIdentityProductTypeDfp::PowerBrick => 0x3,
            TxIdentityProductTypeDfp::Amc => 0x4,
            TxIdentityProductTypeDfp::Reserved(x) => x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TxIdentityProductTypeUfp {
    UndefinedUfp = 0x0,
    PdUsbHub = 0x1,
    PdUsbPeripheral = 0x2,
    Psd = 0x3,
    Reserved(u8),
}

impl From<u8> for TxIdentityProductTypeUfp {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0x0 => TxIdentityProductTypeUfp::UndefinedUfp,
            0x1 => TxIdentityProductTypeUfp::PdUsbHub,
            0x2 => TxIdentityProductTypeUfp::PdUsbPeripheral,
            0x3 => TxIdentityProductTypeUfp::Psd,
            x => TxIdentityProductTypeUfp::Reserved(x),
        }
    }
}

impl From<TxIdentityProductTypeUfp> for u8 {
    fn from(value: TxIdentityProductTypeUfp) -> Self {
        match value {
            TxIdentityProductTypeUfp::UndefinedUfp => 0x0,
            TxIdentityProductTypeUfp::PdUsbHub => 0x1,
            TxIdentityProductTypeUfp::PdUsbPeripheral => 0x2,
            TxIdentityProductTypeUfp::Psd => 0x3,
            TxIdentityProductTypeUfp::Reserved(x) => x,
        }
    }
}

bitfield! {
    /// Tx Identity register, bits 0-391
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct TxIdentityRaw([u8]);
    impl Debug;

    /// Number of valid VDOs in this register
    pub u8, number_valid_vdos, set_number_valid_vdos: 3, 0;
    /// Vendor ID as defined in USB PD specification
    pub u16, vendor_id, set_vendor_id: 24, 8;
    /// Product Type DFP as defined in USB PD specification
    pub u8, product_type_dfp, set_product_type_dfp: 34, 31;
    /// Assert this bit if Alternate Modes are supported
    pub bool, modal_operation_supported, set_modal_operation_supported: 34;
    /// Product Type UFP as defined in USB PD specification
    pub u8, product_type_ufp, set_product_type_ufp: 38, 35;
    /// Assert if USB communications capable as a device
    pub bool, usb_communication_capable_as_device, set_usb_communication_capable_as_device: 38;
    /// Assert if USB communications capable as a host
    pub bool, usb_communication_capable_as_host, set_usb_communication_capable_as_host: 39;
    /// 32-bit XID assigned by USB-IF
    pub u32, certification_test_id, set_certification_test_id: 72, 40;
    /// FW version for the PD controller (read-only)
    pub u16, bcd_device, set_bcd_device: 88, 72;
    /// Product ID used to populate PID in other registers
    pub u16, usb_product_id, set_usb_product_id: 104, 88;
    /// UFP1 VDO
    pub u32, ufp1_vdo, set_ufp1_vdo: 136, 104;
    /// DFP1 VDO
    pub u32, dfp1_vdo, set_dfp1_vdo: 200, 168;
}

/// High-level wrapper around [`TxIdentityRaw`].
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TxIdentity(TxIdentityRaw<[u8; LEN]>);

impl TxIdentity {
    /// The default bytes for the Tx Identity register.
    const DEFAULT: [u8; LEN] = [0u8; LEN];

    /// Get the raw byte representation of the Tx Identity register.
    pub fn as_bytes(&self) -> &[u8; LEN] {
        &self.0 .0
    }

    /// Get number of valid VDOs
    pub fn number_valid_vdos(&self) -> TxIdentityVdoCount {
        self.0.number_valid_vdos().into()
    }

    /// Set number of valid VDOs and return `self` to chain.
    pub fn set_number_valid_vdos(&mut self, value: TxIdentityVdoCount) -> &mut Self {
        self.0.set_number_valid_vdos(value.into());
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
    pub fn product_type_dfp(&self) -> TxIdentityProductTypeDfp {
        self.0.product_type_dfp().into()
    }

    /// Set product type DFP and return `self` to chain.
    pub fn set_product_type_dfp(&mut self, value: TxIdentityProductTypeDfp) -> &mut Self {
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
    pub fn product_type_ufp(&self) -> TxIdentityProductTypeUfp {
        self.0.product_type_ufp().into()
    }

    /// Set product type UFP and return `self` to chain.
    pub fn set_product_type_ufp(&mut self, value: TxIdentityProductTypeUfp) -> &mut Self {
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
