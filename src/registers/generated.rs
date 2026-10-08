// This code was generated using device-driver `2.1.1` (),
// a tool distributed under MIT OR Apache-2.0 by Dion Dokter <dev@diondokter.nl>
// 
// For more information about device-driver, visit the website: https://device-driver.com

/// Root block of the Registers driver
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Registers<I> {
    interface: I,
    #[doc(hidden)]
    #[allow(unused)]
    base_address: u8,
}
impl<I> Registers<I> {
    /// Create a new instance of the device
    pub const fn new(interface: I) -> Self {
        Self { interface, base_address: 0 }
    }
    /// Drop the driver instance and reclaim the interface
    pub fn free(self) -> I {
        self.interface
    }
    /// Controller operation mode
    ///
    /// Register operation:
    /// - Address: `3`
    /// - Reset value: `0x0`
    #[doc(alias = "Mode")]
    pub fn mode(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, Mode, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 3;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || Mode::from([0, 0, 0, 0]),
        )
    }
    /// Customer use
    ///
    /// Register operation:
    /// - Address: `6`
    /// - Reset value: `0x0`
    #[doc(alias = "CustomerUse")]
    pub fn customer_use(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        CustomerUse,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || CustomerUse::from([0, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// Command 1 register
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0x0`
    #[doc(alias = "Cmd1")]
    pub fn cmd_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, Cmd1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || Cmd1::from([0, 0, 0, 0]),
        )
    }
    /// Boot FW version
    ///
    /// Register operation:
    /// - Address: `15`
    /// - Reset value: `0x0`
    #[doc(alias = "Version")]
    pub fn version(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        Version,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 15;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || Version::from([0, 0, 0, 0]),
        )
    }
    /// Asserted interrupts for I2C1
    ///
    /// Register operation:
    /// - Address: `20`
    /// - Reset value: `0x02000008`
    #[doc(alias = "IntEventBus1")]
    pub fn int_event_bus_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        IntEventBus1,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 20;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || IntEventBus1::from([8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// Masked interrupts for I2C1
    ///
    /// Register operation:
    /// - Address: `22`
    /// - Reset value: `0x0F000000CD30380A`
    #[doc(alias = "IntMaskBus1")]
    pub fn int_mask_bus_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        IntEventBus1,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 22;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || IntEventBus1::from([10, 56, 48, 205, 0, 0, 0, 15, 0, 0, 0]),
        )
    }
    /// Set Sx App Config - system power state for application configuration
    ///
    /// Register operation:
    /// - Address: `32`
    /// - Reset value: `0x0`
    #[doc(alias = "SxAppConfig")]
    pub fn sx_app_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        SxAppConfig,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 32;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || SxAppConfig::from([0, 0]),
        )
    }
    /// Interrupt clear for I2C1
    ///
    /// Register operation:
    /// - Address: `24`
    /// - Reset value: `0x0`
    #[doc(alias = "IntClearBus1")]
    pub fn int_clear_bus_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        IntEventBus1,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 24;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || IntEventBus1::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// Port status
    ///
    /// Register operation:
    /// - Address: `26`
    /// - Reset value: `0x0`
    #[doc(alias = "Status")]
    pub fn status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        Status,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 26;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || Status::from([0, 0, 0, 0, 0]),
        )
    }
    /// Power path status
    ///
    /// Register operation:
    /// - Address: `36`
    /// - Reset value: `0x0`
    #[doc(alias = "UsbStatus")]
    pub fn usb_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        UsbStatus,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 36;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || UsbStatus::from([0, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// Power path status
    ///
    /// Register operation:
    /// - Address: `38`
    /// - Reset value: `0x0`
    #[doc(alias = "PowerPathStatus")]
    pub fn power_path_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        PowerPathStatus,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 38;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || PowerPathStatus::from([0, 0, 0, 0, 0]),
        )
    }
    /// Global system configuration
    ///
    /// Register operation:
    /// - Address: `39`
    /// - Reset value: `0x10198C338905`
    #[doc(alias = "SystemConfig")]
    pub fn system_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        SystemConfig,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 39;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || SystemConfig::from([5, 137, 51, 140, 25, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// Port control
    ///
    /// Register operation:
    /// - Address: `41`
    /// - Reset value: `0x06000041C311`
    #[doc(alias = "PortControl")]
    pub fn port_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        PortControl,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 41;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || PortControl::from([17, 195, 65, 0, 0, 6, 0, 0]),
        )
    }
    /// Active PDO contract
    ///
    /// Register operation:
    /// - Address: `52`
    /// - Reset value: `0x0`
    #[doc(alias = "ActivePdoContract")]
    pub fn active_pdo_contract(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ActivePdoContract,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 52;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || ActivePdoContract::from([0, 0, 0, 0, 0, 0]),
        )
    }
    /// Active PDO contract
    ///
    /// Register operation:
    /// - Address: `53`
    /// - Reset value: `0x0`
    #[doc(alias = "ActiveRdoContract")]
    pub fn active_rdo_contract(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ActiveRdoContract,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 53;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || ActiveRdoContract::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// PD status
    ///
    /// Register operation:
    /// - Address: `64`
    /// - Reset value: `0x0`
    #[doc(alias = "PdStatus")]
    pub fn pd_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        PdStatus,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 64;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || PdStatus::from([0, 0, 0, 0]),
        )
    }
    /// Display Port Configuration
    ///
    /// Register operation:
    /// - Address: `81`
    /// - Reset value: `0x010100001C0603`
    #[doc(alias = "DpConfig")]
    pub fn dp_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        DpConfig,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 81;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || DpConfig::from([3, 6, 28, 0, 0, 1, 1, 0, 0, 0]),
        )
    }
    /// Register operation:
    /// - Address: `82`
    /// - Reset value: `0x0`
    #[doc(alias = "TbtConfig")]
    pub fn tbt_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        TbtConfig,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 82;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || TbtConfig::from([0, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// Register operation:
    /// - Address: `87`
    /// - Reset value: `0x0`
    #[doc(alias = "UserVidStatus")]
    pub fn user_vid_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        UserVidStatus,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 87;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || UserVidStatus::from([0, 0]),
        )
    }
    /// Intel VID status
    ///
    /// Register operation:
    /// - Address: `89`
    /// - Reset value: `0x0`
    #[doc(alias = "IntelVidStatus")]
    pub fn intel_vid_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        IntelVidStatus,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 89;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || IntelVidStatus::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// Received User SVID Attention VDM
    ///
    /// Register operation:
    /// - Address: `96`
    /// - Reset value: `0x0`
    #[doc(alias = "RxAttnVdm")]
    pub fn rx_attn_vdm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        RxAttnVdm,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 96;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || RxAttnVdm::from([0, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
    }
    /// Received ADO
    ///
    /// Register operation:
    /// - Address: `116`
    /// - Reset value: `0x0`
    #[doc(alias = "RxAdo")]
    pub fn rx_ado(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, RxAdo, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 116;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || RxAdo::from([0, 0, 0, 0]),
        )
    }
}
impl<I> ::device_driver::Block for Registers<I> {
    type Interface = I;
    type RegisterAddressType = u8;
    type CommandAddressType = u8;
    type BufferAddressType = u8;
    type RegisterAddressMode = ();
    fn interface(&mut self) -> &mut Self::Interface {
        &mut self.interface
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct RxAdo {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for RxAdo {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl RxAdo {
    /// `31:0` - Read the `ado` field.
    ///
    /// ADO
    #[doc(alias = "Ado")]
    #[must_use]
    pub fn ado(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:0` - Set the `ado` field.
    ///
    /// ADO
    #[doc(alias = "Ado")]
    pub fn set_ado(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for RxAdo {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for RxAdo {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<RxAdo> for [u8; 4] {
    fn from(val: RxAdo) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RxAdo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RxAdo");
        d.field("ado", &self.ado());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxAdo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxAdo {{ ");
        defmt::write!(f, "ado: {=u32}, ", & self.ado());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RxAdo {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RxAdo {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RxAdo {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RxAdo {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RxAdo {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RxAdo {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RxAdo {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct RxAttnVdm {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 9],
}
unsafe impl ::device_driver::Fieldset for RxAttnVdm {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 9] };
}
impl RxAttnVdm {
    /// `2:0` - Read the `num_of_valid_vdos` field.
    ///
    /// Number of valid Vdos received
    #[doc(alias = "NumOfValidVdos")]
    #[must_use]
    pub fn num_of_valid_vdos(&self) -> u8 {
        let start = 0;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `7:5` - Read the `seq_num` field.
    ///
    /// Increments by one every time this register is updated
    #[doc(alias = "SeqNum")]
    #[must_use]
    pub fn seq_num(&self) -> u8 {
        let start = 5;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `39:8` - Read the `vdm_header` field.
    ///
    /// VDM header, Rx Vdm data object 1
    #[doc(alias = "VdmHeader")]
    #[must_use]
    pub fn vdm_header(&self) -> u32 {
        let start = 8;
        let end = 39;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `71:40` - Read the `vdo` field.
    ///
    /// VDM data object, Rx Vdm data object 2
    #[doc(alias = "Vdo")]
    #[must_use]
    pub fn vdo(&self) -> u32 {
        let start = 40;
        let end = 71;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `2:0` - Set the `num_of_valid_vdos` field.
    ///
    /// Number of valid Vdos received
    #[doc(alias = "NumOfValidVdos")]
    pub fn set_num_of_valid_vdos(&mut self, value: u8) {
        let start = 0;
        let end = 2;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `7:5` - Set the `seq_num` field.
    ///
    /// Increments by one every time this register is updated
    #[doc(alias = "SeqNum")]
    pub fn set_seq_num(&mut self, value: u8) {
        let start = 5;
        let end = 7;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `39:8` - Set the `vdm_header` field.
    ///
    /// VDM header, Rx Vdm data object 1
    #[doc(alias = "VdmHeader")]
    pub fn set_vdm_header(&mut self, value: u32) {
        let start = 8;
        let end = 39;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `71:40` - Set the `vdo` field.
    ///
    /// VDM data object, Rx Vdm data object 2
    #[doc(alias = "Vdo")]
    pub fn set_vdo(&mut self, value: u32) {
        let start = 40;
        let end = 71;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for RxAttnVdm {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 9]> for RxAttnVdm {
    fn from(bits: [u8; 9]) -> Self {
        Self { bits }
    }
}
impl From<RxAttnVdm> for [u8; 9] {
    fn from(val: RxAttnVdm) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RxAttnVdm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RxAttnVdm");
        d.field("num_of_valid_vdos", &self.num_of_valid_vdos());
        d.field("seq_num", &self.seq_num());
        d.field("vdm_header", &self.vdm_header());
        d.field("vdo", &self.vdo());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxAttnVdm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxAttnVdm {{ ");
        defmt::write!(f, "num_of_valid_vdos: {=u8}, ", & self.num_of_valid_vdos());
        defmt::write!(f, "seq_num: {=u8}, ", & self.seq_num());
        defmt::write!(f, "vdm_header: {=u32}, ", & self.vdm_header());
        defmt::write!(f, "vdo: {=u32}, ", & self.vdo());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RxAttnVdm {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RxAttnVdm {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RxAttnVdm {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RxAttnVdm {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RxAttnVdm {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RxAttnVdm {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RxAttnVdm {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct IntelVidStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 11],
}
unsafe impl ::device_driver::Fieldset for IntelVidStatus {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 11] };
}
impl IntelVidStatus {
    /// `bit 0` - Read the `intel_vid_detected` field.
    ///
    /// Indicates if Intel VID is detected.
    #[doc(alias = "IntelVidDetected")]
    #[must_use]
    pub fn intel_vid_detected(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Read the `tbt_mode_active` field.
    ///
    /// Indicates if TBT Mode is active.
    #[doc(alias = "TbtModeActive")]
    #[must_use]
    pub fn tbt_mode_active(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 2` - Read the `forced_tbt_mode` field.
    ///
    /// Retimer in TBT state and ready for FW update
    #[doc(alias = "ForcedTbtMode")]
    #[must_use]
    pub fn forced_tbt_mode(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `38:8` - Read the `tbt_attention_data` field.
    ///
    /// Attention message contents
    #[doc(alias = "TbtAttentionData")]
    #[must_use]
    pub fn tbt_attention_data(&self) -> u32 {
        let start = 8;
        let end = 38;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `55:40` - Read the `tbt_enter_mode_data` field.
    ///
    /// Data for TBT Enter mode message
    #[doc(alias = "TbtEnterModeData")]
    #[must_use]
    pub fn tbt_enter_mode_data(&self) -> u16 {
        let start = 40;
        let end = 55;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `71:56` - Read the `tbt_mode_data_rx_sop` field.
    ///
    /// Data for Discover Modes response
    #[doc(alias = "TbtModeDataRxSop")]
    #[must_use]
    pub fn tbt_mode_data_rx_sop(&self) -> u16 {
        let start = 56;
        let end = 71;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `87:72` - Read the `tbt_mode_data_rx_sop_prime` field.
    ///
    /// Data for Discover Modes (SOP')
    #[doc(alias = "TbtModeDataRxSopPrime")]
    #[must_use]
    pub fn tbt_mode_data_rx_sop_prime(&self) -> u16 {
        let start = 72;
        let end = 87;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 0` - Set the `intel_vid_detected` field.
    ///
    /// Indicates if Intel VID is detected.
    #[doc(alias = "IntelVidDetected")]
    pub fn set_intel_vid_detected(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `tbt_mode_active` field.
    ///
    /// Indicates if TBT Mode is active.
    #[doc(alias = "TbtModeActive")]
    pub fn set_tbt_mode_active(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 2` - Set the `forced_tbt_mode` field.
    ///
    /// Retimer in TBT state and ready for FW update
    #[doc(alias = "ForcedTbtMode")]
    pub fn set_forced_tbt_mode(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `38:8` - Set the `tbt_attention_data` field.
    ///
    /// Attention message contents
    #[doc(alias = "TbtAttentionData")]
    pub fn set_tbt_attention_data(&mut self, value: u32) {
        let start = 8;
        let end = 38;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `55:40` - Set the `tbt_enter_mode_data` field.
    ///
    /// Data for TBT Enter mode message
    #[doc(alias = "TbtEnterModeData")]
    pub fn set_tbt_enter_mode_data(&mut self, value: u16) {
        let start = 40;
        let end = 55;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `71:56` - Set the `tbt_mode_data_rx_sop` field.
    ///
    /// Data for Discover Modes response
    #[doc(alias = "TbtModeDataRxSop")]
    pub fn set_tbt_mode_data_rx_sop(&mut self, value: u16) {
        let start = 56;
        let end = 71;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `87:72` - Set the `tbt_mode_data_rx_sop_prime` field.
    ///
    /// Data for Discover Modes (SOP')
    #[doc(alias = "TbtModeDataRxSopPrime")]
    pub fn set_tbt_mode_data_rx_sop_prime(&mut self, value: u16) {
        let start = 72;
        let end = 87;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for IntelVidStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 11]> for IntelVidStatus {
    fn from(bits: [u8; 11]) -> Self {
        Self { bits }
    }
}
impl From<IntelVidStatus> for [u8; 11] {
    fn from(val: IntelVidStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for IntelVidStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("IntelVidStatus");
        d.field("intel_vid_detected", &self.intel_vid_detected());
        d.field("tbt_mode_active", &self.tbt_mode_active());
        d.field("forced_tbt_mode", &self.forced_tbt_mode());
        d.field("tbt_attention_data", &self.tbt_attention_data());
        d.field("tbt_enter_mode_data", &self.tbt_enter_mode_data());
        d.field("tbt_mode_data_rx_sop", &self.tbt_mode_data_rx_sop());
        d.field("tbt_mode_data_rx_sop_prime", &self.tbt_mode_data_rx_sop_prime());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntelVidStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntelVidStatus {{ ");
        defmt::write!(f, "intel_vid_detected: {=bool}, ", & self.intel_vid_detected());
        defmt::write!(f, "tbt_mode_active: {=bool}, ", & self.tbt_mode_active());
        defmt::write!(f, "forced_tbt_mode: {=bool}, ", & self.forced_tbt_mode());
        defmt::write!(f, "tbt_attention_data: {=u32}, ", & self.tbt_attention_data());
        defmt::write!(f, "tbt_enter_mode_data: {=u16}, ", & self.tbt_enter_mode_data());
        defmt::write!(
            f, "tbt_mode_data_rx_sop: {=u16}, ", & self.tbt_mode_data_rx_sop()
        );
        defmt::write!(
            f, "tbt_mode_data_rx_sop_prime: {=u16}, ", & self
            .tbt_mode_data_rx_sop_prime()
        );
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for IntelVidStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for IntelVidStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for IntelVidStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for IntelVidStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for IntelVidStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for IntelVidStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for IntelVidStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct UserVidStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for UserVidStatus {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl UserVidStatus {
    /// `bit 0` - Read the `usvid_detected` field.
    ///
    /// Asserted when a User VID has been detected.
    #[doc(alias = "UsvidDetected")]
    #[must_use]
    pub fn usvid_detected(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Read the `usvid_active` field.
    ///
    /// Asserted when a User VID is active.
    #[doc(alias = "UsvidActive")]
    #[must_use]
    pub fn usvid_active(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `4:2` - Read the `usvid_error_code` field.
    ///
    /// Error code
    #[doc(alias = "UsvidErrorCode")]
    #[must_use]
    pub fn usvid_error_code(&self) -> u8 {
        let start = 2;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 9` - Read the `mode_1` field.
    ///
    /// Asserted when Mode1 has been entered
    #[doc(alias = "Mode1")]
    #[must_use]
    pub fn mode_1(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 10` - Read the `mode_2` field.
    ///
    /// Asserted when Mode2 has been entered
    #[doc(alias = "Mode2")]
    #[must_use]
    pub fn mode_2(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 11` - Read the `mode_3` field.
    ///
    /// Asserted when Mode3 has been entered
    #[doc(alias = "Mode3")]
    #[must_use]
    pub fn mode_3(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 12` - Read the `mode_4` field.
    ///
    /// Asserted when Mode4 has been entered
    #[doc(alias = "Mode4")]
    #[must_use]
    pub fn mode_4(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 0` - Set the `usvid_detected` field.
    ///
    /// Asserted when a User VID has been detected.
    #[doc(alias = "UsvidDetected")]
    pub fn set_usvid_detected(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `usvid_active` field.
    ///
    /// Asserted when a User VID is active.
    #[doc(alias = "UsvidActive")]
    pub fn set_usvid_active(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `4:2` - Set the `usvid_error_code` field.
    ///
    /// Error code
    #[doc(alias = "UsvidErrorCode")]
    pub fn set_usvid_error_code(&mut self, value: u8) {
        let start = 2;
        let end = 4;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 9` - Set the `mode_1` field.
    ///
    /// Asserted when Mode1 has been entered
    #[doc(alias = "Mode1")]
    pub fn set_mode_1(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 10` - Set the `mode_2` field.
    ///
    /// Asserted when Mode2 has been entered
    #[doc(alias = "Mode2")]
    pub fn set_mode_2(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 11` - Set the `mode_3` field.
    ///
    /// Asserted when Mode3 has been entered
    #[doc(alias = "Mode3")]
    pub fn set_mode_3(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 12` - Set the `mode_4` field.
    ///
    /// Asserted when Mode4 has been entered
    #[doc(alias = "Mode4")]
    pub fn set_mode_4(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for UserVidStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for UserVidStatus {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<UserVidStatus> for [u8; 2] {
    fn from(val: UserVidStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for UserVidStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("UserVidStatus");
        d.field("usvid_detected", &self.usvid_detected());
        d.field("usvid_active", &self.usvid_active());
        d.field("usvid_error_code", &self.usvid_error_code());
        d.field("mode_1", &self.mode_1());
        d.field("mode_2", &self.mode_2());
        d.field("mode_3", &self.mode_3());
        d.field("mode_4", &self.mode_4());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserVidStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserVidStatus {{ ");
        defmt::write!(f, "usvid_detected: {=bool}, ", & self.usvid_detected());
        defmt::write!(f, "usvid_active: {=bool}, ", & self.usvid_active());
        defmt::write!(f, "usvid_error_code: {=u8}, ", & self.usvid_error_code());
        defmt::write!(f, "mode_1: {=bool}, ", & self.mode_1());
        defmt::write!(f, "mode_2: {=bool}, ", & self.mode_2());
        defmt::write!(f, "mode_3: {=bool}, ", & self.mode_3());
        defmt::write!(f, "mode_4: {=bool}, ", & self.mode_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for UserVidStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for UserVidStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for UserVidStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for UserVidStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for UserVidStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for UserVidStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for UserVidStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct TbtConfig {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for TbtConfig {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl TbtConfig {
    /// `bit 0` - Read the `tbt_vid_en` field.
    ///
    /// Assert this bit to enable Thunderbolt VID.
    #[doc(alias = "TbtVidEn")]
    #[must_use]
    pub fn tbt_vid_en(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Read the `tbt_mode_en` field.
    ///
    /// Assert this bit to enable TBT mode.
    #[doc(alias = "TbtModeEn")]
    #[must_use]
    pub fn tbt_mode_en(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 2` - Read the `advertise_900_ma_implicit_contract` field.
    ///
    /// Advertise 900mA Implicit Contract.
    #[doc(alias = "Advertise900maImplicitContract")]
    #[must_use]
    pub fn advertise_900_ma_implicit_contract(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `6:3` - Read the `i_2_c_3_power_on_delay` field.
    ///
    /// Delay for the Controller I2C commands at power on.
    #[doc(alias = "I2C3PowerOnDelay")]
    #[must_use]
    pub fn i_2_c_3_power_on_delay(&self) -> u8 {
        let start = 3;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `pl_4_handling_en` field.
    ///
    /// Enable PL4 Handling.
    #[doc(alias = "PL4HandlingEn")]
    #[must_use]
    pub fn pl_4_handling_en(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 9` - Read the `tbt_emarker_override` field.
    ///
    /// Configuration for non-responsive Cable Plug.
    #[doc(alias = "TBTEmarkerOverride")]
    #[must_use]
    pub fn tbt_emarker_override(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 10` - Read the `an_min_power_required` field.
    ///
    /// Power required for TBT mode entry.
    #[doc(alias = "ANMinPowerRequired")]
    #[must_use]
    pub fn an_min_power_required(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 12` - Read the `dual_tbt_retimer_present` field.
    ///
    /// Assert this bit when there is a second TBT retimer on this port.
    #[doc(alias = "DualTbtRetimerPresent")]
    #[must_use]
    pub fn dual_tbt_retimer_present(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 13` - Read the `tbt_retimer_present` field.
    ///
    /// Assert this bit when there is a TBT retimer on this port.
    #[doc(alias = "TbtRetimerPresent")]
    #[must_use]
    pub fn tbt_retimer_present(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 14` - Read the `data_status_hpd_events` field.
    ///
    /// This bit controls how HPD events are configured.
    #[doc(alias = "DataStatusHPDEvents")]
    #[must_use]
    pub fn data_status_hpd_events(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 15` - Read the `retimer_compliance_support` field.
    ///
    /// Assert this bit causes the PD controller to place an attached Intel Retimer into compliance mode.
    #[doc(alias = "RetimerComplianceSupport")]
    #[must_use]
    pub fn retimer_compliance_support(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 16` - Read the `legacy_tbt_adapter` field.
    ///
    /// Legacy TBT Adapter.
    #[doc(alias = "LegacyTbtAdapter")]
    #[must_use]
    pub fn legacy_tbt_adapter(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 49` - Read the `tbt_auto_entry_allowed` field.
    ///
    /// Assert this bit to enable TBT auto-entry.
    #[doc(alias = "TbtAutoEntryAllowed")]
    #[must_use]
    pub fn tbt_auto_entry_allowed(&self) -> bool {
        let start = 49;
        let end = 49;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 54` - Read the `usb_data_path` field.
    ///
    /// USB data path support.
    #[doc(alias = "UsbDataPath")]
    #[must_use]
    pub fn usb_data_path(&self) -> TbtUsbDataPath {
        let start = 54;
        let end = 54;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `62:56` - Read the `source_vconn_delay` field.
    ///
    /// Configurable delay for BR.
    #[doc(alias = "SourceVCONNDelay")]
    #[must_use]
    pub fn source_vconn_delay(&self) -> u8 {
        let start = 56;
        let end = 62;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 0` - Set the `tbt_vid_en` field.
    ///
    /// Assert this bit to enable Thunderbolt VID.
    #[doc(alias = "TbtVidEn")]
    pub fn set_tbt_vid_en(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `tbt_mode_en` field.
    ///
    /// Assert this bit to enable TBT mode.
    #[doc(alias = "TbtModeEn")]
    pub fn set_tbt_mode_en(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 2` - Set the `advertise_900_ma_implicit_contract` field.
    ///
    /// Advertise 900mA Implicit Contract.
    #[doc(alias = "Advertise900maImplicitContract")]
    pub fn set_advertise_900_ma_implicit_contract(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `6:3` - Set the `i_2_c_3_power_on_delay` field.
    ///
    /// Delay for the Controller I2C commands at power on.
    #[doc(alias = "I2C3PowerOnDelay")]
    pub fn set_i_2_c_3_power_on_delay(&mut self, value: u8) {
        let start = 3;
        let end = 6;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `pl_4_handling_en` field.
    ///
    /// Enable PL4 Handling.
    #[doc(alias = "PL4HandlingEn")]
    pub fn set_pl_4_handling_en(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 9` - Set the `tbt_emarker_override` field.
    ///
    /// Configuration for non-responsive Cable Plug.
    #[doc(alias = "TBTEmarkerOverride")]
    pub fn set_tbt_emarker_override(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 10` - Set the `an_min_power_required` field.
    ///
    /// Power required for TBT mode entry.
    #[doc(alias = "ANMinPowerRequired")]
    pub fn set_an_min_power_required(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 12` - Set the `dual_tbt_retimer_present` field.
    ///
    /// Assert this bit when there is a second TBT retimer on this port.
    #[doc(alias = "DualTbtRetimerPresent")]
    pub fn set_dual_tbt_retimer_present(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 13` - Set the `tbt_retimer_present` field.
    ///
    /// Assert this bit when there is a TBT retimer on this port.
    #[doc(alias = "TbtRetimerPresent")]
    pub fn set_tbt_retimer_present(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 14` - Set the `data_status_hpd_events` field.
    ///
    /// This bit controls how HPD events are configured.
    #[doc(alias = "DataStatusHPDEvents")]
    pub fn set_data_status_hpd_events(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 15` - Set the `retimer_compliance_support` field.
    ///
    /// Assert this bit causes the PD controller to place an attached Intel Retimer into compliance mode.
    #[doc(alias = "RetimerComplianceSupport")]
    pub fn set_retimer_compliance_support(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 16` - Set the `legacy_tbt_adapter` field.
    ///
    /// Legacy TBT Adapter.
    #[doc(alias = "LegacyTbtAdapter")]
    pub fn set_legacy_tbt_adapter(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 49` - Set the `tbt_auto_entry_allowed` field.
    ///
    /// Assert this bit to enable TBT auto-entry.
    #[doc(alias = "TbtAutoEntryAllowed")]
    pub fn set_tbt_auto_entry_allowed(&mut self, value: bool) {
        let start = 49;
        let end = 49;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 54` - Set the `usb_data_path` field.
    ///
    /// USB data path support.
    #[doc(alias = "UsbDataPath")]
    pub fn set_usb_data_path(&mut self, value: TbtUsbDataPath) {
        let start = 54;
        let end = 54;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `62:56` - Set the `source_vconn_delay` field.
    ///
    /// Configurable delay for BR.
    #[doc(alias = "SourceVCONNDelay")]
    pub fn set_source_vconn_delay(&mut self, value: u8) {
        let start = 56;
        let end = 62;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for TbtConfig {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for TbtConfig {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<TbtConfig> for [u8; 8] {
    fn from(val: TbtConfig) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for TbtConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("TbtConfig");
        d.field("tbt_vid_en", &self.tbt_vid_en());
        d.field("tbt_mode_en", &self.tbt_mode_en());
        d.field(
            "advertise_900_ma_implicit_contract",
            &self.advertise_900_ma_implicit_contract(),
        );
        d.field("i_2_c_3_power_on_delay", &self.i_2_c_3_power_on_delay());
        d.field("pl_4_handling_en", &self.pl_4_handling_en());
        d.field("tbt_emarker_override", &self.tbt_emarker_override());
        d.field("an_min_power_required", &self.an_min_power_required());
        d.field("dual_tbt_retimer_present", &self.dual_tbt_retimer_present());
        d.field("tbt_retimer_present", &self.tbt_retimer_present());
        d.field("data_status_hpd_events", &self.data_status_hpd_events());
        d.field("retimer_compliance_support", &self.retimer_compliance_support());
        d.field("legacy_tbt_adapter", &self.legacy_tbt_adapter());
        d.field("tbt_auto_entry_allowed", &self.tbt_auto_entry_allowed());
        d.field("usb_data_path", &self.usb_data_path());
        d.field("source_vconn_delay", &self.source_vconn_delay());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TbtConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TbtConfig {{ ");
        defmt::write!(f, "tbt_vid_en: {=bool}, ", & self.tbt_vid_en());
        defmt::write!(f, "tbt_mode_en: {=bool}, ", & self.tbt_mode_en());
        defmt::write!(
            f, "advertise_900_ma_implicit_contract: {=bool}, ", & self
            .advertise_900_ma_implicit_contract()
        );
        defmt::write!(
            f, "i_2_c_3_power_on_delay: {=u8}, ", & self.i_2_c_3_power_on_delay()
        );
        defmt::write!(f, "pl_4_handling_en: {=bool}, ", & self.pl_4_handling_en());
        defmt::write!(
            f, "tbt_emarker_override: {=bool}, ", & self.tbt_emarker_override()
        );
        defmt::write!(
            f, "an_min_power_required: {=bool}, ", & self.an_min_power_required()
        );
        defmt::write!(
            f, "dual_tbt_retimer_present: {=bool}, ", & self.dual_tbt_retimer_present()
        );
        defmt::write!(f, "tbt_retimer_present: {=bool}, ", & self.tbt_retimer_present());
        defmt::write!(
            f, "data_status_hpd_events: {=bool}, ", & self.data_status_hpd_events()
        );
        defmt::write!(
            f, "retimer_compliance_support: {=bool}, ", & self
            .retimer_compliance_support()
        );
        defmt::write!(f, "legacy_tbt_adapter: {=bool}, ", & self.legacy_tbt_adapter());
        defmt::write!(
            f, "tbt_auto_entry_allowed: {=bool}, ", & self.tbt_auto_entry_allowed()
        );
        defmt::write!(f, "usb_data_path: {}, ", & self.usb_data_path());
        defmt::write!(f, "source_vconn_delay: {=u8}, ", & self.source_vconn_delay());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for TbtConfig {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for TbtConfig {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for TbtConfig {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for TbtConfig {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for TbtConfig {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for TbtConfig {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for TbtConfig {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DpConfig {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 10],
}
unsafe impl ::device_driver::Fieldset for DpConfig {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 10] };
}
impl DpConfig {
    /// `bit 0` - Read the `enable_dp_svid` field.
    ///
    /// Assert this bit to enable DisplayPort SVID.
    #[doc(alias = "EnableDpSvid")]
    #[must_use]
    pub fn enable_dp_svid(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Read the `enable_dp_mode` field.
    ///
    /// Assert this bit to enable DisplayPort Alternate mode.
    #[doc(alias = "EnableDpMode")]
    #[must_use]
    pub fn enable_dp_mode(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `9:8` - Read the `dp_port_capability` field.
    ///
    /// Display port capabilities
    #[doc(alias = "DpPortCapability")]
    #[must_use]
    pub fn dp_port_capability(&self) -> DpPortCapability {
        let start = 8;
        let end = 9;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `13:10` - Read the `dp_transport_signalling` field.
    ///
    /// Signaling for transport of DisplayPort protocol.
    #[doc(alias = "DpTransportSignalling")]
    #[must_use]
    pub fn dp_transport_signalling(&self) -> DpTransportSignalling {
        let start = 10;
        let end = 13;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 15` - Read the `usb_data_path` field.
    ///
    /// USB data path support.
    #[doc(alias = "UsbDataPath")]
    #[must_use]
    pub fn usb_data_path(&self) -> DpUsbDataPath {
        let start = 15;
        let end = 15;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `23:16` - Read the `dfpd_pin_assignment` field.
    ///
    /// DFP_D Pin Assignments Supported. Each bit corresponds to an allowed pin assignment. Multiple pin assignments may be allowed.
    #[doc(alias = "DfpdPinAssignment")]
    #[must_use]
    pub fn dfpd_pin_assignment(&self) -> u8 {
        let start = 16;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:24` - Read the `ufpd_pin_assignment` field.
    ///
    /// UFP_D Pin Assignments Supported. Each bit corresponds to an allowed pin assignment. Multiple pin assignments may be allowed.
    #[doc(alias = "UfpdPinAssignment")]
    #[must_use]
    pub fn ufpd_pin_assignment(&self) -> u8 {
        let start = 24;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 32` - Read the `multi_function_preferred` field.
    ///
    /// Assert this bit if multi-function is preferred.
    #[doc(alias = "MultiFunctionPreferred")]
    #[must_use]
    pub fn multi_function_preferred(&self) -> bool {
        let start = 32;
        let end = 32;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `36:35` - Read the `dfpd_ufpd_connection_status` field.
    ///
    /// This field indicates the status of the connection.
    #[doc(alias = "DfpdUfpdConnectionStatus")]
    #[must_use]
    pub fn dfpd_ufpd_connection_status(&self) -> DfpdUfpdConnected {
        let start = 35;
        let end = 36;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `38:37` - Read the `dp_vdo_version` field.
    ///
    /// DP VDO Version
    #[doc(alias = "DpVdoVersion")]
    #[must_use]
    pub fn dp_vdo_version(&self) -> DpVdoVersion {
        let start = 37;
        let end = 38;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 40` - Read the `dp_mode_auto_entry_allowed` field.
    ///
    /// Assert this bit to enable auto-entry.
    #[doc(alias = "DpModeAutoEntryAllowed")]
    #[must_use]
    pub fn dp_mode_auto_entry_allowed(&self) -> bool {
        let start = 40;
        let end = 40;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `49:48` - Read the `port_capability` field.
    ///
    /// Port Capability
    #[doc(alias = "PortCapability")]
    #[must_use]
    pub fn port_capability(&self) -> PortCapability {
        let start = 48;
        let end = 49;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `53:50` - Read the `transport_signalling` field.
    ///
    /// Transport Signalling
    #[doc(alias = "TransportSignalling")]
    #[must_use]
    pub fn transport_signalling(&self) -> u8 {
        let start = 50;
        let end = 53;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 54` - Read the `receptacle_indication` field.
    ///
    /// Receptacle Indication.
    #[doc(alias = "ReceptacleIndication")]
    #[must_use]
    pub fn receptacle_indication(&self) -> ReceptacleIndication {
        let start = 54;
        let end = 54;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 55` - Read the `usb_2_signalling_not_used` field.
    ///
    /// USB2 signaling requirement on A6 - A7 or B6 - B7 (D+/D-) while in DP configuration.
    #[doc(alias = "Usb2SignallingNotUsed")]
    #[must_use]
    pub fn usb_2_signalling_not_used(&self) -> Usb2SignalingNotUsed {
        let start = 55;
        let end = 55;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `63:56` - Read the `dp_source_device_pin_assignments_supported` field.
    ///
    /// DP Source DevicePinAssignments Supported
    #[doc(alias = "DpSourceDevicePinAssignmentsSupported")]
    #[must_use]
    pub fn dp_source_device_pin_assignments_supported(&self) -> u8 {
        let start = 56;
        let end = 63;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `71:64` - Read the `dp_sink_device_pin_assignments_supported` field.
    ///
    /// DP Sink DevicePinAssignments Supported
    #[doc(alias = "DpSinkDevicePinAssignmentsSupported")]
    #[must_use]
    pub fn dp_sink_device_pin_assignments_supported(&self) -> u8 {
        let start = 64;
        let end = 71;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 74` - Read the `uhbr_13` field.
    ///
    /// UHBR13
    #[doc(alias = "Uhbr13")]
    #[must_use]
    pub fn uhbr_13(&self) -> bool {
        let start = 74;
        let end = 74;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `77:76` - Read the `active_component` field.
    ///
    /// ActiveComponent
    #[doc(alias = "ActiveComponent")]
    #[must_use]
    pub fn active_component(&self) -> ActiveComponent {
        let start = 76;
        let end = 77;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `79:78` - Read the `dpam_version` field.
    ///
    /// DPAM version
    #[doc(alias = "DpamVersion")]
    #[must_use]
    pub fn dpam_version(&self) -> DpamVersion {
        let start = 78;
        let end = 79;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 0` - Set the `enable_dp_svid` field.
    ///
    /// Assert this bit to enable DisplayPort SVID.
    #[doc(alias = "EnableDpSvid")]
    pub fn set_enable_dp_svid(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `enable_dp_mode` field.
    ///
    /// Assert this bit to enable DisplayPort Alternate mode.
    #[doc(alias = "EnableDpMode")]
    pub fn set_enable_dp_mode(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `9:8` - Set the `dp_port_capability` field.
    ///
    /// Display port capabilities
    #[doc(alias = "DpPortCapability")]
    pub fn set_dp_port_capability(&mut self, value: DpPortCapability) {
        let start = 8;
        let end = 9;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `13:10` - Set the `dp_transport_signalling` field.
    ///
    /// Signaling for transport of DisplayPort protocol.
    #[doc(alias = "DpTransportSignalling")]
    pub fn set_dp_transport_signalling(&mut self, value: DpTransportSignalling) {
        let start = 10;
        let end = 13;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 15` - Set the `usb_data_path` field.
    ///
    /// USB data path support.
    #[doc(alias = "UsbDataPath")]
    pub fn set_usb_data_path(&mut self, value: DpUsbDataPath) {
        let start = 15;
        let end = 15;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `23:16` - Set the `dfpd_pin_assignment` field.
    ///
    /// DFP_D Pin Assignments Supported. Each bit corresponds to an allowed pin assignment. Multiple pin assignments may be allowed.
    #[doc(alias = "DfpdPinAssignment")]
    pub fn set_dfpd_pin_assignment(&mut self, value: u8) {
        let start = 16;
        let end = 23;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `31:24` - Set the `ufpd_pin_assignment` field.
    ///
    /// UFP_D Pin Assignments Supported. Each bit corresponds to an allowed pin assignment. Multiple pin assignments may be allowed.
    #[doc(alias = "UfpdPinAssignment")]
    pub fn set_ufpd_pin_assignment(&mut self, value: u8) {
        let start = 24;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 32` - Set the `multi_function_preferred` field.
    ///
    /// Assert this bit if multi-function is preferred.
    #[doc(alias = "MultiFunctionPreferred")]
    pub fn set_multi_function_preferred(&mut self, value: bool) {
        let start = 32;
        let end = 32;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `36:35` - Set the `dfpd_ufpd_connection_status` field.
    ///
    /// This field indicates the status of the connection.
    #[doc(alias = "DfpdUfpdConnectionStatus")]
    pub fn set_dfpd_ufpd_connection_status(&mut self, value: DfpdUfpdConnected) {
        let start = 35;
        let end = 36;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `38:37` - Set the `dp_vdo_version` field.
    ///
    /// DP VDO Version
    #[doc(alias = "DpVdoVersion")]
    pub fn set_dp_vdo_version(&mut self, value: DpVdoVersion) {
        let start = 37;
        let end = 38;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 40` - Set the `dp_mode_auto_entry_allowed` field.
    ///
    /// Assert this bit to enable auto-entry.
    #[doc(alias = "DpModeAutoEntryAllowed")]
    pub fn set_dp_mode_auto_entry_allowed(&mut self, value: bool) {
        let start = 40;
        let end = 40;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `49:48` - Set the `port_capability` field.
    ///
    /// Port Capability
    #[doc(alias = "PortCapability")]
    pub fn set_port_capability(&mut self, value: PortCapability) {
        let start = 48;
        let end = 49;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `53:50` - Set the `transport_signalling` field.
    ///
    /// Transport Signalling
    #[doc(alias = "TransportSignalling")]
    pub fn set_transport_signalling(&mut self, value: u8) {
        let start = 50;
        let end = 53;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 54` - Set the `receptacle_indication` field.
    ///
    /// Receptacle Indication.
    #[doc(alias = "ReceptacleIndication")]
    pub fn set_receptacle_indication(&mut self, value: ReceptacleIndication) {
        let start = 54;
        let end = 54;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 55` - Set the `usb_2_signalling_not_used` field.
    ///
    /// USB2 signaling requirement on A6 - A7 or B6 - B7 (D+/D-) while in DP configuration.
    #[doc(alias = "Usb2SignallingNotUsed")]
    pub fn set_usb_2_signalling_not_used(&mut self, value: Usb2SignalingNotUsed) {
        let start = 55;
        let end = 55;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `63:56` - Set the `dp_source_device_pin_assignments_supported` field.
    ///
    /// DP Source DevicePinAssignments Supported
    #[doc(alias = "DpSourceDevicePinAssignmentsSupported")]
    pub fn set_dp_source_device_pin_assignments_supported(&mut self, value: u8) {
        let start = 56;
        let end = 63;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `71:64` - Set the `dp_sink_device_pin_assignments_supported` field.
    ///
    /// DP Sink DevicePinAssignments Supported
    #[doc(alias = "DpSinkDevicePinAssignmentsSupported")]
    pub fn set_dp_sink_device_pin_assignments_supported(&mut self, value: u8) {
        let start = 64;
        let end = 71;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 74` - Set the `uhbr_13` field.
    ///
    /// UHBR13
    #[doc(alias = "Uhbr13")]
    pub fn set_uhbr_13(&mut self, value: bool) {
        let start = 74;
        let end = 74;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `77:76` - Set the `active_component` field.
    ///
    /// ActiveComponent
    #[doc(alias = "ActiveComponent")]
    pub fn set_active_component(&mut self, value: ActiveComponent) {
        let start = 76;
        let end = 77;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `79:78` - Set the `dpam_version` field.
    ///
    /// DPAM version
    #[doc(alias = "DpamVersion")]
    pub fn set_dpam_version(&mut self, value: DpamVersion) {
        let start = 78;
        let end = 79;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for DpConfig {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 10]> for DpConfig {
    fn from(bits: [u8; 10]) -> Self {
        Self { bits }
    }
}
impl From<DpConfig> for [u8; 10] {
    fn from(val: DpConfig) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for DpConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("DpConfig");
        d.field("enable_dp_svid", &self.enable_dp_svid());
        d.field("enable_dp_mode", &self.enable_dp_mode());
        d.field("dp_port_capability", &self.dp_port_capability());
        d.field("dp_transport_signalling", &self.dp_transport_signalling());
        d.field("usb_data_path", &self.usb_data_path());
        d.field("dfpd_pin_assignment", &self.dfpd_pin_assignment());
        d.field("ufpd_pin_assignment", &self.ufpd_pin_assignment());
        d.field("multi_function_preferred", &self.multi_function_preferred());
        d.field("dfpd_ufpd_connection_status", &self.dfpd_ufpd_connection_status());
        d.field("dp_vdo_version", &self.dp_vdo_version());
        d.field("dp_mode_auto_entry_allowed", &self.dp_mode_auto_entry_allowed());
        d.field("port_capability", &self.port_capability());
        d.field("transport_signalling", &self.transport_signalling());
        d.field("receptacle_indication", &self.receptacle_indication());
        d.field("usb_2_signalling_not_used", &self.usb_2_signalling_not_used());
        d.field(
            "dp_source_device_pin_assignments_supported",
            &self.dp_source_device_pin_assignments_supported(),
        );
        d.field(
            "dp_sink_device_pin_assignments_supported",
            &self.dp_sink_device_pin_assignments_supported(),
        );
        d.field("uhbr_13", &self.uhbr_13());
        d.field("active_component", &self.active_component());
        d.field("dpam_version", &self.dpam_version());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DpConfig {{ ");
        defmt::write!(f, "enable_dp_svid: {=bool}, ", & self.enable_dp_svid());
        defmt::write!(f, "enable_dp_mode: {=bool}, ", & self.enable_dp_mode());
        defmt::write!(f, "dp_port_capability: {}, ", & self.dp_port_capability());
        defmt::write!(
            f, "dp_transport_signalling: {}, ", & self.dp_transport_signalling()
        );
        defmt::write!(f, "usb_data_path: {}, ", & self.usb_data_path());
        defmt::write!(f, "dfpd_pin_assignment: {=u8}, ", & self.dfpd_pin_assignment());
        defmt::write!(f, "ufpd_pin_assignment: {=u8}, ", & self.ufpd_pin_assignment());
        defmt::write!(
            f, "multi_function_preferred: {=bool}, ", & self.multi_function_preferred()
        );
        defmt::write!(
            f, "dfpd_ufpd_connection_status: {}, ", & self.dfpd_ufpd_connection_status()
        );
        defmt::write!(f, "dp_vdo_version: {}, ", & self.dp_vdo_version());
        defmt::write!(
            f, "dp_mode_auto_entry_allowed: {=bool}, ", & self
            .dp_mode_auto_entry_allowed()
        );
        defmt::write!(f, "port_capability: {}, ", & self.port_capability());
        defmt::write!(f, "transport_signalling: {=u8}, ", & self.transport_signalling());
        defmt::write!(f, "receptacle_indication: {}, ", & self.receptacle_indication());
        defmt::write!(
            f, "usb_2_signalling_not_used: {}, ", & self.usb_2_signalling_not_used()
        );
        defmt::write!(
            f, "dp_source_device_pin_assignments_supported: {=u8}, ", & self
            .dp_source_device_pin_assignments_supported()
        );
        defmt::write!(
            f, "dp_sink_device_pin_assignments_supported: {=u8}, ", & self
            .dp_sink_device_pin_assignments_supported()
        );
        defmt::write!(f, "uhbr_13: {=bool}, ", & self.uhbr_13());
        defmt::write!(f, "active_component: {}, ", & self.active_component());
        defmt::write!(f, "dpam_version: {}, ", & self.dpam_version());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for DpConfig {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for DpConfig {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for DpConfig {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for DpConfig {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for DpConfig {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for DpConfig {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for DpConfig {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PdStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for PdStatus {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl PdStatus {
    /// `3:2` - Read the `cc_pull_up` field.
    ///
    /// CC pull up value
    #[doc(alias = "CcPullUp")]
    #[must_use]
    pub fn cc_pull_up(&self) -> PdCcPullUp {
        let start = 2;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `5:4` - Read the `port_type` field.
    ///
    /// Port type
    #[doc(alias = "PortType")]
    #[must_use]
    pub fn port_type(&self) -> PdPortType {
        let start = 4;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 6` - Read the `is_source` field.
    ///
    /// Present role
    #[doc(alias = "IsSource")]
    #[must_use]
    pub fn is_source(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `12:8` - Read the `soft_reset_details` field.
    ///
    /// Soft reset details
    #[doc(alias = "SoftResetDetails")]
    #[must_use]
    pub fn soft_reset_details(&self) -> PdSoftResetDetails {
        let start = 8;
        let end = 12;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `21:16` - Read the `hard_reset_details` field.
    ///
    /// Soft reset details
    #[doc(alias = "HardResetDetails")]
    #[must_use]
    pub fn hard_reset_details(&self) -> PdHardResetDetails {
        let start = 16;
        let end = 21;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `27:22` - Read the `error_recovery_details` field.
    ///
    /// Soft reset details
    #[doc(alias = "ErrorRecoveryDetails")]
    #[must_use]
    pub fn error_recovery_details(&self) -> PdErrorRecoveryDetails {
        let start = 22;
        let end = 27;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `30:28` - Read the `data_reset_details` field.
    ///
    /// Data reset details
    #[doc(alias = "DataResetDetails")]
    #[must_use]
    pub fn data_reset_details(&self) -> PdDataResetDetails {
        let start = 28;
        let end = 30;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `3:2` - Set the `cc_pull_up` field.
    ///
    /// CC pull up value
    #[doc(alias = "CcPullUp")]
    pub fn set_cc_pull_up(&mut self, value: PdCcPullUp) {
        let start = 2;
        let end = 3;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `5:4` - Set the `port_type` field.
    ///
    /// Port type
    #[doc(alias = "PortType")]
    pub fn set_port_type(&mut self, value: PdPortType) {
        let start = 4;
        let end = 5;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `is_source` field.
    ///
    /// Present role
    #[doc(alias = "IsSource")]
    pub fn set_is_source(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `12:8` - Set the `soft_reset_details` field.
    ///
    /// Soft reset details
    #[doc(alias = "SoftResetDetails")]
    pub fn set_soft_reset_details(&mut self, value: PdSoftResetDetails) {
        let start = 8;
        let end = 12;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `21:16` - Set the `hard_reset_details` field.
    ///
    /// Soft reset details
    #[doc(alias = "HardResetDetails")]
    pub fn set_hard_reset_details(&mut self, value: PdHardResetDetails) {
        let start = 16;
        let end = 21;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `27:22` - Set the `error_recovery_details` field.
    ///
    /// Soft reset details
    #[doc(alias = "ErrorRecoveryDetails")]
    pub fn set_error_recovery_details(&mut self, value: PdErrorRecoveryDetails) {
        let start = 22;
        let end = 27;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `30:28` - Set the `data_reset_details` field.
    ///
    /// Data reset details
    #[doc(alias = "DataResetDetails")]
    pub fn set_data_reset_details(&mut self, value: PdDataResetDetails) {
        let start = 28;
        let end = 30;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for PdStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for PdStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<PdStatus> for [u8; 4] {
    fn from(val: PdStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PdStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PdStatus");
        d.field("cc_pull_up", &self.cc_pull_up());
        d.field("port_type", &self.port_type());
        d.field("is_source", &self.is_source());
        d.field("soft_reset_details", &self.soft_reset_details());
        d.field("hard_reset_details", &self.hard_reset_details());
        d.field("error_recovery_details", &self.error_recovery_details());
        d.field("data_reset_details", &self.data_reset_details());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PdStatus {{ ");
        defmt::write!(f, "cc_pull_up: {}, ", & self.cc_pull_up());
        defmt::write!(f, "port_type: {}, ", & self.port_type());
        defmt::write!(f, "is_source: {=bool}, ", & self.is_source());
        defmt::write!(f, "soft_reset_details: {}, ", & self.soft_reset_details());
        defmt::write!(f, "hard_reset_details: {}, ", & self.hard_reset_details());
        defmt::write!(
            f, "error_recovery_details: {}, ", & self.error_recovery_details()
        );
        defmt::write!(f, "data_reset_details: {}, ", & self.data_reset_details());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PdStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PdStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PdStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PdStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PdStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PdStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PdStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ActiveRdoContract {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 16],
}
unsafe impl ::device_driver::Fieldset for ActiveRdoContract {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 16] };
}
impl ActiveRdoContract {
    /// `31:0` - Read the `active_rdo` field.
    ///
    /// Active RDO
    #[doc(alias = "ActiveRdo")]
    #[must_use]
    pub fn active_rdo(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `63:32` - Read the `source_epr_mode_do` field.
    ///
    /// Source EPR mode data object
    #[doc(alias = "SourceEprModeDo")]
    #[must_use]
    pub fn source_epr_mode_do(&self) -> u32 {
        let start = 32;
        let end = 63;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `95:64` - Read the `sink_epr_mode_do` field.
    ///
    /// Sink EPR mode data object
    #[doc(alias = "SinkEprModeDo")]
    #[must_use]
    pub fn sink_epr_mode_do(&self) -> u32 {
        let start = 64;
        let end = 95;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `127:96` - Read the `accepted_active_rdo` field.
    ///
    /// Accepted active RDO contract
    #[doc(alias = "AcceptedActiveRdo")]
    #[must_use]
    pub fn accepted_active_rdo(&self) -> u32 {
        let start = 96;
        let end = 127;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:0` - Set the `active_rdo` field.
    ///
    /// Active RDO
    #[doc(alias = "ActiveRdo")]
    pub fn set_active_rdo(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `63:32` - Set the `source_epr_mode_do` field.
    ///
    /// Source EPR mode data object
    #[doc(alias = "SourceEprModeDo")]
    pub fn set_source_epr_mode_do(&mut self, value: u32) {
        let start = 32;
        let end = 63;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `95:64` - Set the `sink_epr_mode_do` field.
    ///
    /// Sink EPR mode data object
    #[doc(alias = "SinkEprModeDo")]
    pub fn set_sink_epr_mode_do(&mut self, value: u32) {
        let start = 64;
        let end = 95;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `127:96` - Set the `accepted_active_rdo` field.
    ///
    /// Accepted active RDO contract
    #[doc(alias = "AcceptedActiveRdo")]
    pub fn set_accepted_active_rdo(&mut self, value: u32) {
        let start = 96;
        let end = 127;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ActiveRdoContract {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 16]> for ActiveRdoContract {
    fn from(bits: [u8; 16]) -> Self {
        Self { bits }
    }
}
impl From<ActiveRdoContract> for [u8; 16] {
    fn from(val: ActiveRdoContract) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ActiveRdoContract {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ActiveRdoContract");
        d.field("active_rdo", &self.active_rdo());
        d.field("source_epr_mode_do", &self.source_epr_mode_do());
        d.field("sink_epr_mode_do", &self.sink_epr_mode_do());
        d.field("accepted_active_rdo", &self.accepted_active_rdo());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActiveRdoContract {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ActiveRdoContract {{ ");
        defmt::write!(f, "active_rdo: {=u32}, ", & self.active_rdo());
        defmt::write!(f, "source_epr_mode_do: {=u32}, ", & self.source_epr_mode_do());
        defmt::write!(f, "sink_epr_mode_do: {=u32}, ", & self.sink_epr_mode_do());
        defmt::write!(f, "accepted_active_rdo: {=u32}, ", & self.accepted_active_rdo());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ActiveRdoContract {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ActiveRdoContract {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ActiveRdoContract {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ActiveRdoContract {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ActiveRdoContract {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ActiveRdoContract {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ActiveRdoContract {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ActivePdoContract {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 6],
}
unsafe impl ::device_driver::Fieldset for ActivePdoContract {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 6] };
}
impl ActivePdoContract {
    /// `31:0` - Read the `active_pdo` field.
    ///
    /// Active PDO
    #[doc(alias = "ActivePdo")]
    #[must_use]
    pub fn active_pdo(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `41:32` - Read the `first_pdo_control` field.
    ///
    /// Bits 20-29 of the first PDO
    #[doc(alias = "FirstPdoControl")]
    #[must_use]
    pub fn first_pdo_control(&self) -> u16 {
        let start = 32;
        let end = 41;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:0` - Set the `active_pdo` field.
    ///
    /// Active PDO
    #[doc(alias = "ActivePdo")]
    pub fn set_active_pdo(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `41:32` - Set the `first_pdo_control` field.
    ///
    /// Bits 20-29 of the first PDO
    #[doc(alias = "FirstPdoControl")]
    pub fn set_first_pdo_control(&mut self, value: u16) {
        let start = 32;
        let end = 41;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ActivePdoContract {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 6]> for ActivePdoContract {
    fn from(bits: [u8; 6]) -> Self {
        Self { bits }
    }
}
impl From<ActivePdoContract> for [u8; 6] {
    fn from(val: ActivePdoContract) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ActivePdoContract {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ActivePdoContract");
        d.field("active_pdo", &self.active_pdo());
        d.field("first_pdo_control", &self.first_pdo_control());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActivePdoContract {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ActivePdoContract {{ ");
        defmt::write!(f, "active_pdo: {=u32}, ", & self.active_pdo());
        defmt::write!(f, "first_pdo_control: {=u16}, ", & self.first_pdo_control());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ActivePdoContract {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ActivePdoContract {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ActivePdoContract {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ActivePdoContract {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ActivePdoContract {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ActivePdoContract {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ActivePdoContract {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PortControl {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for PortControl {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl PortControl {
    /// `1:0` - Read the `typec_current` field.
    ///
    /// Type-C current limit
    #[doc(alias = "TypecCurrent")]
    #[must_use]
    pub fn typec_current(&self) -> TypecCurrent {
        let start = 0;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 4` - Read the `process_swap_to_sink` field.
    ///
    /// Process swap to sink
    #[doc(alias = "ProcessSwapToSink")]
    #[must_use]
    pub fn process_swap_to_sink(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 5` - Read the `initiate_swap_to_sink` field.
    ///
    /// Initiate swap to sink
    #[doc(alias = "InitiateSwapToSink")]
    #[must_use]
    pub fn initiate_swap_to_sink(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `process_swap_to_source` field.
    ///
    /// Process swap to source
    #[doc(alias = "ProcessSwapToSource")]
    #[must_use]
    pub fn process_swap_to_source(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 7` - Read the `initiate_swap_to_source` field.
    ///
    /// Initiate swap to source
    #[doc(alias = "InitiateSwapToSource")]
    #[must_use]
    pub fn initiate_swap_to_source(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 8` - Read the `auto_alert_enable` field.
    ///
    /// Automatically initiate alert messaging
    #[doc(alias = "AutoAlertEnable")]
    #[must_use]
    pub fn auto_alert_enable(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 10` - Read the `auto_pps_status_enable` field.
    ///
    /// Automatically return PPS_Status
    #[doc(alias = "AutoPpsStatusEnable")]
    #[must_use]
    pub fn auto_pps_status_enable(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 11` - Read the `retimer_fw_update` field.
    ///
    /// Enable retimer firmware update
    #[doc(alias = "RetimerFwUpdate")]
    #[must_use]
    pub fn retimer_fw_update(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 12` - Read the `process_swap_to_ufp` field.
    ///
    /// Process swap to UFP
    #[doc(alias = "ProcessSwapToUfp")]
    #[must_use]
    pub fn process_swap_to_ufp(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 13` - Read the `initiate_swap_to_ufp` field.
    ///
    /// Initiate swap to UFP
    #[doc(alias = "InitiateSwapToUfp")]
    #[must_use]
    pub fn initiate_swap_to_ufp(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 14` - Read the `process_swap_to_dfp` field.
    ///
    /// Process swap to DFP
    #[doc(alias = "ProcessSwapToDfp")]
    #[must_use]
    pub fn process_swap_to_dfp(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 15` - Read the `initiate_swap_to_dfp` field.
    ///
    /// Initiate swap to DFP
    #[doc(alias = "InitiateSwapToDfp")]
    #[must_use]
    pub fn initiate_swap_to_dfp(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 16` - Read the `automatic_id_request` field.
    ///
    /// Automatically issue discover identity VDMs to appropriate SOPs
    #[doc(alias = "AutomaticIdRequest")]
    #[must_use]
    pub fn automatic_id_request(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 17` - Read the `am_intrusive_mode` field.
    ///
    /// Allow host to manage alt mode process
    #[doc(alias = "AmIntrusiveMode")]
    #[must_use]
    pub fn am_intrusive_mode(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 18` - Read the `force_usb_3_gen_1` field.
    ///
    /// Force USB3 Gen1 mode
    #[doc(alias = "ForceUsb3Gen1")]
    #[must_use]
    pub fn force_usb_3_gen_1(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 19` - Read the `unconstrained_power` field.
    ///
    /// External power present
    #[doc(alias = "UnconstrainedPower")]
    #[must_use]
    pub fn unconstrained_power(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 20` - Read the `enable_current_monitor` field.
    ///
    /// Enable current monitor using onboard ADC
    #[doc(alias = "EnableCurrentMonitor")]
    #[must_use]
    pub fn enable_current_monitor(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 21` - Read the `sink_control` field.
    ///
    /// Disable PP3/4 switches automatically
    #[doc(alias = "SinkControl")]
    #[must_use]
    pub fn sink_control(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 22` - Read the `fr_swap_enabled` field.
    ///
    /// Enable fast role swap
    #[doc(alias = "FrSwapEnabled")]
    #[must_use]
    pub fn fr_swap_enabled(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 29` - Read the `usb_disable` field.
    ///
    /// Disable USB data
    #[doc(alias = "UsbDisable")]
    #[must_use]
    pub fn usb_disable(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `44:43` - Read the `vconn_current_limit` field.
    ///
    /// Vconn current limit
    #[doc(alias = "VconnCurrentLimit")]
    #[must_use]
    pub fn vconn_current_limit(&self) -> VconnCurrentLimit {
        let start = 43;
        let end = 44;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `46:45` - Read the `active_dbg_channel` field.
    ///
    /// SBU Channel Control
    #[doc(alias = "ActiveDbgChannel")]
    #[must_use]
    pub fn active_dbg_channel(&self) -> ActiveDbgChannel {
        let start = 45;
        let end = 46;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `1:0` - Set the `typec_current` field.
    ///
    /// Type-C current limit
    #[doc(alias = "TypecCurrent")]
    pub fn set_typec_current(&mut self, value: TypecCurrent) {
        let start = 0;
        let end = 1;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `process_swap_to_sink` field.
    ///
    /// Process swap to sink
    #[doc(alias = "ProcessSwapToSink")]
    pub fn set_process_swap_to_sink(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `initiate_swap_to_sink` field.
    ///
    /// Initiate swap to sink
    #[doc(alias = "InitiateSwapToSink")]
    pub fn set_initiate_swap_to_sink(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `process_swap_to_source` field.
    ///
    /// Process swap to source
    #[doc(alias = "ProcessSwapToSource")]
    pub fn set_process_swap_to_source(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `initiate_swap_to_source` field.
    ///
    /// Initiate swap to source
    #[doc(alias = "InitiateSwapToSource")]
    pub fn set_initiate_swap_to_source(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 8` - Set the `auto_alert_enable` field.
    ///
    /// Automatically initiate alert messaging
    #[doc(alias = "AutoAlertEnable")]
    pub fn set_auto_alert_enable(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 10` - Set the `auto_pps_status_enable` field.
    ///
    /// Automatically return PPS_Status
    #[doc(alias = "AutoPpsStatusEnable")]
    pub fn set_auto_pps_status_enable(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 11` - Set the `retimer_fw_update` field.
    ///
    /// Enable retimer firmware update
    #[doc(alias = "RetimerFwUpdate")]
    pub fn set_retimer_fw_update(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 12` - Set the `process_swap_to_ufp` field.
    ///
    /// Process swap to UFP
    #[doc(alias = "ProcessSwapToUfp")]
    pub fn set_process_swap_to_ufp(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 13` - Set the `initiate_swap_to_ufp` field.
    ///
    /// Initiate swap to UFP
    #[doc(alias = "InitiateSwapToUfp")]
    pub fn set_initiate_swap_to_ufp(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 14` - Set the `process_swap_to_dfp` field.
    ///
    /// Process swap to DFP
    #[doc(alias = "ProcessSwapToDfp")]
    pub fn set_process_swap_to_dfp(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 15` - Set the `initiate_swap_to_dfp` field.
    ///
    /// Initiate swap to DFP
    #[doc(alias = "InitiateSwapToDfp")]
    pub fn set_initiate_swap_to_dfp(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 16` - Set the `automatic_id_request` field.
    ///
    /// Automatically issue discover identity VDMs to appropriate SOPs
    #[doc(alias = "AutomaticIdRequest")]
    pub fn set_automatic_id_request(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 17` - Set the `am_intrusive_mode` field.
    ///
    /// Allow host to manage alt mode process
    #[doc(alias = "AmIntrusiveMode")]
    pub fn set_am_intrusive_mode(&mut self, value: bool) {
        let start = 17;
        let end = 17;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 18` - Set the `force_usb_3_gen_1` field.
    ///
    /// Force USB3 Gen1 mode
    #[doc(alias = "ForceUsb3Gen1")]
    pub fn set_force_usb_3_gen_1(&mut self, value: bool) {
        let start = 18;
        let end = 18;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 19` - Set the `unconstrained_power` field.
    ///
    /// External power present
    #[doc(alias = "UnconstrainedPower")]
    pub fn set_unconstrained_power(&mut self, value: bool) {
        let start = 19;
        let end = 19;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 20` - Set the `enable_current_monitor` field.
    ///
    /// Enable current monitor using onboard ADC
    #[doc(alias = "EnableCurrentMonitor")]
    pub fn set_enable_current_monitor(&mut self, value: bool) {
        let start = 20;
        let end = 20;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 21` - Set the `sink_control` field.
    ///
    /// Disable PP3/4 switches automatically
    #[doc(alias = "SinkControl")]
    pub fn set_sink_control(&mut self, value: bool) {
        let start = 21;
        let end = 21;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 22` - Set the `fr_swap_enabled` field.
    ///
    /// Enable fast role swap
    #[doc(alias = "FrSwapEnabled")]
    pub fn set_fr_swap_enabled(&mut self, value: bool) {
        let start = 22;
        let end = 22;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 29` - Set the `usb_disable` field.
    ///
    /// Disable USB data
    #[doc(alias = "UsbDisable")]
    pub fn set_usb_disable(&mut self, value: bool) {
        let start = 29;
        let end = 29;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `44:43` - Set the `vconn_current_limit` field.
    ///
    /// Vconn current limit
    #[doc(alias = "VconnCurrentLimit")]
    pub fn set_vconn_current_limit(&mut self, value: VconnCurrentLimit) {
        let start = 43;
        let end = 44;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `46:45` - Set the `active_dbg_channel` field.
    ///
    /// SBU Channel Control
    #[doc(alias = "ActiveDbgChannel")]
    pub fn set_active_dbg_channel(&mut self, value: ActiveDbgChannel) {
        let start = 45;
        let end = 46;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for PortControl {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for PortControl {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<PortControl> for [u8; 8] {
    fn from(val: PortControl) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PortControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PortControl");
        d.field("typec_current", &self.typec_current());
        d.field("process_swap_to_sink", &self.process_swap_to_sink());
        d.field("initiate_swap_to_sink", &self.initiate_swap_to_sink());
        d.field("process_swap_to_source", &self.process_swap_to_source());
        d.field("initiate_swap_to_source", &self.initiate_swap_to_source());
        d.field("auto_alert_enable", &self.auto_alert_enable());
        d.field("auto_pps_status_enable", &self.auto_pps_status_enable());
        d.field("retimer_fw_update", &self.retimer_fw_update());
        d.field("process_swap_to_ufp", &self.process_swap_to_ufp());
        d.field("initiate_swap_to_ufp", &self.initiate_swap_to_ufp());
        d.field("process_swap_to_dfp", &self.process_swap_to_dfp());
        d.field("initiate_swap_to_dfp", &self.initiate_swap_to_dfp());
        d.field("automatic_id_request", &self.automatic_id_request());
        d.field("am_intrusive_mode", &self.am_intrusive_mode());
        d.field("force_usb_3_gen_1", &self.force_usb_3_gen_1());
        d.field("unconstrained_power", &self.unconstrained_power());
        d.field("enable_current_monitor", &self.enable_current_monitor());
        d.field("sink_control", &self.sink_control());
        d.field("fr_swap_enabled", &self.fr_swap_enabled());
        d.field("usb_disable", &self.usb_disable());
        d.field("vconn_current_limit", &self.vconn_current_limit());
        d.field("active_dbg_channel", &self.active_dbg_channel());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PortControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PortControl {{ ");
        defmt::write!(f, "typec_current: {}, ", & self.typec_current());
        defmt::write!(
            f, "process_swap_to_sink: {=bool}, ", & self.process_swap_to_sink()
        );
        defmt::write!(
            f, "initiate_swap_to_sink: {=bool}, ", & self.initiate_swap_to_sink()
        );
        defmt::write!(
            f, "process_swap_to_source: {=bool}, ", & self.process_swap_to_source()
        );
        defmt::write!(
            f, "initiate_swap_to_source: {=bool}, ", & self.initiate_swap_to_source()
        );
        defmt::write!(f, "auto_alert_enable: {=bool}, ", & self.auto_alert_enable());
        defmt::write!(
            f, "auto_pps_status_enable: {=bool}, ", & self.auto_pps_status_enable()
        );
        defmt::write!(f, "retimer_fw_update: {=bool}, ", & self.retimer_fw_update());
        defmt::write!(f, "process_swap_to_ufp: {=bool}, ", & self.process_swap_to_ufp());
        defmt::write!(
            f, "initiate_swap_to_ufp: {=bool}, ", & self.initiate_swap_to_ufp()
        );
        defmt::write!(f, "process_swap_to_dfp: {=bool}, ", & self.process_swap_to_dfp());
        defmt::write!(
            f, "initiate_swap_to_dfp: {=bool}, ", & self.initiate_swap_to_dfp()
        );
        defmt::write!(
            f, "automatic_id_request: {=bool}, ", & self.automatic_id_request()
        );
        defmt::write!(f, "am_intrusive_mode: {=bool}, ", & self.am_intrusive_mode());
        defmt::write!(f, "force_usb_3_gen_1: {=bool}, ", & self.force_usb_3_gen_1());
        defmt::write!(f, "unconstrained_power: {=bool}, ", & self.unconstrained_power());
        defmt::write!(
            f, "enable_current_monitor: {=bool}, ", & self.enable_current_monitor()
        );
        defmt::write!(f, "sink_control: {=bool}, ", & self.sink_control());
        defmt::write!(f, "fr_swap_enabled: {=bool}, ", & self.fr_swap_enabled());
        defmt::write!(f, "usb_disable: {=bool}, ", & self.usb_disable());
        defmt::write!(f, "vconn_current_limit: {}, ", & self.vconn_current_limit());
        defmt::write!(f, "active_dbg_channel: {}, ", & self.active_dbg_channel());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PortControl {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PortControl {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PortControl {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PortControl {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PortControl {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PortControl {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PortControl {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct SystemConfig {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 15],
}
unsafe impl ::device_driver::Fieldset for SystemConfig {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 15] };
}
impl SystemConfig {
    /// `bit 0` - Read the `pa_vconn_config` field.
    ///
    /// Enable PA VCONN
    #[doc(alias = "PaVconnConfig")]
    #[must_use]
    pub fn pa_vconn_config(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 2` - Read the `pb_vconn_config` field.
    ///
    /// Enable PB VCONN
    #[doc(alias = "PbVconnConfig")]
    #[must_use]
    pub fn pb_vconn_config(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `10:8` - Read the `pa_pp_5_v_vbus_sw_config` field.
    ///
    /// PA PP5V VBUS configuration
    #[doc(alias = "PaPp5vVbusSwConfig")]
    #[must_use]
    pub fn pa_pp_5_v_vbus_sw_config(&self) -> VbusSwConfig {
        let start = 8;
        let end = 10;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `13:11` - Read the `pb_pp_5_v_vbus_sw_config` field.
    ///
    /// PB PP5V VBUS configuration
    #[doc(alias = "PbPp5vVbusSwConfig")]
    #[must_use]
    pub fn pb_pp_5_v_vbus_sw_config(&self) -> VbusSwConfig {
        let start = 11;
        let end = 13;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `15:14` - Read the `ilim_over_shoot` field.
    ///
    /// PP_5V ILIM configuration
    #[doc(alias = "IlimOverShoot")]
    #[must_use]
    pub fn ilim_over_shoot(&self) -> IlimOverShoot {
        let start = 14;
        let end = 15;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `18:16` - Read the `pa_ppext_vbus_sw_config` field.
    ///
    /// PA PPEXT configuration
    #[doc(alias = "PaPpextVbusSwConfig")]
    #[must_use]
    pub fn pa_ppext_vbus_sw_config(&self) -> PpextVbusSwConfig {
        let start = 16;
        let end = 18;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `21:19` - Read the `pb_ppext_vbus_sw_config` field.
    ///
    /// PB PPEXT configuration
    #[doc(alias = "PbPpextVbusSwConfig")]
    #[must_use]
    pub fn pb_ppext_vbus_sw_config(&self) -> PpextVbusSwConfig {
        let start = 19;
        let end = 21;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `23:22` - Read the `rcp_threshold` field.
    ///
    /// Threshold used for RCP on PP_EXT
    #[doc(alias = "RcpThreshold")]
    #[must_use]
    pub fn rcp_threshold(&self) -> RcpThreshold {
        let start = 22;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 24` - Read the `multi_port_sink_policy_highest_power` field.
    ///
    /// Automatic sink-path coordination, true for highest power, false for no sink management
    #[doc(alias = "MultiPortSinkPolicyHighestPower")]
    #[must_use]
    pub fn multi_port_sink_policy_highest_power(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `28:26` - Read the `tbt_controller_type` field.
    ///
    /// Type of TBT controller
    #[doc(alias = "TbtControllerType")]
    #[must_use]
    pub fn tbt_controller_type(&self) -> TbtControllerType {
        let start = 26;
        let end = 28;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 29` - Read the `enable_one_ufp_policy` field.
    ///
    /// Enable bit for simple UFP policy manager
    #[doc(alias = "EnableOneUfpPolicy")]
    #[must_use]
    pub fn enable_one_ufp_policy(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 30` - Read the `enable_spm` field.
    ///
    /// Enable bit for simple source power management
    #[doc(alias = "EnableSpm")]
    #[must_use]
    pub fn enable_spm(&self) -> bool {
        let start = 30;
        let end = 30;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `32:31` - Read the `multi_port_sink_non_overlap_time` field.
    ///
    /// Delay configuration for MultiPortSinkPolicy
    #[doc(alias = "MultiPortSinkNonOverlapTime")]
    #[must_use]
    pub fn multi_port_sink_non_overlap_time(&self) -> MultiPortSinkNonOverlapTime {
        let start = 31;
        let end = 32;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 33` - Read the `enable_i_2_c_multi_controller_mode` field.
    ///
    /// Enables I2C Multi Controller mode
    #[doc(alias = "EnableI2cMultiControllerMode")]
    #[must_use]
    pub fn enable_i_2_c_multi_controller_mode(&self) -> bool {
        let start = 33;
        let end = 33;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `36:34` - Read the `i_2_c_timeout` field.
    ///
    /// I2C bus timeout
    #[doc(alias = "I2cTimeout")]
    #[must_use]
    pub fn i_2_c_timeout(&self) -> I2CTimeout {
        let start = 34;
        let end = 36;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 37` - Read the `disable_eeprom_updates` field.
    ///
    /// EEPROM updates not allowed if this bit asserted
    #[doc(alias = "DisableEepromUpdates")]
    #[must_use]
    pub fn disable_eeprom_updates(&self) -> bool {
        let start = 37;
        let end = 37;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 38` - Read the `emulate_single_port` field.
    ///
    /// Enable only port A
    #[doc(alias = "EmulateSinglePort")]
    #[must_use]
    pub fn emulate_single_port(&self) -> bool {
        let start = 38;
        let end = 38;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 39` - Read the `minimum_current_advertisement_1_a_5` field.
    ///
    /// SPM minimum current advertisement, true for 1.5 A, false for USB default
    #[doc(alias = "MinimumCurrentAdvertisement1A5")]
    #[must_use]
    pub fn minimum_current_advertisement_1_a_5(&self) -> bool {
        let start = 39;
        let end = 39;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `44:43` - Read the `usb_default_current` field.
    ///
    /// Value for USB default current
    #[doc(alias = "UsbDefaultCurrent")]
    #[must_use]
    pub fn usb_default_current(&self) -> UsbDefaultCurrent {
        let start = 43;
        let end = 44;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 45` - Read the `epr_supported_as_source` field.
    ///
    /// EPR supported as source
    #[doc(alias = "EprSupportedAsSource")]
    #[must_use]
    pub fn epr_supported_as_source(&self) -> bool {
        let start = 45;
        let end = 45;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 46` - Read the `epr_supported_as_sink` field.
    ///
    /// EPR supported as sink
    #[doc(alias = "EprSupportedAsSink")]
    #[must_use]
    pub fn epr_supported_as_sink(&self) -> bool {
        let start = 46;
        let end = 46;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 47` - Read the `enable_low_power_mode_am_entry_exit` field.
    ///
    /// Enable AM entry/exit on low-power mode exit/entry
    #[doc(alias = "EnableLowPowerModeAmEntryExit")]
    #[must_use]
    pub fn enable_low_power_mode_am_entry_exit(&self) -> bool {
        let start = 47;
        let end = 47;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 54` - Read the `crossbar_polling_mode` field.
    ///
    /// Enable crossbar polling mode
    #[doc(alias = "CrossbarPollingMode")]
    #[must_use]
    pub fn crossbar_polling_mode(&self) -> bool {
        let start = 54;
        let end = 54;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 55` - Read the `crossbar_config_type_1_extended` field.
    ///
    /// Enable crossbar type 1 extended write
    #[doc(alias = "CrossbarConfigType1Extended")]
    #[must_use]
    pub fn crossbar_config_type_1_extended(&self) -> bool {
        let start = 55;
        let end = 55;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `63:56` - Read the `external_dcdc_status_polling_interval` field.
    ///
    /// External DCDC Status Polling Interval
    #[doc(alias = "ExternalDcdcStatusPollingInterval")]
    #[must_use]
    pub fn external_dcdc_status_polling_interval(&self) -> u8 {
        let start = 56;
        let end = 63;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `71:64` - Read the `port_1_i_2_c_2_target_address` field.
    ///
    /// Target address for Port 1 on I2C2s
    #[doc(alias = "Port1I2c2TargetAddress")]
    #[must_use]
    pub fn port_1_i_2_c_2_target_address(&self) -> u8 {
        let start = 64;
        let end = 71;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `79:72` - Read the `port_2_i_2_c_2_target_address` field.
    ///
    /// Target address for Port 2 on I2C2s
    #[doc(alias = "Port2I2c2TargetAddress")]
    #[must_use]
    pub fn port_2_i_2_c_2_target_address(&self) -> u8 {
        let start = 72;
        let end = 79;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 80` - Read the `vsys_prevents_high_power` field.
    ///
    /// Halts setting up external DCDC configuration until 5V power is present from the system
    #[doc(alias = "VsysPreventsHighPower")]
    #[must_use]
    pub fn vsys_prevents_high_power(&self) -> bool {
        let start = 80;
        let end = 80;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 81` - Read the `wait_for_vin_3_v_3` field.
    ///
    /// Stalls the PD in PTCH mode until Vsys is present
    #[doc(alias = "WaitForVin3v3")]
    #[must_use]
    pub fn wait_for_vin_3_v_3(&self) -> bool {
        let start = 81;
        let end = 81;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 82` - Read the `wait_for_minimum_power` field.
    ///
    /// Stalls the PD in PTCH mode until a power connection is made that meets the needed conditions
    #[doc(alias = "WaitForMinimumPower")]
    #[must_use]
    pub fn wait_for_minimum_power(&self) -> bool {
        let start = 82;
        let end = 82;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 86` - Read the `auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3` field.
    ///
    /// On detecting VIN_3V3, auto clear the dead battery flag and reset the connection on the source port to update PD contract negotiated when the battery was dead
    #[doc(alias = "AutoClrDeadBatteryFlagAndResetOnVin3v3")]
    #[must_use]
    pub fn auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3(&self) -> bool {
        let start = 86;
        let end = 86;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `104:103` - Read the `source_policy_mode` field.
    ///
    /// Source Policy Mode
    #[doc(alias = "SourcePolicyMode")]
    #[must_use]
    pub fn source_policy_mode(&self) -> u8 {
        let start = 103;
        let end = 104;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 0` - Set the `pa_vconn_config` field.
    ///
    /// Enable PA VCONN
    #[doc(alias = "PaVconnConfig")]
    pub fn set_pa_vconn_config(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 2` - Set the `pb_vconn_config` field.
    ///
    /// Enable PB VCONN
    #[doc(alias = "PbVconnConfig")]
    pub fn set_pb_vconn_config(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `10:8` - Set the `pa_pp_5_v_vbus_sw_config` field.
    ///
    /// PA PP5V VBUS configuration
    #[doc(alias = "PaPp5vVbusSwConfig")]
    pub fn set_pa_pp_5_v_vbus_sw_config(&mut self, value: VbusSwConfig) {
        let start = 8;
        let end = 10;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `13:11` - Set the `pb_pp_5_v_vbus_sw_config` field.
    ///
    /// PB PP5V VBUS configuration
    #[doc(alias = "PbPp5vVbusSwConfig")]
    pub fn set_pb_pp_5_v_vbus_sw_config(&mut self, value: VbusSwConfig) {
        let start = 11;
        let end = 13;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `15:14` - Set the `ilim_over_shoot` field.
    ///
    /// PP_5V ILIM configuration
    #[doc(alias = "IlimOverShoot")]
    pub fn set_ilim_over_shoot(&mut self, value: IlimOverShoot) {
        let start = 14;
        let end = 15;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `18:16` - Set the `pa_ppext_vbus_sw_config` field.
    ///
    /// PA PPEXT configuration
    #[doc(alias = "PaPpextVbusSwConfig")]
    pub fn set_pa_ppext_vbus_sw_config(&mut self, value: PpextVbusSwConfig) {
        let start = 16;
        let end = 18;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `21:19` - Set the `pb_ppext_vbus_sw_config` field.
    ///
    /// PB PPEXT configuration
    #[doc(alias = "PbPpextVbusSwConfig")]
    pub fn set_pb_ppext_vbus_sw_config(&mut self, value: PpextVbusSwConfig) {
        let start = 19;
        let end = 21;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `23:22` - Set the `rcp_threshold` field.
    ///
    /// Threshold used for RCP on PP_EXT
    #[doc(alias = "RcpThreshold")]
    pub fn set_rcp_threshold(&mut self, value: RcpThreshold) {
        let start = 22;
        let end = 23;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 24` - Set the `multi_port_sink_policy_highest_power` field.
    ///
    /// Automatic sink-path coordination, true for highest power, false for no sink management
    #[doc(alias = "MultiPortSinkPolicyHighestPower")]
    pub fn set_multi_port_sink_policy_highest_power(&mut self, value: bool) {
        let start = 24;
        let end = 24;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `28:26` - Set the `tbt_controller_type` field.
    ///
    /// Type of TBT controller
    #[doc(alias = "TbtControllerType")]
    pub fn set_tbt_controller_type(&mut self, value: TbtControllerType) {
        let start = 26;
        let end = 28;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 29` - Set the `enable_one_ufp_policy` field.
    ///
    /// Enable bit for simple UFP policy manager
    #[doc(alias = "EnableOneUfpPolicy")]
    pub fn set_enable_one_ufp_policy(&mut self, value: bool) {
        let start = 29;
        let end = 29;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 30` - Set the `enable_spm` field.
    ///
    /// Enable bit for simple source power management
    #[doc(alias = "EnableSpm")]
    pub fn set_enable_spm(&mut self, value: bool) {
        let start = 30;
        let end = 30;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `32:31` - Set the `multi_port_sink_non_overlap_time` field.
    ///
    /// Delay configuration for MultiPortSinkPolicy
    #[doc(alias = "MultiPortSinkNonOverlapTime")]
    pub fn set_multi_port_sink_non_overlap_time(
        &mut self,
        value: MultiPortSinkNonOverlapTime,
    ) {
        let start = 31;
        let end = 32;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 33` - Set the `enable_i_2_c_multi_controller_mode` field.
    ///
    /// Enables I2C Multi Controller mode
    #[doc(alias = "EnableI2cMultiControllerMode")]
    pub fn set_enable_i_2_c_multi_controller_mode(&mut self, value: bool) {
        let start = 33;
        let end = 33;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `36:34` - Set the `i_2_c_timeout` field.
    ///
    /// I2C bus timeout
    #[doc(alias = "I2cTimeout")]
    pub fn set_i_2_c_timeout(&mut self, value: I2CTimeout) {
        let start = 34;
        let end = 36;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 37` - Set the `disable_eeprom_updates` field.
    ///
    /// EEPROM updates not allowed if this bit asserted
    #[doc(alias = "DisableEepromUpdates")]
    pub fn set_disable_eeprom_updates(&mut self, value: bool) {
        let start = 37;
        let end = 37;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 38` - Set the `emulate_single_port` field.
    ///
    /// Enable only port A
    #[doc(alias = "EmulateSinglePort")]
    pub fn set_emulate_single_port(&mut self, value: bool) {
        let start = 38;
        let end = 38;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 39` - Set the `minimum_current_advertisement_1_a_5` field.
    ///
    /// SPM minimum current advertisement, true for 1.5 A, false for USB default
    #[doc(alias = "MinimumCurrentAdvertisement1A5")]
    pub fn set_minimum_current_advertisement_1_a_5(&mut self, value: bool) {
        let start = 39;
        let end = 39;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `44:43` - Set the `usb_default_current` field.
    ///
    /// Value for USB default current
    #[doc(alias = "UsbDefaultCurrent")]
    pub fn set_usb_default_current(&mut self, value: UsbDefaultCurrent) {
        let start = 43;
        let end = 44;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 45` - Set the `epr_supported_as_source` field.
    ///
    /// EPR supported as source
    #[doc(alias = "EprSupportedAsSource")]
    pub fn set_epr_supported_as_source(&mut self, value: bool) {
        let start = 45;
        let end = 45;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 46` - Set the `epr_supported_as_sink` field.
    ///
    /// EPR supported as sink
    #[doc(alias = "EprSupportedAsSink")]
    pub fn set_epr_supported_as_sink(&mut self, value: bool) {
        let start = 46;
        let end = 46;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 47` - Set the `enable_low_power_mode_am_entry_exit` field.
    ///
    /// Enable AM entry/exit on low-power mode exit/entry
    #[doc(alias = "EnableLowPowerModeAmEntryExit")]
    pub fn set_enable_low_power_mode_am_entry_exit(&mut self, value: bool) {
        let start = 47;
        let end = 47;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 54` - Set the `crossbar_polling_mode` field.
    ///
    /// Enable crossbar polling mode
    #[doc(alias = "CrossbarPollingMode")]
    pub fn set_crossbar_polling_mode(&mut self, value: bool) {
        let start = 54;
        let end = 54;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 55` - Set the `crossbar_config_type_1_extended` field.
    ///
    /// Enable crossbar type 1 extended write
    #[doc(alias = "CrossbarConfigType1Extended")]
    pub fn set_crossbar_config_type_1_extended(&mut self, value: bool) {
        let start = 55;
        let end = 55;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `63:56` - Set the `external_dcdc_status_polling_interval` field.
    ///
    /// External DCDC Status Polling Interval
    #[doc(alias = "ExternalDcdcStatusPollingInterval")]
    pub fn set_external_dcdc_status_polling_interval(&mut self, value: u8) {
        let start = 56;
        let end = 63;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `71:64` - Set the `port_1_i_2_c_2_target_address` field.
    ///
    /// Target address for Port 1 on I2C2s
    #[doc(alias = "Port1I2c2TargetAddress")]
    pub fn set_port_1_i_2_c_2_target_address(&mut self, value: u8) {
        let start = 64;
        let end = 71;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `79:72` - Set the `port_2_i_2_c_2_target_address` field.
    ///
    /// Target address for Port 2 on I2C2s
    #[doc(alias = "Port2I2c2TargetAddress")]
    pub fn set_port_2_i_2_c_2_target_address(&mut self, value: u8) {
        let start = 72;
        let end = 79;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 80` - Set the `vsys_prevents_high_power` field.
    ///
    /// Halts setting up external DCDC configuration until 5V power is present from the system
    #[doc(alias = "VsysPreventsHighPower")]
    pub fn set_vsys_prevents_high_power(&mut self, value: bool) {
        let start = 80;
        let end = 80;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 81` - Set the `wait_for_vin_3_v_3` field.
    ///
    /// Stalls the PD in PTCH mode until Vsys is present
    #[doc(alias = "WaitForVin3v3")]
    pub fn set_wait_for_vin_3_v_3(&mut self, value: bool) {
        let start = 81;
        let end = 81;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 82` - Set the `wait_for_minimum_power` field.
    ///
    /// Stalls the PD in PTCH mode until a power connection is made that meets the needed conditions
    #[doc(alias = "WaitForMinimumPower")]
    pub fn set_wait_for_minimum_power(&mut self, value: bool) {
        let start = 82;
        let end = 82;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 86` - Set the `auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3` field.
    ///
    /// On detecting VIN_3V3, auto clear the dead battery flag and reset the connection on the source port to update PD contract negotiated when the battery was dead
    #[doc(alias = "AutoClrDeadBatteryFlagAndResetOnVin3v3")]
    pub fn set_auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3(
        &mut self,
        value: bool,
    ) {
        let start = 86;
        let end = 86;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `104:103` - Set the `source_policy_mode` field.
    ///
    /// Source Policy Mode
    #[doc(alias = "SourcePolicyMode")]
    pub fn set_source_policy_mode(&mut self, value: u8) {
        let start = 103;
        let end = 104;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for SystemConfig {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 15]> for SystemConfig {
    fn from(bits: [u8; 15]) -> Self {
        Self { bits }
    }
}
impl From<SystemConfig> for [u8; 15] {
    fn from(val: SystemConfig) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for SystemConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("SystemConfig");
        d.field("pa_vconn_config", &self.pa_vconn_config());
        d.field("pb_vconn_config", &self.pb_vconn_config());
        d.field("pa_pp_5_v_vbus_sw_config", &self.pa_pp_5_v_vbus_sw_config());
        d.field("pb_pp_5_v_vbus_sw_config", &self.pb_pp_5_v_vbus_sw_config());
        d.field("ilim_over_shoot", &self.ilim_over_shoot());
        d.field("pa_ppext_vbus_sw_config", &self.pa_ppext_vbus_sw_config());
        d.field("pb_ppext_vbus_sw_config", &self.pb_ppext_vbus_sw_config());
        d.field("rcp_threshold", &self.rcp_threshold());
        d.field(
            "multi_port_sink_policy_highest_power",
            &self.multi_port_sink_policy_highest_power(),
        );
        d.field("tbt_controller_type", &self.tbt_controller_type());
        d.field("enable_one_ufp_policy", &self.enable_one_ufp_policy());
        d.field("enable_spm", &self.enable_spm());
        d.field(
            "multi_port_sink_non_overlap_time",
            &self.multi_port_sink_non_overlap_time(),
        );
        d.field(
            "enable_i_2_c_multi_controller_mode",
            &self.enable_i_2_c_multi_controller_mode(),
        );
        d.field("i_2_c_timeout", &self.i_2_c_timeout());
        d.field("disable_eeprom_updates", &self.disable_eeprom_updates());
        d.field("emulate_single_port", &self.emulate_single_port());
        d.field(
            "minimum_current_advertisement_1_a_5",
            &self.minimum_current_advertisement_1_a_5(),
        );
        d.field("usb_default_current", &self.usb_default_current());
        d.field("epr_supported_as_source", &self.epr_supported_as_source());
        d.field("epr_supported_as_sink", &self.epr_supported_as_sink());
        d.field(
            "enable_low_power_mode_am_entry_exit",
            &self.enable_low_power_mode_am_entry_exit(),
        );
        d.field("crossbar_polling_mode", &self.crossbar_polling_mode());
        d.field(
            "crossbar_config_type_1_extended",
            &self.crossbar_config_type_1_extended(),
        );
        d.field(
            "external_dcdc_status_polling_interval",
            &self.external_dcdc_status_polling_interval(),
        );
        d.field("port_1_i_2_c_2_target_address", &self.port_1_i_2_c_2_target_address());
        d.field("port_2_i_2_c_2_target_address", &self.port_2_i_2_c_2_target_address());
        d.field("vsys_prevents_high_power", &self.vsys_prevents_high_power());
        d.field("wait_for_vin_3_v_3", &self.wait_for_vin_3_v_3());
        d.field("wait_for_minimum_power", &self.wait_for_minimum_power());
        d.field(
            "auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3",
            &self.auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3(),
        );
        d.field("source_policy_mode", &self.source_policy_mode());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SystemConfig {{ ");
        defmt::write!(f, "pa_vconn_config: {=bool}, ", & self.pa_vconn_config());
        defmt::write!(f, "pb_vconn_config: {=bool}, ", & self.pb_vconn_config());
        defmt::write!(
            f, "pa_pp_5_v_vbus_sw_config: {}, ", & self.pa_pp_5_v_vbus_sw_config()
        );
        defmt::write!(
            f, "pb_pp_5_v_vbus_sw_config: {}, ", & self.pb_pp_5_v_vbus_sw_config()
        );
        defmt::write!(f, "ilim_over_shoot: {}, ", & self.ilim_over_shoot());
        defmt::write!(
            f, "pa_ppext_vbus_sw_config: {}, ", & self.pa_ppext_vbus_sw_config()
        );
        defmt::write!(
            f, "pb_ppext_vbus_sw_config: {}, ", & self.pb_ppext_vbus_sw_config()
        );
        defmt::write!(f, "rcp_threshold: {}, ", & self.rcp_threshold());
        defmt::write!(
            f, "multi_port_sink_policy_highest_power: {=bool}, ", & self
            .multi_port_sink_policy_highest_power()
        );
        defmt::write!(f, "tbt_controller_type: {}, ", & self.tbt_controller_type());
        defmt::write!(
            f, "enable_one_ufp_policy: {=bool}, ", & self.enable_one_ufp_policy()
        );
        defmt::write!(f, "enable_spm: {=bool}, ", & self.enable_spm());
        defmt::write!(
            f, "multi_port_sink_non_overlap_time: {}, ", & self
            .multi_port_sink_non_overlap_time()
        );
        defmt::write!(
            f, "enable_i_2_c_multi_controller_mode: {=bool}, ", & self
            .enable_i_2_c_multi_controller_mode()
        );
        defmt::write!(f, "i_2_c_timeout: {}, ", & self.i_2_c_timeout());
        defmt::write!(
            f, "disable_eeprom_updates: {=bool}, ", & self.disable_eeprom_updates()
        );
        defmt::write!(f, "emulate_single_port: {=bool}, ", & self.emulate_single_port());
        defmt::write!(
            f, "minimum_current_advertisement_1_a_5: {=bool}, ", & self
            .minimum_current_advertisement_1_a_5()
        );
        defmt::write!(f, "usb_default_current: {}, ", & self.usb_default_current());
        defmt::write!(
            f, "epr_supported_as_source: {=bool}, ", & self.epr_supported_as_source()
        );
        defmt::write!(
            f, "epr_supported_as_sink: {=bool}, ", & self.epr_supported_as_sink()
        );
        defmt::write!(
            f, "enable_low_power_mode_am_entry_exit: {=bool}, ", & self
            .enable_low_power_mode_am_entry_exit()
        );
        defmt::write!(
            f, "crossbar_polling_mode: {=bool}, ", & self.crossbar_polling_mode()
        );
        defmt::write!(
            f, "crossbar_config_type_1_extended: {=bool}, ", & self
            .crossbar_config_type_1_extended()
        );
        defmt::write!(
            f, "external_dcdc_status_polling_interval: {=u8}, ", & self
            .external_dcdc_status_polling_interval()
        );
        defmt::write!(
            f, "port_1_i_2_c_2_target_address: {=u8}, ", & self
            .port_1_i_2_c_2_target_address()
        );
        defmt::write!(
            f, "port_2_i_2_c_2_target_address: {=u8}, ", & self
            .port_2_i_2_c_2_target_address()
        );
        defmt::write!(
            f, "vsys_prevents_high_power: {=bool}, ", & self.vsys_prevents_high_power()
        );
        defmt::write!(f, "wait_for_vin_3_v_3: {=bool}, ", & self.wait_for_vin_3_v_3());
        defmt::write!(
            f, "wait_for_minimum_power: {=bool}, ", & self.wait_for_minimum_power()
        );
        defmt::write!(
            f, "auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3: {=bool}, ", & self
            .auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3()
        );
        defmt::write!(f, "source_policy_mode: {=u8}, ", & self.source_policy_mode());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for SystemConfig {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for SystemConfig {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for SystemConfig {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for SystemConfig {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for SystemConfig {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for SystemConfig {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for SystemConfig {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PowerPathStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 5],
}
unsafe impl ::device_driver::Fieldset for PowerPathStatus {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 5] };
}
impl PowerPathStatus {
    /// `1:0` - Read the `pa_vconn_sw` field.
    ///
    /// PA Vconn switch status
    #[doc(alias = "PaVconnSw")]
    #[must_use]
    pub fn pa_vconn_sw(&self) -> PpVconnSw {
        let start = 0;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `3:2` - Read the `pb_vconn_sw` field.
    ///
    /// PA Vconn switch status
    #[doc(alias = "PbVconnSw")]
    #[must_use]
    pub fn pb_vconn_sw(&self) -> PpVconnSw {
        let start = 2;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `8:6` - Read the `pa_int_vbus_sw` field.
    ///
    /// PA int vbus switch status
    #[doc(alias = "PaIntVbusSw")]
    #[must_use]
    pub fn pa_int_vbus_sw(&self) -> PpIntVbusSw {
        let start = 6;
        let end = 8;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `11:9` - Read the `pb_int_vbus_sw` field.
    ///
    /// PB int vbus switch status
    #[doc(alias = "PbIntVbusSw")]
    #[must_use]
    pub fn pb_int_vbus_sw(&self) -> PpIntVbusSw {
        let start = 9;
        let end = 11;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `14:12` - Read the `pa_ext_vbus_sw` field.
    ///
    /// PA ext vbus switch status
    #[doc(alias = "PaExtVbusSw")]
    #[must_use]
    pub fn pa_ext_vbus_sw(&self) -> PpExtVbusSw {
        let start = 12;
        let end = 14;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `17:15` - Read the `pb_ext_vbus_sw` field.
    ///
    /// PB ext vbus switch status
    #[doc(alias = "PbExtVbusSw")]
    #[must_use]
    pub fn pb_ext_vbus_sw(&self) -> PpExtVbusSw {
        let start = 15;
        let end = 17;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 28` - Read the `pa_int_vbus_oc` field.
    ///
    /// PA int vbus overcurrent
    #[doc(alias = "PaIntVbusOc")]
    #[must_use]
    pub fn pa_int_vbus_oc(&self) -> bool {
        let start = 28;
        let end = 28;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 29` - Read the `pb_int_vbus_oc` field.
    ///
    /// PB int vbus overcurrent
    #[doc(alias = "PbIntVbusOc")]
    #[must_use]
    pub fn pb_int_vbus_oc(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 34` - Read the `pa_vconn_oc` field.
    ///
    /// PA vconn overcurrent
    #[doc(alias = "PaVconnOc")]
    #[must_use]
    pub fn pa_vconn_oc(&self) -> bool {
        let start = 34;
        let end = 34;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 35` - Read the `pb_vconn_oc` field.
    ///
    /// PB vconn overcurrent
    #[doc(alias = "PbVconnOc")]
    #[must_use]
    pub fn pb_vconn_oc(&self) -> bool {
        let start = 35;
        let end = 35;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `39:38` - Read the `power_source` field.
    ///
    /// How the PD controller is powered
    #[doc(alias = "PowerSource")]
    #[must_use]
    pub fn power_source(&self) -> PpPowerSource {
        let start = 38;
        let end = 39;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `1:0` - Set the `pa_vconn_sw` field.
    ///
    /// PA Vconn switch status
    #[doc(alias = "PaVconnSw")]
    pub fn set_pa_vconn_sw(&mut self, value: PpVconnSw) {
        let start = 0;
        let end = 1;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `3:2` - Set the `pb_vconn_sw` field.
    ///
    /// PA Vconn switch status
    #[doc(alias = "PbVconnSw")]
    pub fn set_pb_vconn_sw(&mut self, value: PpVconnSw) {
        let start = 2;
        let end = 3;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `8:6` - Set the `pa_int_vbus_sw` field.
    ///
    /// PA int vbus switch status
    #[doc(alias = "PaIntVbusSw")]
    pub fn set_pa_int_vbus_sw(&mut self, value: PpIntVbusSw) {
        let start = 6;
        let end = 8;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `11:9` - Set the `pb_int_vbus_sw` field.
    ///
    /// PB int vbus switch status
    #[doc(alias = "PbIntVbusSw")]
    pub fn set_pb_int_vbus_sw(&mut self, value: PpIntVbusSw) {
        let start = 9;
        let end = 11;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `14:12` - Set the `pa_ext_vbus_sw` field.
    ///
    /// PA ext vbus switch status
    #[doc(alias = "PaExtVbusSw")]
    pub fn set_pa_ext_vbus_sw(&mut self, value: PpExtVbusSw) {
        let start = 12;
        let end = 14;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `17:15` - Set the `pb_ext_vbus_sw` field.
    ///
    /// PB ext vbus switch status
    #[doc(alias = "PbExtVbusSw")]
    pub fn set_pb_ext_vbus_sw(&mut self, value: PpExtVbusSw) {
        let start = 15;
        let end = 17;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 28` - Set the `pa_int_vbus_oc` field.
    ///
    /// PA int vbus overcurrent
    #[doc(alias = "PaIntVbusOc")]
    pub fn set_pa_int_vbus_oc(&mut self, value: bool) {
        let start = 28;
        let end = 28;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 29` - Set the `pb_int_vbus_oc` field.
    ///
    /// PB int vbus overcurrent
    #[doc(alias = "PbIntVbusOc")]
    pub fn set_pb_int_vbus_oc(&mut self, value: bool) {
        let start = 29;
        let end = 29;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 34` - Set the `pa_vconn_oc` field.
    ///
    /// PA vconn overcurrent
    #[doc(alias = "PaVconnOc")]
    pub fn set_pa_vconn_oc(&mut self, value: bool) {
        let start = 34;
        let end = 34;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 35` - Set the `pb_vconn_oc` field.
    ///
    /// PB vconn overcurrent
    #[doc(alias = "PbVconnOc")]
    pub fn set_pb_vconn_oc(&mut self, value: bool) {
        let start = 35;
        let end = 35;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `39:38` - Set the `power_source` field.
    ///
    /// How the PD controller is powered
    #[doc(alias = "PowerSource")]
    pub fn set_power_source(&mut self, value: PpPowerSource) {
        let start = 38;
        let end = 39;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for PowerPathStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 5]> for PowerPathStatus {
    fn from(bits: [u8; 5]) -> Self {
        Self { bits }
    }
}
impl From<PowerPathStatus> for [u8; 5] {
    fn from(val: PowerPathStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PowerPathStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PowerPathStatus");
        d.field("pa_vconn_sw", &self.pa_vconn_sw());
        d.field("pb_vconn_sw", &self.pb_vconn_sw());
        d.field("pa_int_vbus_sw", &self.pa_int_vbus_sw());
        d.field("pb_int_vbus_sw", &self.pb_int_vbus_sw());
        d.field("pa_ext_vbus_sw", &self.pa_ext_vbus_sw());
        d.field("pb_ext_vbus_sw", &self.pb_ext_vbus_sw());
        d.field("pa_int_vbus_oc", &self.pa_int_vbus_oc());
        d.field("pb_int_vbus_oc", &self.pb_int_vbus_oc());
        d.field("pa_vconn_oc", &self.pa_vconn_oc());
        d.field("pb_vconn_oc", &self.pb_vconn_oc());
        d.field("power_source", &self.power_source());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PowerPathStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PowerPathStatus {{ ");
        defmt::write!(f, "pa_vconn_sw: {}, ", & self.pa_vconn_sw());
        defmt::write!(f, "pb_vconn_sw: {}, ", & self.pb_vconn_sw());
        defmt::write!(f, "pa_int_vbus_sw: {}, ", & self.pa_int_vbus_sw());
        defmt::write!(f, "pb_int_vbus_sw: {}, ", & self.pb_int_vbus_sw());
        defmt::write!(f, "pa_ext_vbus_sw: {}, ", & self.pa_ext_vbus_sw());
        defmt::write!(f, "pb_ext_vbus_sw: {}, ", & self.pb_ext_vbus_sw());
        defmt::write!(f, "pa_int_vbus_oc: {=bool}, ", & self.pa_int_vbus_oc());
        defmt::write!(f, "pb_int_vbus_oc: {=bool}, ", & self.pb_int_vbus_oc());
        defmt::write!(f, "pa_vconn_oc: {=bool}, ", & self.pa_vconn_oc());
        defmt::write!(f, "pb_vconn_oc: {=bool}, ", & self.pb_vconn_oc());
        defmt::write!(f, "power_source: {}, ", & self.power_source());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PowerPathStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PowerPathStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PowerPathStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PowerPathStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PowerPathStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PowerPathStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PowerPathStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct UsbStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 9],
}
unsafe impl ::device_driver::Fieldset for UsbStatus {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 9] };
}
impl UsbStatus {
    /// `1:0` - Read the `eudo_sop_sent_status` field.
    ///
    /// Enter USB4 mode status
    #[doc(alias = "EudoSopSentStatus")]
    #[must_use]
    pub fn eudo_sop_sent_status(&self) -> EudoSopSentStatus {
        let start = 0;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `3:2` - Read the `usb_4_required_plug_mode` field.
    ///
    /// USB4 plug mode requirement
    #[doc(alias = "Usb4RequiredPlugMode")]
    #[must_use]
    pub fn usb_4_required_plug_mode(&self) -> Usb4RequiredPlugMode {
        let start = 2;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 4` - Read the `usb_mode_active_on_plug` field.
    ///
    /// USB4 mode active on plug
    #[doc(alias = "UsbModeActiveOnPlug")]
    #[must_use]
    pub fn usb_mode_active_on_plug(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 5` - Read the `vpro_entry_failed` field.
    ///
    /// vPro mode error. This bit is asserted ifa n error occurred while trying to enter the vPro mode.
    #[doc(alias = "VproEntryFailed")]
    #[must_use]
    pub fn vpro_entry_failed(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `usb_reentry_needed` field.
    ///
    /// USB re-entry is needed
    #[doc(alias = "UsbReentryNeeded")]
    #[must_use]
    pub fn usb_reentry_needed(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `39:8` - Read the `enter_usb_data_object` field.
    ///
    /// Enter_USB Data Object (EUDO)
    #[doc(alias = "EnterUsbDataObject")]
    #[must_use]
    pub fn enter_usb_data_object(&self) -> u32 {
        let start = 8;
        let end = 39;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `71:40` - Read the `tbt_enter_mode_vdo` field.
    ///
    /// vPro mode VDO.
    #[doc(alias = "TbtEnterModeVdo")]
    #[must_use]
    pub fn tbt_enter_mode_vdo(&self) -> u32 {
        let start = 40;
        let end = 71;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `1:0` - Set the `eudo_sop_sent_status` field.
    ///
    /// Enter USB4 mode status
    #[doc(alias = "EudoSopSentStatus")]
    pub fn set_eudo_sop_sent_status(&mut self, value: EudoSopSentStatus) {
        let start = 0;
        let end = 1;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `3:2` - Set the `usb_4_required_plug_mode` field.
    ///
    /// USB4 plug mode requirement
    #[doc(alias = "Usb4RequiredPlugMode")]
    pub fn set_usb_4_required_plug_mode(&mut self, value: Usb4RequiredPlugMode) {
        let start = 2;
        let end = 3;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `usb_mode_active_on_plug` field.
    ///
    /// USB4 mode active on plug
    #[doc(alias = "UsbModeActiveOnPlug")]
    pub fn set_usb_mode_active_on_plug(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `vpro_entry_failed` field.
    ///
    /// vPro mode error. This bit is asserted ifa n error occurred while trying to enter the vPro mode.
    #[doc(alias = "VproEntryFailed")]
    pub fn set_vpro_entry_failed(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `usb_reentry_needed` field.
    ///
    /// USB re-entry is needed
    #[doc(alias = "UsbReentryNeeded")]
    pub fn set_usb_reentry_needed(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `39:8` - Set the `enter_usb_data_object` field.
    ///
    /// Enter_USB Data Object (EUDO)
    #[doc(alias = "EnterUsbDataObject")]
    pub fn set_enter_usb_data_object(&mut self, value: u32) {
        let start = 8;
        let end = 39;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `71:40` - Set the `tbt_enter_mode_vdo` field.
    ///
    /// vPro mode VDO.
    #[doc(alias = "TbtEnterModeVdo")]
    pub fn set_tbt_enter_mode_vdo(&mut self, value: u32) {
        let start = 40;
        let end = 71;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for UsbStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 9]> for UsbStatus {
    fn from(bits: [u8; 9]) -> Self {
        Self { bits }
    }
}
impl From<UsbStatus> for [u8; 9] {
    fn from(val: UsbStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for UsbStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("UsbStatus");
        d.field("eudo_sop_sent_status", &self.eudo_sop_sent_status());
        d.field("usb_4_required_plug_mode", &self.usb_4_required_plug_mode());
        d.field("usb_mode_active_on_plug", &self.usb_mode_active_on_plug());
        d.field("vpro_entry_failed", &self.vpro_entry_failed());
        d.field("usb_reentry_needed", &self.usb_reentry_needed());
        d.field("enter_usb_data_object", &self.enter_usb_data_object());
        d.field("tbt_enter_mode_vdo", &self.tbt_enter_mode_vdo());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbStatus {{ ");
        defmt::write!(f, "eudo_sop_sent_status: {}, ", & self.eudo_sop_sent_status());
        defmt::write!(
            f, "usb_4_required_plug_mode: {}, ", & self.usb_4_required_plug_mode()
        );
        defmt::write!(
            f, "usb_mode_active_on_plug: {=bool}, ", & self.usb_mode_active_on_plug()
        );
        defmt::write!(f, "vpro_entry_failed: {=bool}, ", & self.vpro_entry_failed());
        defmt::write!(f, "usb_reentry_needed: {=bool}, ", & self.usb_reentry_needed());
        defmt::write!(
            f, "enter_usb_data_object: {=u32}, ", & self.enter_usb_data_object()
        );
        defmt::write!(f, "tbt_enter_mode_vdo: {=u32}, ", & self.tbt_enter_mode_vdo());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for UsbStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for UsbStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for UsbStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for UsbStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for UsbStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for UsbStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for UsbStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Status {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 5],
}
unsafe impl ::device_driver::Fieldset for Status {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 5] };
}
impl Status {
    /// `bit 0` - Read the `plug_present` field.
    ///
    /// Plug present
    #[doc(alias = "PlugPresent")]
    #[must_use]
    pub fn plug_present(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `3:1` - Read the `connection_state` field.
    ///
    /// Connection state
    #[doc(alias = "ConnectionState")]
    #[must_use]
    pub fn connection_state(&self) -> PlugMode {
        let start = 1;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 4` - Read the `plug_orientation` field.
    ///
    /// Connector oreintation, 0 for normal, 1 for flipped
    #[doc(alias = "PlugOrientation")]
    #[must_use]
    pub fn plug_orientation(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 5` - Read the `port_role` field.
    ///
    /// PD role, 0 for sink, 1 for source
    #[doc(alias = "PortRole")]
    #[must_use]
    pub fn port_role(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `data_role` field.
    ///
    /// Data role, 0 for UFP, 1 for DFP
    #[doc(alias = "DataRole")]
    #[must_use]
    pub fn data_role(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 7` - Read the `erp_mode` field.
    ///
    /// Is EPR mode active
    #[doc(alias = "ErpMode")]
    #[must_use]
    pub fn erp_mode(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `21:20` - Read the `vbus_status` field.
    ///
    /// Vbus status
    #[doc(alias = "VbusStatus")]
    #[must_use]
    pub fn vbus_status(&self) -> VbusMode {
        let start = 20;
        let end = 21;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `23:22` - Read the `usb_host` field.
    ///
    /// USB host mode
    #[doc(alias = "UsbHost")]
    #[must_use]
    pub fn usb_host(&self) -> UsbHostMode {
        let start = 22;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `25:24` - Read the `legacy` field.
    ///
    /// Legacy mode stotus
    #[doc(alias = "Legacy")]
    #[must_use]
    pub fn legacy(&self) -> LegacyMode {
        let start = 24;
        let end = 25;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 27` - Read the `bist_in_progress` field.
    ///
    /// If a BIST is in progress
    #[doc(alias = "BistInProgress")]
    #[must_use]
    pub fn bist_in_progress(&self) -> bool {
        let start = 27;
        let end = 27;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 30` - Read the `soc_ack_timeout` field.
    ///
    /// Set when the SOC acknolwedgement has timed out
    #[doc(alias = "SocAckTimeout")]
    #[must_use]
    pub fn soc_ack_timeout(&self) -> bool {
        let start = 30;
        let end = 30;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `33:32` - Read the `am_status` field.
    ///
    /// Alternate mode entry status
    #[doc(alias = "AmStatus")]
    #[must_use]
    pub fn am_status(&self) -> AmStatus {
        let start = 32;
        let end = 33;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 0` - Set the `plug_present` field.
    ///
    /// Plug present
    #[doc(alias = "PlugPresent")]
    pub fn set_plug_present(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `3:1` - Set the `connection_state` field.
    ///
    /// Connection state
    #[doc(alias = "ConnectionState")]
    pub fn set_connection_state(&mut self, value: PlugMode) {
        let start = 1;
        let end = 3;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `plug_orientation` field.
    ///
    /// Connector oreintation, 0 for normal, 1 for flipped
    #[doc(alias = "PlugOrientation")]
    pub fn set_plug_orientation(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `port_role` field.
    ///
    /// PD role, 0 for sink, 1 for source
    #[doc(alias = "PortRole")]
    pub fn set_port_role(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `data_role` field.
    ///
    /// Data role, 0 for UFP, 1 for DFP
    #[doc(alias = "DataRole")]
    pub fn set_data_role(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `erp_mode` field.
    ///
    /// Is EPR mode active
    #[doc(alias = "ErpMode")]
    pub fn set_erp_mode(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `21:20` - Set the `vbus_status` field.
    ///
    /// Vbus status
    #[doc(alias = "VbusStatus")]
    pub fn set_vbus_status(&mut self, value: VbusMode) {
        let start = 20;
        let end = 21;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `23:22` - Set the `usb_host` field.
    ///
    /// USB host mode
    #[doc(alias = "UsbHost")]
    pub fn set_usb_host(&mut self, value: UsbHostMode) {
        let start = 22;
        let end = 23;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `25:24` - Set the `legacy` field.
    ///
    /// Legacy mode stotus
    #[doc(alias = "Legacy")]
    pub fn set_legacy(&mut self, value: LegacyMode) {
        let start = 24;
        let end = 25;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 27` - Set the `bist_in_progress` field.
    ///
    /// If a BIST is in progress
    #[doc(alias = "BistInProgress")]
    pub fn set_bist_in_progress(&mut self, value: bool) {
        let start = 27;
        let end = 27;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 30` - Set the `soc_ack_timeout` field.
    ///
    /// Set when the SOC acknolwedgement has timed out
    #[doc(alias = "SocAckTimeout")]
    pub fn set_soc_ack_timeout(&mut self, value: bool) {
        let start = 30;
        let end = 30;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `33:32` - Set the `am_status` field.
    ///
    /// Alternate mode entry status
    #[doc(alias = "AmStatus")]
    pub fn set_am_status(&mut self, value: AmStatus) {
        let start = 32;
        let end = 33;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for Status {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 5]> for Status {
    fn from(bits: [u8; 5]) -> Self {
        Self { bits }
    }
}
impl From<Status> for [u8; 5] {
    fn from(val: Status) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Status");
        d.field("plug_present", &self.plug_present());
        d.field("connection_state", &self.connection_state());
        d.field("plug_orientation", &self.plug_orientation());
        d.field("port_role", &self.port_role());
        d.field("data_role", &self.data_role());
        d.field("erp_mode", &self.erp_mode());
        d.field("vbus_status", &self.vbus_status());
        d.field("usb_host", &self.usb_host());
        d.field("legacy", &self.legacy());
        d.field("bist_in_progress", &self.bist_in_progress());
        d.field("soc_ack_timeout", &self.soc_ack_timeout());
        d.field("am_status", &self.am_status());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Status {{ ");
        defmt::write!(f, "plug_present: {=bool}, ", & self.plug_present());
        defmt::write!(f, "connection_state: {}, ", & self.connection_state());
        defmt::write!(f, "plug_orientation: {=bool}, ", & self.plug_orientation());
        defmt::write!(f, "port_role: {=bool}, ", & self.port_role());
        defmt::write!(f, "data_role: {=bool}, ", & self.data_role());
        defmt::write!(f, "erp_mode: {=bool}, ", & self.erp_mode());
        defmt::write!(f, "vbus_status: {}, ", & self.vbus_status());
        defmt::write!(f, "usb_host: {}, ", & self.usb_host());
        defmt::write!(f, "legacy: {}, ", & self.legacy());
        defmt::write!(f, "bist_in_progress: {=bool}, ", & self.bist_in_progress());
        defmt::write!(f, "soc_ack_timeout: {=bool}, ", & self.soc_ack_timeout());
        defmt::write!(f, "am_status: {}, ", & self.am_status());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Status {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Status {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Status {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Status {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Status {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Status {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Status {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct SxAppConfig {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for SxAppConfig {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl SxAppConfig {
    /// `2:0` - Read the `sleep_state` field.
    ///
    /// Current system power state
    #[doc(alias = "SleepState")]
    #[must_use]
    pub fn sleep_state(&self) -> SystemPowerState {
        let start = 0;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `2:0` - Set the `sleep_state` field.
    ///
    /// Current system power state
    #[doc(alias = "SleepState")]
    pub fn set_sleep_state(&mut self, value: SystemPowerState) {
        let start = 0;
        let end = 2;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for SxAppConfig {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for SxAppConfig {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<SxAppConfig> for [u8; 2] {
    fn from(val: SxAppConfig) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for SxAppConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("SxAppConfig");
        d.field("sleep_state", &self.sleep_state());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SxAppConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SxAppConfig {{ ");
        defmt::write!(f, "sleep_state: {}, ", & self.sleep_state());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for SxAppConfig {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for SxAppConfig {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for SxAppConfig {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for SxAppConfig {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for SxAppConfig {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for SxAppConfig {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for SxAppConfig {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct IntEventBus1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 11],
}
unsafe impl ::device_driver::Fieldset for IntEventBus1 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 11] };
}
impl IntEventBus1 {
    /// `bit 1` - Read the `hard_reset` field.
    ///
    /// A PD hard reset has been performed
    #[doc(alias = "HardReset")]
    #[must_use]
    pub fn hard_reset(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 3` - Read the `plug_event` field.
    ///
    /// A plug has been inserted or removed
    #[doc(alias = "PlugEvent")]
    #[must_use]
    pub fn plug_event(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 4` - Read the `power_swap_completed` field.
    ///
    /// Power swap completed
    #[doc(alias = "PowerSwapCompleted")]
    #[must_use]
    pub fn power_swap_completed(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 5` - Read the `data_swap_completed` field.
    ///
    /// Data swap completed
    #[doc(alias = "DataSwapCompleted")]
    #[must_use]
    pub fn data_swap_completed(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `fast_role_swap_completed` field.
    ///
    /// Fast role swap completed
    #[doc(alias = "FastRoleSwapCompleted")]
    #[must_use]
    pub fn fast_role_swap_completed(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 7` - Read the `source_cap_updated` field.
    ///
    /// Source capabilities updated
    #[doc(alias = "SourceCapUpdated")]
    #[must_use]
    pub fn source_cap_updated(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 8` - Read the `sink_ready` field.
    ///
    /// Asserts under an implicit contract or an explicit contract when PS_RDY has been received
    #[doc(alias = "SinkReady")]
    #[must_use]
    pub fn sink_ready(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 9` - Read the `overcurrent` field.
    ///
    /// Overcurrent
    #[doc(alias = "Overcurrent")]
    #[must_use]
    pub fn overcurrent(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 10` - Read the `attention_received` field.
    ///
    /// Attention received
    #[doc(alias = "AttentionReceived")]
    #[must_use]
    pub fn attention_received(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 11` - Read the `vdm_received` field.
    ///
    /// VDM received
    #[doc(alias = "VDMReceived")]
    #[must_use]
    pub fn vdm_received(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 12` - Read the `new_consumer_contract` field.
    ///
    /// New contract as consumer
    #[doc(alias = "NewConsumerContract")]
    #[must_use]
    pub fn new_consumer_contract(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 13` - Read the `new_provider_contract` field.
    ///
    /// New contract as provider
    #[doc(alias = "NewProviderContract")]
    #[must_use]
    pub fn new_provider_contract(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 14` - Read the `source_caps_received` field.
    ///
    /// Source capabilities received
    #[doc(alias = "SourceCapsReceived")]
    #[must_use]
    pub fn source_caps_received(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 15` - Read the `sink_caps_received` field.
    ///
    /// Sink capabilities received
    #[doc(alias = "SinkCapsReceived")]
    #[must_use]
    pub fn sink_caps_received(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 17` - Read the `power_swap_requested` field.
    ///
    /// Power swap requested
    #[doc(alias = "PowerSwapRequested")]
    #[must_use]
    pub fn power_swap_requested(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 18` - Read the `data_swap_requested` field.
    ///
    /// Data swap requested
    #[doc(alias = "DataSwapRequested")]
    #[must_use]
    pub fn data_swap_requested(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 20` - Read the `usb_host_present` field.
    ///
    /// USB host present
    #[doc(alias = "UsbHostPresent")]
    #[must_use]
    pub fn usb_host_present(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 21` - Read the `usb_host_not_present` field.
    ///
    /// Set when USB host status transitions to anything other than present
    #[doc(alias = "UsbHostNotPresent")]
    #[must_use]
    pub fn usb_host_not_present(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 23` - Read the `power_path_switch_changed` field.
    ///
    /// Power path status register changed
    #[doc(alias = "PowerPathSwitchChanged")]
    #[must_use]
    pub fn power_path_switch_changed(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 25` - Read the `data_status_updated` field.
    ///
    /// Data status register changed
    #[doc(alias = "DataStatusUpdated")]
    #[must_use]
    pub fn data_status_updated(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 26` - Read the `status_updated` field.
    ///
    /// Status register changed
    #[doc(alias = "StatusUpdated")]
    #[must_use]
    pub fn status_updated(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 27` - Read the `pd_status_updated` field.
    ///
    /// PD status register changed
    #[doc(alias = "PdStatusUpdated")]
    #[must_use]
    pub fn pd_status_updated(&self) -> bool {
        let start = 27;
        let end = 27;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 30` - Read the `cmd_1_completed` field.
    ///
    /// Command 1 completed
    #[doc(alias = "Cmd1Completed")]
    #[must_use]
    pub fn cmd_1_completed(&self) -> bool {
        let start = 30;
        let end = 30;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 31` - Read the `cmd_2_completed` field.
    ///
    /// Command 2 completed
    #[doc(alias = "Cmd2Completed")]
    #[must_use]
    pub fn cmd_2_completed(&self) -> bool {
        let start = 31;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 32` - Read the `device_incompatible` field.
    ///
    /// Device lacks PD or has incompatible PD version
    #[doc(alias = "DeviceIncompatible")]
    #[must_use]
    pub fn device_incompatible(&self) -> bool {
        let start = 32;
        let end = 32;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 33` - Read the `cannot_source` field.
    ///
    /// Source cannot supply requested voltage or current
    #[doc(alias = "CannotSource")]
    #[must_use]
    pub fn cannot_source(&self) -> bool {
        let start = 33;
        let end = 33;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 34` - Read the `can_source_later` field.
    ///
    /// Source can supply requested voltage or current later
    #[doc(alias = "CanSourceLater")]
    #[must_use]
    pub fn can_source_later(&self) -> bool {
        let start = 34;
        let end = 34;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 35` - Read the `power_event_error` field.
    ///
    /// Voltage or current exceeded
    #[doc(alias = "PowerEventError")]
    #[must_use]
    pub fn power_event_error(&self) -> bool {
        let start = 35;
        let end = 35;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 36` - Read the `no_caps_response` field.
    ///
    /// Device did not response to get caps message
    #[doc(alias = "NoCapsResponse")]
    #[must_use]
    pub fn no_caps_response(&self) -> bool {
        let start = 36;
        let end = 36;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 38` - Read the `protocol_error` field.
    ///
    /// Unexpected message received from partner
    #[doc(alias = "ProtocolError")]
    #[must_use]
    pub fn protocol_error(&self) -> bool {
        let start = 38;
        let end = 38;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 42` - Read the `sink_transition_completed` field.
    ///
    /// Sink transition completed
    #[doc(alias = "SinkTransitionCompleted")]
    #[must_use]
    pub fn sink_transition_completed(&self) -> bool {
        let start = 42;
        let end = 42;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 43` - Read the `plug_early_notification` field.
    ///
    /// Plug connected but not debounced
    #[doc(alias = "PlugEarlyNotification")]
    #[must_use]
    pub fn plug_early_notification(&self) -> bool {
        let start = 43;
        let end = 43;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 44` - Read the `prochot_notification` field.
    ///
    /// Prochot asserted
    #[doc(alias = "ProchotNotification")]
    #[must_use]
    pub fn prochot_notification(&self) -> bool {
        let start = 44;
        let end = 44;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 46` - Read the `source_cannot_provide` field.
    ///
    /// Source cannot produce negociated voltage or current
    #[doc(alias = "SourceCannotProvide")]
    #[must_use]
    pub fn source_cannot_provide(&self) -> bool {
        let start = 46;
        let end = 46;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 48` - Read the `am_entry_fail` field.
    ///
    /// Alternate mode entry failed
    #[doc(alias = "AmEntryFail")]
    #[must_use]
    pub fn am_entry_fail(&self) -> bool {
        let start = 48;
        let end = 48;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 49` - Read the `am_entered` field.
    ///
    /// Alternate mode entered
    #[doc(alias = "AmEntered")]
    #[must_use]
    pub fn am_entered(&self) -> bool {
        let start = 49;
        let end = 49;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 51` - Read the `discover_mode_completed` field.
    ///
    /// Discover modes process completed
    #[doc(alias = "DiscoverModeCompleted")]
    #[must_use]
    pub fn discover_mode_completed(&self) -> bool {
        let start = 51;
        let end = 51;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 52` - Read the `exit_mode_completed` field.
    ///
    /// Exit mode process completed
    #[doc(alias = "ExitModeCompleted")]
    #[must_use]
    pub fn exit_mode_completed(&self) -> bool {
        let start = 52;
        let end = 52;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 53` - Read the `data_reset_started` field.
    ///
    /// Data reset process started
    #[doc(alias = "DataResetStarted")]
    #[must_use]
    pub fn data_reset_started(&self) -> bool {
        let start = 53;
        let end = 53;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 54` - Read the `usb_status_updated` field.
    ///
    /// USB status updated
    #[doc(alias = "UsbStatusUpdated")]
    #[must_use]
    pub fn usb_status_updated(&self) -> bool {
        let start = 54;
        let end = 54;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 55` - Read the `connection_manager_updated` field.
    ///
    /// Connection manager updated
    #[doc(alias = "ConnectionManagerUpdated")]
    #[must_use]
    pub fn connection_manager_updated(&self) -> bool {
        let start = 55;
        let end = 55;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 56` - Read the `usvid_mode_entered` field.
    ///
    /// User VID alternate mode entered
    #[doc(alias = "UsvidModeEntered")]
    #[must_use]
    pub fn usvid_mode_entered(&self) -> bool {
        let start = 56;
        let end = 56;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 57` - Read the `usvid_mode_exited` field.
    ///
    /// User VID alternate mode entered
    #[doc(alias = "UsvidModeExited")]
    #[must_use]
    pub fn usvid_mode_exited(&self) -> bool {
        let start = 57;
        let end = 57;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 58` - Read the `usvid_attention_vdm_received` field.
    ///
    /// User VID SVDM attention received
    #[doc(alias = "UsvidAttentionVdmReceived")]
    #[must_use]
    pub fn usvid_attention_vdm_received(&self) -> bool {
        let start = 58;
        let end = 58;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 59` - Read the `usvid_other_vdm_received` field.
    ///
    /// User VID SVDM non-attention or unstructured VDM received
    #[doc(alias = "UsvidOtherVdmReceived")]
    #[must_use]
    pub fn usvid_other_vdm_received(&self) -> bool {
        let start = 59;
        let end = 59;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 61` - Read the `external_dc_dc_event` field.
    ///
    /// External DCDC event
    #[doc(alias = "ExternalDcDcEvent")]
    #[must_use]
    pub fn external_dc_dc_event(&self) -> bool {
        let start = 61;
        let end = 61;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 62` - Read the `dp_sid_status_updated` field.
    ///
    /// DP SID status register changed
    #[doc(alias = "DpSidStatusUpdated")]
    #[must_use]
    pub fn dp_sid_status_updated(&self) -> bool {
        let start = 62;
        let end = 62;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 63` - Read the `intel_vid_status_updated` field.
    ///
    /// Intel VID status register changed
    #[doc(alias = "IntelVidStatusUpdated")]
    #[must_use]
    pub fn intel_vid_status_updated(&self) -> bool {
        let start = 63;
        let end = 63;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 64` - Read the `pd_3_status_updated` field.
    ///
    /// PD3 status register changed
    #[doc(alias = "Pd3StatusUpdated")]
    #[must_use]
    pub fn pd_3_status_updated(&self) -> bool {
        let start = 64;
        let end = 64;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 65` - Read the `tx_memory_buffer_empty` field.
    ///
    /// TX memory buffer empty
    #[doc(alias = "TxMemoryBufferEmpty")]
    #[must_use]
    pub fn tx_memory_buffer_empty(&self) -> bool {
        let start = 65;
        let end = 65;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 66` - Read the `mbrd_buffer_ready` field.
    ///
    /// Buffer for mbrd command received and ready
    #[doc(alias = "MbrdBufferReady")]
    #[must_use]
    pub fn mbrd_buffer_ready(&self) -> bool {
        let start = 66;
        let end = 66;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 70` - Read the `soc_ack_timeout` field.
    ///
    /// SOC ack timeout
    #[doc(alias = "SocAckTimeout")]
    #[must_use]
    pub fn soc_ack_timeout(&self) -> bool {
        let start = 70;
        let end = 70;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 71` - Read the `not_supported_received` field.
    ///
    /// Not supported PD message received
    #[doc(alias = "NotSupportedReceived")]
    #[must_use]
    pub fn not_supported_received(&self) -> bool {
        let start = 71;
        let end = 71;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 72` - Read the `crossbar_error` field.
    ///
    /// Error configuring the crossbar mux
    #[doc(alias = "CrossbarError")]
    #[must_use]
    pub fn crossbar_error(&self) -> bool {
        let start = 72;
        let end = 72;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 73` - Read the `mailbox_updated` field.
    ///
    /// Mailbox updated
    #[doc(alias = "MailboxUpdated")]
    #[must_use]
    pub fn mailbox_updated(&self) -> bool {
        let start = 73;
        let end = 73;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 74` - Read the `bus_error` field.
    ///
    /// I2C error communicating with external bus
    #[doc(alias = "BusError")]
    #[must_use]
    pub fn bus_error(&self) -> bool {
        let start = 74;
        let end = 74;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 75` - Read the `external_dc_dc_status_changed` field.
    ///
    /// External DCDC status changed
    #[doc(alias = "ExternalDcDcStatusChanged")]
    #[must_use]
    pub fn external_dc_dc_status_changed(&self) -> bool {
        let start = 75;
        let end = 75;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 76` - Read the `frs_signal_received` field.
    ///
    /// Fast role swap signal received
    #[doc(alias = "FrsSignalReceived")]
    #[must_use]
    pub fn frs_signal_received(&self) -> bool {
        let start = 76;
        let end = 76;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 77` - Read the `chunk_response_received` field.
    ///
    /// Chunk response received
    #[doc(alias = "ChunkResponseReceived")]
    #[must_use]
    pub fn chunk_response_received(&self) -> bool {
        let start = 77;
        let end = 77;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 78` - Read the `chunk_request_received` field.
    ///
    /// Chunk request received
    #[doc(alias = "ChunkRequestReceived")]
    #[must_use]
    pub fn chunk_request_received(&self) -> bool {
        let start = 78;
        let end = 78;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 79` - Read the `alert_message_received` field.
    ///
    /// Alert message received
    #[doc(alias = "AlertMessageReceived")]
    #[must_use]
    pub fn alert_message_received(&self) -> bool {
        let start = 79;
        let end = 79;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 80` - Read the `patch_loaded` field.
    ///
    /// Patch loaded to device
    #[doc(alias = "PatchLoaded")]
    #[must_use]
    pub fn patch_loaded(&self) -> bool {
        let start = 80;
        let end = 80;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 81` - Read the `ready_f_211` field.
    ///
    /// Ready for F211 image
    #[doc(alias = "ReadyF211")]
    #[must_use]
    pub fn ready_f_211(&self) -> bool {
        let start = 81;
        let end = 81;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 84` - Read the `boot_error` field.
    ///
    /// Boot error
    #[doc(alias = "BootError")]
    #[must_use]
    pub fn boot_error(&self) -> bool {
        let start = 84;
        let end = 84;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 85` - Read the `ready_for_data_block` field.
    ///
    /// Ready for data block
    #[doc(alias = "ReadyForDataBlock")]
    #[must_use]
    pub fn ready_for_data_block(&self) -> bool {
        let start = 85;
        let end = 85;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Set the `hard_reset` field.
    ///
    /// A PD hard reset has been performed
    #[doc(alias = "HardReset")]
    pub fn set_hard_reset(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 3` - Set the `plug_event` field.
    ///
    /// A plug has been inserted or removed
    #[doc(alias = "PlugEvent")]
    pub fn set_plug_event(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `power_swap_completed` field.
    ///
    /// Power swap completed
    #[doc(alias = "PowerSwapCompleted")]
    pub fn set_power_swap_completed(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `data_swap_completed` field.
    ///
    /// Data swap completed
    #[doc(alias = "DataSwapCompleted")]
    pub fn set_data_swap_completed(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `fast_role_swap_completed` field.
    ///
    /// Fast role swap completed
    #[doc(alias = "FastRoleSwapCompleted")]
    pub fn set_fast_role_swap_completed(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `source_cap_updated` field.
    ///
    /// Source capabilities updated
    #[doc(alias = "SourceCapUpdated")]
    pub fn set_source_cap_updated(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 8` - Set the `sink_ready` field.
    ///
    /// Asserts under an implicit contract or an explicit contract when PS_RDY has been received
    #[doc(alias = "SinkReady")]
    pub fn set_sink_ready(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 9` - Set the `overcurrent` field.
    ///
    /// Overcurrent
    #[doc(alias = "Overcurrent")]
    pub fn set_overcurrent(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 10` - Set the `attention_received` field.
    ///
    /// Attention received
    #[doc(alias = "AttentionReceived")]
    pub fn set_attention_received(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 11` - Set the `vdm_received` field.
    ///
    /// VDM received
    #[doc(alias = "VDMReceived")]
    pub fn set_vdm_received(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 12` - Set the `new_consumer_contract` field.
    ///
    /// New contract as consumer
    #[doc(alias = "NewConsumerContract")]
    pub fn set_new_consumer_contract(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 13` - Set the `new_provider_contract` field.
    ///
    /// New contract as provider
    #[doc(alias = "NewProviderContract")]
    pub fn set_new_provider_contract(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 14` - Set the `source_caps_received` field.
    ///
    /// Source capabilities received
    #[doc(alias = "SourceCapsReceived")]
    pub fn set_source_caps_received(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 15` - Set the `sink_caps_received` field.
    ///
    /// Sink capabilities received
    #[doc(alias = "SinkCapsReceived")]
    pub fn set_sink_caps_received(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 17` - Set the `power_swap_requested` field.
    ///
    /// Power swap requested
    #[doc(alias = "PowerSwapRequested")]
    pub fn set_power_swap_requested(&mut self, value: bool) {
        let start = 17;
        let end = 17;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 18` - Set the `data_swap_requested` field.
    ///
    /// Data swap requested
    #[doc(alias = "DataSwapRequested")]
    pub fn set_data_swap_requested(&mut self, value: bool) {
        let start = 18;
        let end = 18;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 20` - Set the `usb_host_present` field.
    ///
    /// USB host present
    #[doc(alias = "UsbHostPresent")]
    pub fn set_usb_host_present(&mut self, value: bool) {
        let start = 20;
        let end = 20;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 21` - Set the `usb_host_not_present` field.
    ///
    /// Set when USB host status transitions to anything other than present
    #[doc(alias = "UsbHostNotPresent")]
    pub fn set_usb_host_not_present(&mut self, value: bool) {
        let start = 21;
        let end = 21;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 23` - Set the `power_path_switch_changed` field.
    ///
    /// Power path status register changed
    #[doc(alias = "PowerPathSwitchChanged")]
    pub fn set_power_path_switch_changed(&mut self, value: bool) {
        let start = 23;
        let end = 23;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 25` - Set the `data_status_updated` field.
    ///
    /// Data status register changed
    #[doc(alias = "DataStatusUpdated")]
    pub fn set_data_status_updated(&mut self, value: bool) {
        let start = 25;
        let end = 25;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 26` - Set the `status_updated` field.
    ///
    /// Status register changed
    #[doc(alias = "StatusUpdated")]
    pub fn set_status_updated(&mut self, value: bool) {
        let start = 26;
        let end = 26;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 27` - Set the `pd_status_updated` field.
    ///
    /// PD status register changed
    #[doc(alias = "PdStatusUpdated")]
    pub fn set_pd_status_updated(&mut self, value: bool) {
        let start = 27;
        let end = 27;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 30` - Set the `cmd_1_completed` field.
    ///
    /// Command 1 completed
    #[doc(alias = "Cmd1Completed")]
    pub fn set_cmd_1_completed(&mut self, value: bool) {
        let start = 30;
        let end = 30;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 31` - Set the `cmd_2_completed` field.
    ///
    /// Command 2 completed
    #[doc(alias = "Cmd2Completed")]
    pub fn set_cmd_2_completed(&mut self, value: bool) {
        let start = 31;
        let end = 31;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 32` - Set the `device_incompatible` field.
    ///
    /// Device lacks PD or has incompatible PD version
    #[doc(alias = "DeviceIncompatible")]
    pub fn set_device_incompatible(&mut self, value: bool) {
        let start = 32;
        let end = 32;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 33` - Set the `cannot_source` field.
    ///
    /// Source cannot supply requested voltage or current
    #[doc(alias = "CannotSource")]
    pub fn set_cannot_source(&mut self, value: bool) {
        let start = 33;
        let end = 33;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 34` - Set the `can_source_later` field.
    ///
    /// Source can supply requested voltage or current later
    #[doc(alias = "CanSourceLater")]
    pub fn set_can_source_later(&mut self, value: bool) {
        let start = 34;
        let end = 34;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 35` - Set the `power_event_error` field.
    ///
    /// Voltage or current exceeded
    #[doc(alias = "PowerEventError")]
    pub fn set_power_event_error(&mut self, value: bool) {
        let start = 35;
        let end = 35;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 36` - Set the `no_caps_response` field.
    ///
    /// Device did not response to get caps message
    #[doc(alias = "NoCapsResponse")]
    pub fn set_no_caps_response(&mut self, value: bool) {
        let start = 36;
        let end = 36;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 38` - Set the `protocol_error` field.
    ///
    /// Unexpected message received from partner
    #[doc(alias = "ProtocolError")]
    pub fn set_protocol_error(&mut self, value: bool) {
        let start = 38;
        let end = 38;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 42` - Set the `sink_transition_completed` field.
    ///
    /// Sink transition completed
    #[doc(alias = "SinkTransitionCompleted")]
    pub fn set_sink_transition_completed(&mut self, value: bool) {
        let start = 42;
        let end = 42;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 43` - Set the `plug_early_notification` field.
    ///
    /// Plug connected but not debounced
    #[doc(alias = "PlugEarlyNotification")]
    pub fn set_plug_early_notification(&mut self, value: bool) {
        let start = 43;
        let end = 43;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 44` - Set the `prochot_notification` field.
    ///
    /// Prochot asserted
    #[doc(alias = "ProchotNotification")]
    pub fn set_prochot_notification(&mut self, value: bool) {
        let start = 44;
        let end = 44;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 46` - Set the `source_cannot_provide` field.
    ///
    /// Source cannot produce negociated voltage or current
    #[doc(alias = "SourceCannotProvide")]
    pub fn set_source_cannot_provide(&mut self, value: bool) {
        let start = 46;
        let end = 46;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 48` - Set the `am_entry_fail` field.
    ///
    /// Alternate mode entry failed
    #[doc(alias = "AmEntryFail")]
    pub fn set_am_entry_fail(&mut self, value: bool) {
        let start = 48;
        let end = 48;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 49` - Set the `am_entered` field.
    ///
    /// Alternate mode entered
    #[doc(alias = "AmEntered")]
    pub fn set_am_entered(&mut self, value: bool) {
        let start = 49;
        let end = 49;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 51` - Set the `discover_mode_completed` field.
    ///
    /// Discover modes process completed
    #[doc(alias = "DiscoverModeCompleted")]
    pub fn set_discover_mode_completed(&mut self, value: bool) {
        let start = 51;
        let end = 51;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 52` - Set the `exit_mode_completed` field.
    ///
    /// Exit mode process completed
    #[doc(alias = "ExitModeCompleted")]
    pub fn set_exit_mode_completed(&mut self, value: bool) {
        let start = 52;
        let end = 52;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 53` - Set the `data_reset_started` field.
    ///
    /// Data reset process started
    #[doc(alias = "DataResetStarted")]
    pub fn set_data_reset_started(&mut self, value: bool) {
        let start = 53;
        let end = 53;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 54` - Set the `usb_status_updated` field.
    ///
    /// USB status updated
    #[doc(alias = "UsbStatusUpdated")]
    pub fn set_usb_status_updated(&mut self, value: bool) {
        let start = 54;
        let end = 54;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 55` - Set the `connection_manager_updated` field.
    ///
    /// Connection manager updated
    #[doc(alias = "ConnectionManagerUpdated")]
    pub fn set_connection_manager_updated(&mut self, value: bool) {
        let start = 55;
        let end = 55;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 56` - Set the `usvid_mode_entered` field.
    ///
    /// User VID alternate mode entered
    #[doc(alias = "UsvidModeEntered")]
    pub fn set_usvid_mode_entered(&mut self, value: bool) {
        let start = 56;
        let end = 56;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 57` - Set the `usvid_mode_exited` field.
    ///
    /// User VID alternate mode entered
    #[doc(alias = "UsvidModeExited")]
    pub fn set_usvid_mode_exited(&mut self, value: bool) {
        let start = 57;
        let end = 57;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 58` - Set the `usvid_attention_vdm_received` field.
    ///
    /// User VID SVDM attention received
    #[doc(alias = "UsvidAttentionVdmReceived")]
    pub fn set_usvid_attention_vdm_received(&mut self, value: bool) {
        let start = 58;
        let end = 58;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 59` - Set the `usvid_other_vdm_received` field.
    ///
    /// User VID SVDM non-attention or unstructured VDM received
    #[doc(alias = "UsvidOtherVdmReceived")]
    pub fn set_usvid_other_vdm_received(&mut self, value: bool) {
        let start = 59;
        let end = 59;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 61` - Set the `external_dc_dc_event` field.
    ///
    /// External DCDC event
    #[doc(alias = "ExternalDcDcEvent")]
    pub fn set_external_dc_dc_event(&mut self, value: bool) {
        let start = 61;
        let end = 61;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 62` - Set the `dp_sid_status_updated` field.
    ///
    /// DP SID status register changed
    #[doc(alias = "DpSidStatusUpdated")]
    pub fn set_dp_sid_status_updated(&mut self, value: bool) {
        let start = 62;
        let end = 62;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 63` - Set the `intel_vid_status_updated` field.
    ///
    /// Intel VID status register changed
    #[doc(alias = "IntelVidStatusUpdated")]
    pub fn set_intel_vid_status_updated(&mut self, value: bool) {
        let start = 63;
        let end = 63;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 64` - Set the `pd_3_status_updated` field.
    ///
    /// PD3 status register changed
    #[doc(alias = "Pd3StatusUpdated")]
    pub fn set_pd_3_status_updated(&mut self, value: bool) {
        let start = 64;
        let end = 64;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 65` - Set the `tx_memory_buffer_empty` field.
    ///
    /// TX memory buffer empty
    #[doc(alias = "TxMemoryBufferEmpty")]
    pub fn set_tx_memory_buffer_empty(&mut self, value: bool) {
        let start = 65;
        let end = 65;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 66` - Set the `mbrd_buffer_ready` field.
    ///
    /// Buffer for mbrd command received and ready
    #[doc(alias = "MbrdBufferReady")]
    pub fn set_mbrd_buffer_ready(&mut self, value: bool) {
        let start = 66;
        let end = 66;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 70` - Set the `soc_ack_timeout` field.
    ///
    /// SOC ack timeout
    #[doc(alias = "SocAckTimeout")]
    pub fn set_soc_ack_timeout(&mut self, value: bool) {
        let start = 70;
        let end = 70;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 71` - Set the `not_supported_received` field.
    ///
    /// Not supported PD message received
    #[doc(alias = "NotSupportedReceived")]
    pub fn set_not_supported_received(&mut self, value: bool) {
        let start = 71;
        let end = 71;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 72` - Set the `crossbar_error` field.
    ///
    /// Error configuring the crossbar mux
    #[doc(alias = "CrossbarError")]
    pub fn set_crossbar_error(&mut self, value: bool) {
        let start = 72;
        let end = 72;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 73` - Set the `mailbox_updated` field.
    ///
    /// Mailbox updated
    #[doc(alias = "MailboxUpdated")]
    pub fn set_mailbox_updated(&mut self, value: bool) {
        let start = 73;
        let end = 73;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 74` - Set the `bus_error` field.
    ///
    /// I2C error communicating with external bus
    #[doc(alias = "BusError")]
    pub fn set_bus_error(&mut self, value: bool) {
        let start = 74;
        let end = 74;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 75` - Set the `external_dc_dc_status_changed` field.
    ///
    /// External DCDC status changed
    #[doc(alias = "ExternalDcDcStatusChanged")]
    pub fn set_external_dc_dc_status_changed(&mut self, value: bool) {
        let start = 75;
        let end = 75;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 76` - Set the `frs_signal_received` field.
    ///
    /// Fast role swap signal received
    #[doc(alias = "FrsSignalReceived")]
    pub fn set_frs_signal_received(&mut self, value: bool) {
        let start = 76;
        let end = 76;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 77` - Set the `chunk_response_received` field.
    ///
    /// Chunk response received
    #[doc(alias = "ChunkResponseReceived")]
    pub fn set_chunk_response_received(&mut self, value: bool) {
        let start = 77;
        let end = 77;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 78` - Set the `chunk_request_received` field.
    ///
    /// Chunk request received
    #[doc(alias = "ChunkRequestReceived")]
    pub fn set_chunk_request_received(&mut self, value: bool) {
        let start = 78;
        let end = 78;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 79` - Set the `alert_message_received` field.
    ///
    /// Alert message received
    #[doc(alias = "AlertMessageReceived")]
    pub fn set_alert_message_received(&mut self, value: bool) {
        let start = 79;
        let end = 79;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 80` - Set the `patch_loaded` field.
    ///
    /// Patch loaded to device
    #[doc(alias = "PatchLoaded")]
    pub fn set_patch_loaded(&mut self, value: bool) {
        let start = 80;
        let end = 80;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 81` - Set the `ready_f_211` field.
    ///
    /// Ready for F211 image
    #[doc(alias = "ReadyF211")]
    pub fn set_ready_f_211(&mut self, value: bool) {
        let start = 81;
        let end = 81;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 84` - Set the `boot_error` field.
    ///
    /// Boot error
    #[doc(alias = "BootError")]
    pub fn set_boot_error(&mut self, value: bool) {
        let start = 84;
        let end = 84;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 85` - Set the `ready_for_data_block` field.
    ///
    /// Ready for data block
    #[doc(alias = "ReadyForDataBlock")]
    pub fn set_ready_for_data_block(&mut self, value: bool) {
        let start = 85;
        let end = 85;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for IntEventBus1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 11]> for IntEventBus1 {
    fn from(bits: [u8; 11]) -> Self {
        Self { bits }
    }
}
impl From<IntEventBus1> for [u8; 11] {
    fn from(val: IntEventBus1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for IntEventBus1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("IntEventBus1");
        d.field("hard_reset", &self.hard_reset());
        d.field("plug_event", &self.plug_event());
        d.field("power_swap_completed", &self.power_swap_completed());
        d.field("data_swap_completed", &self.data_swap_completed());
        d.field("fast_role_swap_completed", &self.fast_role_swap_completed());
        d.field("source_cap_updated", &self.source_cap_updated());
        d.field("sink_ready", &self.sink_ready());
        d.field("overcurrent", &self.overcurrent());
        d.field("attention_received", &self.attention_received());
        d.field("vdm_received", &self.vdm_received());
        d.field("new_consumer_contract", &self.new_consumer_contract());
        d.field("new_provider_contract", &self.new_provider_contract());
        d.field("source_caps_received", &self.source_caps_received());
        d.field("sink_caps_received", &self.sink_caps_received());
        d.field("power_swap_requested", &self.power_swap_requested());
        d.field("data_swap_requested", &self.data_swap_requested());
        d.field("usb_host_present", &self.usb_host_present());
        d.field("usb_host_not_present", &self.usb_host_not_present());
        d.field("power_path_switch_changed", &self.power_path_switch_changed());
        d.field("data_status_updated", &self.data_status_updated());
        d.field("status_updated", &self.status_updated());
        d.field("pd_status_updated", &self.pd_status_updated());
        d.field("cmd_1_completed", &self.cmd_1_completed());
        d.field("cmd_2_completed", &self.cmd_2_completed());
        d.field("device_incompatible", &self.device_incompatible());
        d.field("cannot_source", &self.cannot_source());
        d.field("can_source_later", &self.can_source_later());
        d.field("power_event_error", &self.power_event_error());
        d.field("no_caps_response", &self.no_caps_response());
        d.field("protocol_error", &self.protocol_error());
        d.field("sink_transition_completed", &self.sink_transition_completed());
        d.field("plug_early_notification", &self.plug_early_notification());
        d.field("prochot_notification", &self.prochot_notification());
        d.field("source_cannot_provide", &self.source_cannot_provide());
        d.field("am_entry_fail", &self.am_entry_fail());
        d.field("am_entered", &self.am_entered());
        d.field("discover_mode_completed", &self.discover_mode_completed());
        d.field("exit_mode_completed", &self.exit_mode_completed());
        d.field("data_reset_started", &self.data_reset_started());
        d.field("usb_status_updated", &self.usb_status_updated());
        d.field("connection_manager_updated", &self.connection_manager_updated());
        d.field("usvid_mode_entered", &self.usvid_mode_entered());
        d.field("usvid_mode_exited", &self.usvid_mode_exited());
        d.field("usvid_attention_vdm_received", &self.usvid_attention_vdm_received());
        d.field("usvid_other_vdm_received", &self.usvid_other_vdm_received());
        d.field("external_dc_dc_event", &self.external_dc_dc_event());
        d.field("dp_sid_status_updated", &self.dp_sid_status_updated());
        d.field("intel_vid_status_updated", &self.intel_vid_status_updated());
        d.field("pd_3_status_updated", &self.pd_3_status_updated());
        d.field("tx_memory_buffer_empty", &self.tx_memory_buffer_empty());
        d.field("mbrd_buffer_ready", &self.mbrd_buffer_ready());
        d.field("soc_ack_timeout", &self.soc_ack_timeout());
        d.field("not_supported_received", &self.not_supported_received());
        d.field("crossbar_error", &self.crossbar_error());
        d.field("mailbox_updated", &self.mailbox_updated());
        d.field("bus_error", &self.bus_error());
        d.field("external_dc_dc_status_changed", &self.external_dc_dc_status_changed());
        d.field("frs_signal_received", &self.frs_signal_received());
        d.field("chunk_response_received", &self.chunk_response_received());
        d.field("chunk_request_received", &self.chunk_request_received());
        d.field("alert_message_received", &self.alert_message_received());
        d.field("patch_loaded", &self.patch_loaded());
        d.field("ready_f_211", &self.ready_f_211());
        d.field("boot_error", &self.boot_error());
        d.field("ready_for_data_block", &self.ready_for_data_block());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntEventBus1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntEventBus1 {{ ");
        defmt::write!(f, "hard_reset: {=bool}, ", & self.hard_reset());
        defmt::write!(f, "plug_event: {=bool}, ", & self.plug_event());
        defmt::write!(
            f, "power_swap_completed: {=bool}, ", & self.power_swap_completed()
        );
        defmt::write!(f, "data_swap_completed: {=bool}, ", & self.data_swap_completed());
        defmt::write!(
            f, "fast_role_swap_completed: {=bool}, ", & self.fast_role_swap_completed()
        );
        defmt::write!(f, "source_cap_updated: {=bool}, ", & self.source_cap_updated());
        defmt::write!(f, "sink_ready: {=bool}, ", & self.sink_ready());
        defmt::write!(f, "overcurrent: {=bool}, ", & self.overcurrent());
        defmt::write!(f, "attention_received: {=bool}, ", & self.attention_received());
        defmt::write!(f, "vdm_received: {=bool}, ", & self.vdm_received());
        defmt::write!(
            f, "new_consumer_contract: {=bool}, ", & self.new_consumer_contract()
        );
        defmt::write!(
            f, "new_provider_contract: {=bool}, ", & self.new_provider_contract()
        );
        defmt::write!(
            f, "source_caps_received: {=bool}, ", & self.source_caps_received()
        );
        defmt::write!(f, "sink_caps_received: {=bool}, ", & self.sink_caps_received());
        defmt::write!(
            f, "power_swap_requested: {=bool}, ", & self.power_swap_requested()
        );
        defmt::write!(f, "data_swap_requested: {=bool}, ", & self.data_swap_requested());
        defmt::write!(f, "usb_host_present: {=bool}, ", & self.usb_host_present());
        defmt::write!(
            f, "usb_host_not_present: {=bool}, ", & self.usb_host_not_present()
        );
        defmt::write!(
            f, "power_path_switch_changed: {=bool}, ", & self.power_path_switch_changed()
        );
        defmt::write!(f, "data_status_updated: {=bool}, ", & self.data_status_updated());
        defmt::write!(f, "status_updated: {=bool}, ", & self.status_updated());
        defmt::write!(f, "pd_status_updated: {=bool}, ", & self.pd_status_updated());
        defmt::write!(f, "cmd_1_completed: {=bool}, ", & self.cmd_1_completed());
        defmt::write!(f, "cmd_2_completed: {=bool}, ", & self.cmd_2_completed());
        defmt::write!(f, "device_incompatible: {=bool}, ", & self.device_incompatible());
        defmt::write!(f, "cannot_source: {=bool}, ", & self.cannot_source());
        defmt::write!(f, "can_source_later: {=bool}, ", & self.can_source_later());
        defmt::write!(f, "power_event_error: {=bool}, ", & self.power_event_error());
        defmt::write!(f, "no_caps_response: {=bool}, ", & self.no_caps_response());
        defmt::write!(f, "protocol_error: {=bool}, ", & self.protocol_error());
        defmt::write!(
            f, "sink_transition_completed: {=bool}, ", & self.sink_transition_completed()
        );
        defmt::write!(
            f, "plug_early_notification: {=bool}, ", & self.plug_early_notification()
        );
        defmt::write!(
            f, "prochot_notification: {=bool}, ", & self.prochot_notification()
        );
        defmt::write!(
            f, "source_cannot_provide: {=bool}, ", & self.source_cannot_provide()
        );
        defmt::write!(f, "am_entry_fail: {=bool}, ", & self.am_entry_fail());
        defmt::write!(f, "am_entered: {=bool}, ", & self.am_entered());
        defmt::write!(
            f, "discover_mode_completed: {=bool}, ", & self.discover_mode_completed()
        );
        defmt::write!(f, "exit_mode_completed: {=bool}, ", & self.exit_mode_completed());
        defmt::write!(f, "data_reset_started: {=bool}, ", & self.data_reset_started());
        defmt::write!(f, "usb_status_updated: {=bool}, ", & self.usb_status_updated());
        defmt::write!(
            f, "connection_manager_updated: {=bool}, ", & self
            .connection_manager_updated()
        );
        defmt::write!(f, "usvid_mode_entered: {=bool}, ", & self.usvid_mode_entered());
        defmt::write!(f, "usvid_mode_exited: {=bool}, ", & self.usvid_mode_exited());
        defmt::write!(
            f, "usvid_attention_vdm_received: {=bool}, ", & self
            .usvid_attention_vdm_received()
        );
        defmt::write!(
            f, "usvid_other_vdm_received: {=bool}, ", & self.usvid_other_vdm_received()
        );
        defmt::write!(
            f, "external_dc_dc_event: {=bool}, ", & self.external_dc_dc_event()
        );
        defmt::write!(
            f, "dp_sid_status_updated: {=bool}, ", & self.dp_sid_status_updated()
        );
        defmt::write!(
            f, "intel_vid_status_updated: {=bool}, ", & self.intel_vid_status_updated()
        );
        defmt::write!(f, "pd_3_status_updated: {=bool}, ", & self.pd_3_status_updated());
        defmt::write!(
            f, "tx_memory_buffer_empty: {=bool}, ", & self.tx_memory_buffer_empty()
        );
        defmt::write!(f, "mbrd_buffer_ready: {=bool}, ", & self.mbrd_buffer_ready());
        defmt::write!(f, "soc_ack_timeout: {=bool}, ", & self.soc_ack_timeout());
        defmt::write!(
            f, "not_supported_received: {=bool}, ", & self.not_supported_received()
        );
        defmt::write!(f, "crossbar_error: {=bool}, ", & self.crossbar_error());
        defmt::write!(f, "mailbox_updated: {=bool}, ", & self.mailbox_updated());
        defmt::write!(f, "bus_error: {=bool}, ", & self.bus_error());
        defmt::write!(
            f, "external_dc_dc_status_changed: {=bool}, ", & self
            .external_dc_dc_status_changed()
        );
        defmt::write!(f, "frs_signal_received: {=bool}, ", & self.frs_signal_received());
        defmt::write!(
            f, "chunk_response_received: {=bool}, ", & self.chunk_response_received()
        );
        defmt::write!(
            f, "chunk_request_received: {=bool}, ", & self.chunk_request_received()
        );
        defmt::write!(
            f, "alert_message_received: {=bool}, ", & self.alert_message_received()
        );
        defmt::write!(f, "patch_loaded: {=bool}, ", & self.patch_loaded());
        defmt::write!(f, "ready_f_211: {=bool}, ", & self.ready_f_211());
        defmt::write!(f, "boot_error: {=bool}, ", & self.boot_error());
        defmt::write!(
            f, "ready_for_data_block: {=bool}, ", & self.ready_for_data_block()
        );
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for IntEventBus1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for IntEventBus1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for IntEventBus1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for IntEventBus1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for IntEventBus1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for IntEventBus1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for IntEventBus1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Version {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for Version {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl Version {
    /// `31:0` - Read the `version` field.
    ///
    /// Boot FW version
    #[doc(alias = "Version")]
    #[must_use]
    pub fn version(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:0` - Set the `version` field.
    ///
    /// Boot FW version
    #[doc(alias = "Version")]
    pub fn set_version(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for Version {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for Version {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<Version> for [u8; 4] {
    fn from(val: Version) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Version");
        d.field("version", &self.version());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Version {{ ");
        defmt::write!(f, "version: {=u32}, ", & self.version());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Version {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Version {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Version {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Version {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Version {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Version {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Version {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Cmd1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for Cmd1 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl Cmd1 {
    /// `31:0` - Read the `command` field.
    ///
    /// Command value
    #[doc(alias = "Command")]
    #[must_use]
    pub fn command(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:0` - Set the `command` field.
    ///
    /// Command value
    #[doc(alias = "Command")]
    pub fn set_command(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for Cmd1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for Cmd1 {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<Cmd1> for [u8; 4] {
    fn from(val: Cmd1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Cmd1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Cmd1");
        d.field("command", &self.command());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmd1 {{ ");
        defmt::write!(f, "command: {=u32}, ", & self.command());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Cmd1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Cmd1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Cmd1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Cmd1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Cmd1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Cmd1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Cmd1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CustomerUse {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for CustomerUse {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl CustomerUse {
    /// `63:0` - Read the `customer_use` field.
    ///
    /// Controller operation mode
    #[doc(alias = "CustomerUse")]
    #[must_use]
    pub fn customer_use(&self) -> u64 {
        let start = 0;
        let end = 63;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u64,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `63:0` - Set the `customer_use` field.
    ///
    /// Controller operation mode
    #[doc(alias = "CustomerUse")]
    pub fn set_customer_use(&mut self, value: u64) {
        let start = 0;
        let end = 63;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u64,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for CustomerUse {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for CustomerUse {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<CustomerUse> for [u8; 8] {
    fn from(val: CustomerUse) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CustomerUse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CustomerUse");
        d.field("customer_use", &self.customer_use());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CustomerUse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CustomerUse {{ ");
        defmt::write!(f, "customer_use: {=u64}, ", & self.customer_use());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CustomerUse {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CustomerUse {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CustomerUse {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CustomerUse {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CustomerUse {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CustomerUse {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CustomerUse {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Mode {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for Mode {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl Mode {
    /// `31:0` - Read the `mode` field.
    ///
    /// Controller operation mode
    #[doc(alias = "Mode")]
    #[must_use]
    pub fn mode(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:0` - Set the `mode` field.
    ///
    /// Controller operation mode
    #[doc(alias = "Mode")]
    pub fn set_mode(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for Mode {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for Mode {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<Mode> for [u8; 4] {
    fn from(val: Mode) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Mode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Mode");
        d.field("mode", &self.mode());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mode {{ ");
        defmt::write!(f, "mode: {=u32}, ", & self.mode());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Mode {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Mode {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Mode {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Mode {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Mode {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Mode {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Mode {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TbtUsbDataPath {
    MayBeRequired = 0,
    NotRequired = 1,
}
impl core::convert::TryFrom<u8> for TbtUsbDataPath {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::MayBeRequired),
            1 => Ok(Self::NotRequired),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "TbtUsbDataPath",
                })
            }
        }
    }
}
impl From<TbtUsbDataPath> for u8 {
    fn from(val: TbtUsbDataPath) -> Self {
        match val {
            TbtUsbDataPath::MayBeRequired => 0,
            TbtUsbDataPath::NotRequired => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for TbtUsbDataPath {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpamVersion {
    Version2OrEarlier = 0,
    #[doc(alias = "Version2p1OrHigher")]
    Version2P1OrHigher = 1,
    Reserved(u8) = 2,
}
impl From<u8> for DpamVersion {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Version2OrEarlier,
            1 => Self::Version2P1OrHigher,
            val => Self::Reserved(val),
        }
    }
}
impl From<DpamVersion> for u8 {
    fn from(val: DpamVersion) -> Self {
        match val {
            DpamVersion::Version2OrEarlier => 0,
            DpamVersion::Version2P1OrHigher => 1,
            DpamVersion::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for DpamVersion {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveComponent {
    Passive = 0,
    Retimer = 1,
    Redriver = 2,
    Optical = 3,
}
impl core::convert::TryFrom<u8> for ActiveComponent {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Passive),
            1 => Ok(Self::Retimer),
            2 => Ok(Self::Redriver),
            3 => Ok(Self::Optical),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ActiveComponent",
                })
            }
        }
    }
}
impl From<ActiveComponent> for u8 {
    fn from(val: ActiveComponent) -> Self {
        match val {
            ActiveComponent::Passive => 0,
            ActiveComponent::Retimer => 1,
            ActiveComponent::Redriver => 2,
            ActiveComponent::Optical => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ActiveComponent {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2SignalingNotUsed {
    MayBeRequired = 0,
    NotNeededOnA6A7 = 1,
}
impl core::convert::TryFrom<u8> for Usb2SignalingNotUsed {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::MayBeRequired),
            1 => Ok(Self::NotNeededOnA6A7),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "Usb2SignalingNotUsed",
                })
            }
        }
    }
}
impl From<Usb2SignalingNotUsed> for u8 {
    fn from(val: Usb2SignalingNotUsed) -> Self {
        match val {
            Usb2SignalingNotUsed::MayBeRequired => 0,
            Usb2SignalingNotUsed::NotNeededOnA6A7 => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Usb2SignalingNotUsed {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReceptacleIndication {
    Plug = 0,
    Receptacle = 1,
}
impl core::convert::TryFrom<u8> for ReceptacleIndication {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Plug),
            1 => Ok(Self::Receptacle),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ReceptacleIndication",
                })
            }
        }
    }
}
impl From<ReceptacleIndication> for u8 {
    fn from(val: ReceptacleIndication) -> Self {
        match val {
            ReceptacleIndication::Plug => 0,
            ReceptacleIndication::Receptacle => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ReceptacleIndication {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortCapability {
    Reserved = 0,
    DpSink = 1,
    DpSource = 2,
    BothDpSourceAndSink = 3,
}
impl core::convert::TryFrom<u8> for PortCapability {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::DpSink),
            2 => Ok(Self::DpSource),
            3 => Ok(Self::BothDpSourceAndSink),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PortCapability",
                })
            }
        }
    }
}
impl From<PortCapability> for u8 {
    fn from(val: PortCapability) -> Self {
        match val {
            PortCapability::Reserved => 0,
            PortCapability::DpSink => 1,
            PortCapability::DpSource => 2,
            PortCapability::BothDpSourceAndSink => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PortCapability {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpVdoVersion {
    DpV20 = 0,
    DpV21 = 1,
    Reserved(u8) = 2,
}
impl From<u8> for DpVdoVersion {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::DpV20,
            1 => Self::DpV21,
            val => Self::Reserved(val),
        }
    }
}
impl From<DpVdoVersion> for u8 {
    fn from(val: DpVdoVersion) -> Self {
        match val {
            DpVdoVersion::DpV20 => 0,
            DpVdoVersion::DpV21 => 1,
            DpVdoVersion::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for DpVdoVersion {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DfpdUfpdConnected {
    Neither = 0,
    DfpD = 1,
    UfpD = 2,
    Both = 3,
}
impl core::convert::TryFrom<u8> for DfpdUfpdConnected {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Neither),
            1 => Ok(Self::DfpD),
            2 => Ok(Self::UfpD),
            3 => Ok(Self::Both),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "DfpdUfpdConnected",
                })
            }
        }
    }
}
impl From<DfpdUfpdConnected> for u8 {
    fn from(val: DfpdUfpdConnected) -> Self {
        match val {
            DfpdUfpdConnected::Neither => 0,
            DfpdUfpdConnected::DfpD => 1,
            DfpdUfpdConnected::UfpD => 2,
            DfpdUfpdConnected::Both => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for DfpdUfpdConnected {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpUsbDataPath {
    MayBeRequired = 0,
    NotRequired = 1,
}
impl core::convert::TryFrom<u8> for DpUsbDataPath {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::MayBeRequired),
            1 => Ok(Self::NotRequired),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "DpUsbDataPath",
                })
            }
        }
    }
}
impl From<DpUsbDataPath> for u8 {
    fn from(val: DpUsbDataPath) -> Self {
        match val {
            DpUsbDataPath::MayBeRequired => 0,
            DpUsbDataPath::NotRequired => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for DpUsbDataPath {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpTransportSignalling {
    Usb = 0,
    Dp = 1,
    Reserved(u8) = 2,
}
impl From<u8> for DpTransportSignalling {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Usb,
            1 => Self::Dp,
            val => Self::Reserved(val),
        }
    }
}
impl From<DpTransportSignalling> for u8 {
    fn from(val: DpTransportSignalling) -> Self {
        match val {
            DpTransportSignalling::Usb => 0,
            DpTransportSignalling::Dp => 1,
            DpTransportSignalling::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for DpTransportSignalling {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpPortCapability {
    Reserved = 0,
    UfpD = 1,
    DfpD = 2,
    Reserved2 = 3,
}
impl core::convert::TryFrom<u8> for DpPortCapability {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::UfpD),
            2 => Ok(Self::DfpD),
            3 => Ok(Self::Reserved2),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "DpPortCapability",
                })
            }
        }
    }
}
impl From<DpPortCapability> for u8 {
    fn from(val: DpPortCapability) -> Self {
        match val {
            DpPortCapability::Reserved => 0,
            DpPortCapability::UfpD => 1,
            DpPortCapability::DfpD => 2,
            DpPortCapability::Reserved2 => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for DpPortCapability {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdDataResetDetails {
    NoDataReset = 0,
    ReceivedFromPortPartner = 1,
    RequestedByHostDrst = 2,
    RequestedByHostDataControl = 3,
    ExitUsb4FollowingDrSwap = 4,
    Reserved(u8) = 5,
}
impl From<u8> for PdDataResetDetails {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoDataReset,
            1 => Self::ReceivedFromPortPartner,
            2 => Self::RequestedByHostDrst,
            3 => Self::RequestedByHostDataControl,
            4 => Self::ExitUsb4FollowingDrSwap,
            val => Self::Reserved(val),
        }
    }
}
impl From<PdDataResetDetails> for u8 {
    fn from(val: PdDataResetDetails) -> Self {
        match val {
            PdDataResetDetails::NoDataReset => 0,
            PdDataResetDetails::ReceivedFromPortPartner => 1,
            PdDataResetDetails::RequestedByHostDrst => 2,
            PdDataResetDetails::RequestedByHostDataControl => 3,
            PdDataResetDetails::ExitUsb4FollowingDrSwap => 4,
            PdDataResetDetails::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PdDataResetDetails {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdErrorRecoveryDetails {
    NoErrorRecovery = 0,
    OverTemperatureShutdown = 1,
    Pp5VLow = 2,
    FaultInputGpioAsserted = 3,
    OverVoltageOnPxVbus = 4,
    IlimOnPp5V = 6,
    IlimOnPpCable = 7,
    OvpOnCcDetected = 8,
    BackToNormalSystemPowerState = 9,
    InvalidDrSwap = 16,
    #[doc(alias = "PrSwapNoGoodCRC")]
    PrSwapNoGoodCrc = 17,
    #[doc(alias = "FrSwapNoGoodCRC")]
    FrSwapNoGoodCrc = 18,
    NoResponseTimeout = 21,
    PrSwapSourceOffTimer = 22,
    PrSwapSourceOnTimer = 23,
    FrSwapSourceOnTimer = 24,
    FrSwapTypeCSourceFailed = 25,
    FrSwapSenderResponseTimer = 26,
    FrSwapSourceOffTimer = 27,
    PolicyEngineErrorAttached = 28,
    PortConfig = 32,
    ErrorWithDataControl = 33,
    SwappingErrorDeadBattery = 34,
    HostUpdatedGlobalSystemConfig = 35,
    HostIssuedGaid = 36,
    HostIssuedDisc = 38,
    HostIssuedResetUcsi = 39,
    ErrorAttached = 48,
    VconnFailedToDischarge = 49,
    SystemPowerState = 50,
    HostDataControlUsbDisable = 51,
    SpmClientPortDisableChanged = 52,
    GpioEventTypecDisable = 53,
    CrOvp = 54,
    SbcOvp = 55,
    SbcRxOvp = 56,
    Reserved(u8) = 57,
}
impl From<u8> for PdErrorRecoveryDetails {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoErrorRecovery,
            1 => Self::OverTemperatureShutdown,
            2 => Self::Pp5VLow,
            3 => Self::FaultInputGpioAsserted,
            4 => Self::OverVoltageOnPxVbus,
            6 => Self::IlimOnPp5V,
            7 => Self::IlimOnPpCable,
            8 => Self::OvpOnCcDetected,
            9 => Self::BackToNormalSystemPowerState,
            16 => Self::InvalidDrSwap,
            17 => Self::PrSwapNoGoodCrc,
            18 => Self::FrSwapNoGoodCrc,
            21 => Self::NoResponseTimeout,
            22 => Self::PrSwapSourceOffTimer,
            23 => Self::PrSwapSourceOnTimer,
            24 => Self::FrSwapSourceOnTimer,
            25 => Self::FrSwapTypeCSourceFailed,
            26 => Self::FrSwapSenderResponseTimer,
            27 => Self::FrSwapSourceOffTimer,
            28 => Self::PolicyEngineErrorAttached,
            32 => Self::PortConfig,
            33 => Self::ErrorWithDataControl,
            34 => Self::SwappingErrorDeadBattery,
            35 => Self::HostUpdatedGlobalSystemConfig,
            36 => Self::HostIssuedGaid,
            38 => Self::HostIssuedDisc,
            39 => Self::HostIssuedResetUcsi,
            48 => Self::ErrorAttached,
            49 => Self::VconnFailedToDischarge,
            50 => Self::SystemPowerState,
            51 => Self::HostDataControlUsbDisable,
            52 => Self::SpmClientPortDisableChanged,
            53 => Self::GpioEventTypecDisable,
            54 => Self::CrOvp,
            55 => Self::SbcOvp,
            56 => Self::SbcRxOvp,
            val => Self::Reserved(val),
        }
    }
}
impl From<PdErrorRecoveryDetails> for u8 {
    fn from(val: PdErrorRecoveryDetails) -> Self {
        match val {
            PdErrorRecoveryDetails::NoErrorRecovery => 0,
            PdErrorRecoveryDetails::OverTemperatureShutdown => 1,
            PdErrorRecoveryDetails::Pp5VLow => 2,
            PdErrorRecoveryDetails::FaultInputGpioAsserted => 3,
            PdErrorRecoveryDetails::OverVoltageOnPxVbus => 4,
            PdErrorRecoveryDetails::IlimOnPp5V => 6,
            PdErrorRecoveryDetails::IlimOnPpCable => 7,
            PdErrorRecoveryDetails::OvpOnCcDetected => 8,
            PdErrorRecoveryDetails::BackToNormalSystemPowerState => 9,
            PdErrorRecoveryDetails::InvalidDrSwap => 16,
            PdErrorRecoveryDetails::PrSwapNoGoodCrc => 17,
            PdErrorRecoveryDetails::FrSwapNoGoodCrc => 18,
            PdErrorRecoveryDetails::NoResponseTimeout => 21,
            PdErrorRecoveryDetails::PrSwapSourceOffTimer => 22,
            PdErrorRecoveryDetails::PrSwapSourceOnTimer => 23,
            PdErrorRecoveryDetails::FrSwapSourceOnTimer => 24,
            PdErrorRecoveryDetails::FrSwapTypeCSourceFailed => 25,
            PdErrorRecoveryDetails::FrSwapSenderResponseTimer => 26,
            PdErrorRecoveryDetails::FrSwapSourceOffTimer => 27,
            PdErrorRecoveryDetails::PolicyEngineErrorAttached => 28,
            PdErrorRecoveryDetails::PortConfig => 32,
            PdErrorRecoveryDetails::ErrorWithDataControl => 33,
            PdErrorRecoveryDetails::SwappingErrorDeadBattery => 34,
            PdErrorRecoveryDetails::HostUpdatedGlobalSystemConfig => 35,
            PdErrorRecoveryDetails::HostIssuedGaid => 36,
            PdErrorRecoveryDetails::HostIssuedDisc => 38,
            PdErrorRecoveryDetails::HostIssuedResetUcsi => 39,
            PdErrorRecoveryDetails::ErrorAttached => 48,
            PdErrorRecoveryDetails::VconnFailedToDischarge => 49,
            PdErrorRecoveryDetails::SystemPowerState => 50,
            PdErrorRecoveryDetails::HostDataControlUsbDisable => 51,
            PdErrorRecoveryDetails::SpmClientPortDisableChanged => 52,
            PdErrorRecoveryDetails::GpioEventTypecDisable => 53,
            PdErrorRecoveryDetails::CrOvp => 54,
            PdErrorRecoveryDetails::SbcOvp => 55,
            PdErrorRecoveryDetails::SbcRxOvp => 56,
            PdErrorRecoveryDetails::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PdErrorRecoveryDetails {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdHardResetDetails {
    ResetValueNoHardReset = 0,
    ReceivedFromPortPartner = 1,
    RequestedByHost = 2,
    InvalidDrSwapRequest = 3,
    DischargeFailed = 4,
    NoResponseTimeout = 5,
    SendSoftReset = 6,
    SinkSelectCapability = 7,
    SinkTransitionSink = 8,
    SinkWaitForCapabilities = 9,
    SoftReset = 10,
    SourceOnTimeout = 11,
    SourceCapabilityResponse = 12,
    SourceSendCapabilities = 13,
    SourcingFault = 14,
    UnableToSource = 15,
    FrsFailure = 16,
    UnexpectedMessage = 17,
    VconnRecoverySequenceFailure = 18,
    Reserved(u8) = 19,
}
impl From<u8> for PdHardResetDetails {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::ResetValueNoHardReset,
            1 => Self::ReceivedFromPortPartner,
            2 => Self::RequestedByHost,
            3 => Self::InvalidDrSwapRequest,
            4 => Self::DischargeFailed,
            5 => Self::NoResponseTimeout,
            6 => Self::SendSoftReset,
            7 => Self::SinkSelectCapability,
            8 => Self::SinkTransitionSink,
            9 => Self::SinkWaitForCapabilities,
            10 => Self::SoftReset,
            11 => Self::SourceOnTimeout,
            12 => Self::SourceCapabilityResponse,
            13 => Self::SourceSendCapabilities,
            14 => Self::SourcingFault,
            15 => Self::UnableToSource,
            16 => Self::FrsFailure,
            17 => Self::UnexpectedMessage,
            18 => Self::VconnRecoverySequenceFailure,
            val => Self::Reserved(val),
        }
    }
}
impl From<PdHardResetDetails> for u8 {
    fn from(val: PdHardResetDetails) -> Self {
        match val {
            PdHardResetDetails::ResetValueNoHardReset => 0,
            PdHardResetDetails::ReceivedFromPortPartner => 1,
            PdHardResetDetails::RequestedByHost => 2,
            PdHardResetDetails::InvalidDrSwapRequest => 3,
            PdHardResetDetails::DischargeFailed => 4,
            PdHardResetDetails::NoResponseTimeout => 5,
            PdHardResetDetails::SendSoftReset => 6,
            PdHardResetDetails::SinkSelectCapability => 7,
            PdHardResetDetails::SinkTransitionSink => 8,
            PdHardResetDetails::SinkWaitForCapabilities => 9,
            PdHardResetDetails::SoftReset => 10,
            PdHardResetDetails::SourceOnTimeout => 11,
            PdHardResetDetails::SourceCapabilityResponse => 12,
            PdHardResetDetails::SourceSendCapabilities => 13,
            PdHardResetDetails::SourcingFault => 14,
            PdHardResetDetails::UnableToSource => 15,
            PdHardResetDetails::FrsFailure => 16,
            PdHardResetDetails::UnexpectedMessage => 17,
            PdHardResetDetails::VconnRecoverySequenceFailure => 18,
            PdHardResetDetails::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PdHardResetDetails {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdSoftResetDetails {
    NoSoftReset = 0,
    SoftResetReceived = 1,
    InvalidSourceCapabilities = 4,
    MessageRetriesExhausted = 5,
    UnexpectedAcceptMessage = 6,
    UnexpectedControlMessage = 7,
    UnexpectedGetSinkCapMessage = 8,
    UnexpectedGetSourceCapMessage = 9,
    UnexpectedGotoMinMessage = 10,
    UnexpectedPsrdyMessage = 11,
    UnexpectedPingMessage = 12,
    UnexpectedRejectMessage = 13,
    UnexpectedRequestMessage = 14,
    UnexpectedSinkCapabilitiesMessage = 15,
    UnexpectedSourceCapabilitiesMessage = 16,
    UnexpectedSwapMessage = 17,
    UnexpectedWaitCapabilitiesMessage = 18,
    UnknownControlMessage = 19,
    UnknownDataMessage = 20,
    InitializeSopController = 21,
    InitializeSopPrimeController = 22,
    UnexpectedExtendedMessage = 23,
    UnknownExtendedMessage = 24,
    UnexpectedDataMessage = 25,
    UnexpectedNotSupportedMessage = 26,
    UnexpectedGetStatusMessage = 27,
    Reserved(u8) = 28,
}
impl From<u8> for PdSoftResetDetails {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoSoftReset,
            1 => Self::SoftResetReceived,
            4 => Self::InvalidSourceCapabilities,
            5 => Self::MessageRetriesExhausted,
            6 => Self::UnexpectedAcceptMessage,
            7 => Self::UnexpectedControlMessage,
            8 => Self::UnexpectedGetSinkCapMessage,
            9 => Self::UnexpectedGetSourceCapMessage,
            10 => Self::UnexpectedGotoMinMessage,
            11 => Self::UnexpectedPsrdyMessage,
            12 => Self::UnexpectedPingMessage,
            13 => Self::UnexpectedRejectMessage,
            14 => Self::UnexpectedRequestMessage,
            15 => Self::UnexpectedSinkCapabilitiesMessage,
            16 => Self::UnexpectedSourceCapabilitiesMessage,
            17 => Self::UnexpectedSwapMessage,
            18 => Self::UnexpectedWaitCapabilitiesMessage,
            19 => Self::UnknownControlMessage,
            20 => Self::UnknownDataMessage,
            21 => Self::InitializeSopController,
            22 => Self::InitializeSopPrimeController,
            23 => Self::UnexpectedExtendedMessage,
            24 => Self::UnknownExtendedMessage,
            25 => Self::UnexpectedDataMessage,
            26 => Self::UnexpectedNotSupportedMessage,
            27 => Self::UnexpectedGetStatusMessage,
            val => Self::Reserved(val),
        }
    }
}
impl From<PdSoftResetDetails> for u8 {
    fn from(val: PdSoftResetDetails) -> Self {
        match val {
            PdSoftResetDetails::NoSoftReset => 0,
            PdSoftResetDetails::SoftResetReceived => 1,
            PdSoftResetDetails::InvalidSourceCapabilities => 4,
            PdSoftResetDetails::MessageRetriesExhausted => 5,
            PdSoftResetDetails::UnexpectedAcceptMessage => 6,
            PdSoftResetDetails::UnexpectedControlMessage => 7,
            PdSoftResetDetails::UnexpectedGetSinkCapMessage => 8,
            PdSoftResetDetails::UnexpectedGetSourceCapMessage => 9,
            PdSoftResetDetails::UnexpectedGotoMinMessage => 10,
            PdSoftResetDetails::UnexpectedPsrdyMessage => 11,
            PdSoftResetDetails::UnexpectedPingMessage => 12,
            PdSoftResetDetails::UnexpectedRejectMessage => 13,
            PdSoftResetDetails::UnexpectedRequestMessage => 14,
            PdSoftResetDetails::UnexpectedSinkCapabilitiesMessage => 15,
            PdSoftResetDetails::UnexpectedSourceCapabilitiesMessage => 16,
            PdSoftResetDetails::UnexpectedSwapMessage => 17,
            PdSoftResetDetails::UnexpectedWaitCapabilitiesMessage => 18,
            PdSoftResetDetails::UnknownControlMessage => 19,
            PdSoftResetDetails::UnknownDataMessage => 20,
            PdSoftResetDetails::InitializeSopController => 21,
            PdSoftResetDetails::InitializeSopPrimeController => 22,
            PdSoftResetDetails::UnexpectedExtendedMessage => 23,
            PdSoftResetDetails::UnknownExtendedMessage => 24,
            PdSoftResetDetails::UnexpectedDataMessage => 25,
            PdSoftResetDetails::UnexpectedNotSupportedMessage => 26,
            PdSoftResetDetails::UnexpectedGetStatusMessage => 27,
            PdSoftResetDetails::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PdSoftResetDetails {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdPortType {
    SinkSource = 0,
    Sink = 1,
    Source = 2,
    SourceSink = 3,
}
impl core::convert::TryFrom<u8> for PdPortType {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::SinkSource),
            1 => Ok(Self::Sink),
            2 => Ok(Self::Source),
            3 => Ok(Self::SourceSink),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PdPortType",
                })
            }
        }
    }
}
impl From<PdPortType> for u8 {
    fn from(val: PdPortType) -> Self {
        match val {
            PdPortType::SinkSource => 0,
            PdPortType::Sink => 1,
            PdPortType::Source => 2,
            PdPortType::SourceSink => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PdPortType {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdCcPullUp {
    NoPull = 0,
    UsbDefault = 1,
    Current1A5 = 2,
    Current3A0 = 3,
}
impl core::convert::TryFrom<u8> for PdCcPullUp {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoPull),
            1 => Ok(Self::UsbDefault),
            2 => Ok(Self::Current1A5),
            3 => Ok(Self::Current3A0),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PdCcPullUp",
                })
            }
        }
    }
}
impl From<PdCcPullUp> for u8 {
    fn from(val: PdCcPullUp) -> Self {
        match val {
            PdCcPullUp::NoPull => 0,
            PdCcPullUp::UsbDefault => 1,
            PdCcPullUp::Current1A5 => 2,
            PdCcPullUp::Current3A0 => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PdCcPullUp {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveDbgChannel {
    Dbg = 0,
    SbRxTx = 1,
    Aux = 2,
    Open = 3,
}
impl core::convert::TryFrom<u8> for ActiveDbgChannel {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Dbg),
            1 => Ok(Self::SbRxTx),
            2 => Ok(Self::Aux),
            3 => Ok(Self::Open),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ActiveDbgChannel",
                })
            }
        }
    }
}
impl From<ActiveDbgChannel> for u8 {
    fn from(val: ActiveDbgChannel) -> Self {
        match val {
            ActiveDbgChannel::Dbg => 0,
            ActiveDbgChannel::SbRxTx => 1,
            ActiveDbgChannel::Aux => 2,
            ActiveDbgChannel::Open => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ActiveDbgChannel {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VconnCurrentLimit {
    #[doc(alias = "Current410ma")]
    Current410Ma = 0,
    #[doc(alias = "Current590ma")]
    Current590Ma = 1,
    Other(u8) = 2,
}
impl From<u8> for VconnCurrentLimit {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Current410Ma,
            1 => Self::Current590Ma,
            val => Self::Other(val),
        }
    }
}
impl From<VconnCurrentLimit> for u8 {
    fn from(val: VconnCurrentLimit) -> Self {
        match val {
            VconnCurrentLimit::Current410Ma => 0,
            VconnCurrentLimit::Current590Ma => 1,
            VconnCurrentLimit::Other(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for VconnCurrentLimit {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TypecCurrent {
    UsbDefault = 0,
    Current1A5 = 1,
    Current3A0 = 2,
    Reserved(u8) = 3,
}
impl From<u8> for TypecCurrent {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UsbDefault,
            1 => Self::Current1A5,
            2 => Self::Current3A0,
            val => Self::Reserved(val),
        }
    }
}
impl From<TypecCurrent> for u8 {
    fn from(val: TypecCurrent) -> Self {
        match val {
            TypecCurrent::UsbDefault => 0,
            TypecCurrent::Current1A5 => 1,
            TypecCurrent::Current3A0 => 2,
            TypecCurrent::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for TypecCurrent {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbDefaultCurrent {
    UsbDefault = 0,
    #[doc(alias = "Current900ma")]
    Current900Ma = 1,
    #[doc(alias = "Current150ma")]
    Current150Ma = 2,
    Reserved(u8) = 3,
}
impl From<u8> for UsbDefaultCurrent {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UsbDefault,
            1 => Self::Current900Ma,
            2 => Self::Current150Ma,
            val => Self::Reserved(val),
        }
    }
}
impl From<UsbDefaultCurrent> for u8 {
    fn from(val: UsbDefaultCurrent) -> Self {
        match val {
            UsbDefaultCurrent::UsbDefault => 0,
            UsbDefaultCurrent::Current900Ma => 1,
            UsbDefaultCurrent::Current150Ma => 2,
            UsbDefaultCurrent::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for UsbDefaultCurrent {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "I2cTimeout")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2CTimeout {
    #[doc(alias = "Timeout25ms")]
    Timeout25Ms = 0,
    #[doc(alias = "Timeout50ms")]
    Timeout50Ms = 1,
    #[doc(alias = "Timeout75ms")]
    Timeout75Ms = 2,
    #[doc(alias = "Timeout100ms")]
    Timeout100Ms = 3,
    #[doc(alias = "Timeout125ms")]
    Timeout125Ms = 4,
    #[doc(alias = "Timeout150ms")]
    Timeout150Ms = 5,
    #[doc(alias = "Timeout175ms")]
    Timeout175Ms = 6,
    #[doc(alias = "Timeout1000ms")]
    Timeout1000Ms = 7,
}
impl core::convert::TryFrom<u8> for I2CTimeout {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Timeout25Ms),
            1 => Ok(Self::Timeout50Ms),
            2 => Ok(Self::Timeout75Ms),
            3 => Ok(Self::Timeout100Ms),
            4 => Ok(Self::Timeout125Ms),
            5 => Ok(Self::Timeout150Ms),
            6 => Ok(Self::Timeout175Ms),
            7 => Ok(Self::Timeout1000Ms),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "I2CTimeout",
                })
            }
        }
    }
}
impl From<I2CTimeout> for u8 {
    fn from(val: I2CTimeout) -> Self {
        match val {
            I2CTimeout::Timeout25Ms => 0,
            I2CTimeout::Timeout50Ms => 1,
            I2CTimeout::Timeout75Ms => 2,
            I2CTimeout::Timeout100Ms => 3,
            I2CTimeout::Timeout125Ms => 4,
            I2CTimeout::Timeout150Ms => 5,
            I2CTimeout::Timeout175Ms => 6,
            I2CTimeout::Timeout1000Ms => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for I2CTimeout {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiPortSinkNonOverlapTime {
    #[doc(alias = "Delay1ms")]
    Delay1Ms = 0,
    #[doc(alias = "Delay5ms")]
    Delay5Ms = 1,
    #[doc(alias = "Delay10ms")]
    Delay10Ms = 2,
    #[doc(alias = "Delay15ms")]
    Delay15Ms = 3,
}
impl core::convert::TryFrom<u8> for MultiPortSinkNonOverlapTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Delay1Ms),
            1 => Ok(Self::Delay5Ms),
            2 => Ok(Self::Delay10Ms),
            3 => Ok(Self::Delay15Ms),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "MultiPortSinkNonOverlapTime",
                })
            }
        }
    }
}
impl From<MultiPortSinkNonOverlapTime> for u8 {
    fn from(val: MultiPortSinkNonOverlapTime) -> Self {
        match val {
            MultiPortSinkNonOverlapTime::Delay1Ms => 0,
            MultiPortSinkNonOverlapTime::Delay5Ms => 1,
            MultiPortSinkNonOverlapTime::Delay10Ms => 2,
            MultiPortSinkNonOverlapTime::Delay15Ms => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for MultiPortSinkNonOverlapTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TbtControllerType {
    Default = 0,
    Ar = 1,
    Tr = 2,
    Icl = 3,
    Gr = 4,
    Br = 5,
    Reserved(u8) = 6,
}
impl From<u8> for TbtControllerType {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Default,
            1 => Self::Ar,
            2 => Self::Tr,
            3 => Self::Icl,
            4 => Self::Gr,
            5 => Self::Br,
            val => Self::Reserved(val),
        }
    }
}
impl From<TbtControllerType> for u8 {
    fn from(val: TbtControllerType) -> Self {
        match val {
            TbtControllerType::Default => 0,
            TbtControllerType::Ar => 1,
            TbtControllerType::Tr => 2,
            TbtControllerType::Icl => 3,
            TbtControllerType::Gr => 4,
            TbtControllerType::Br => 5,
            TbtControllerType::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for TbtControllerType {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcpThreshold {
    #[doc(alias = "Threshold6mv")]
    Threshold6Mv = 0,
    #[doc(alias = "Threshold8mv")]
    Threshold8Mv = 1,
    #[doc(alias = "Threshold10mv")]
    Threshold10Mv = 2,
    #[doc(alias = "Threshold12mv")]
    Threshold12Mv = 3,
}
impl core::convert::TryFrom<u8> for RcpThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Threshold6Mv),
            1 => Ok(Self::Threshold8Mv),
            2 => Ok(Self::Threshold10Mv),
            3 => Ok(Self::Threshold12Mv),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "RcpThreshold",
                })
            }
        }
    }
}
impl From<RcpThreshold> for u8 {
    fn from(val: RcpThreshold) -> Self {
        match val {
            RcpThreshold::Threshold6Mv => 0,
            RcpThreshold::Threshold8Mv => 1,
            RcpThreshold::Threshold10Mv => 2,
            RcpThreshold::Threshold12Mv => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for RcpThreshold {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PpextVbusSwConfig {
    Unused = 0,
    Source = 1,
    Sink = 2,
    SinkWaitSrdyNonDeadBattery = 3,
    BiDirectional = 4,
    BiDirectionalWaitSrdy = 5,
    SinkWaitSrdy = 6,
    BiDirectionalPpextDisabled = 7,
}
impl core::convert::TryFrom<u8> for PpextVbusSwConfig {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Unused),
            1 => Ok(Self::Source),
            2 => Ok(Self::Sink),
            3 => Ok(Self::SinkWaitSrdyNonDeadBattery),
            4 => Ok(Self::BiDirectional),
            5 => Ok(Self::BiDirectionalWaitSrdy),
            6 => Ok(Self::SinkWaitSrdy),
            7 => Ok(Self::BiDirectionalPpextDisabled),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PpextVbusSwConfig",
                })
            }
        }
    }
}
impl From<PpextVbusSwConfig> for u8 {
    fn from(val: PpextVbusSwConfig) -> Self {
        match val {
            PpextVbusSwConfig::Unused => 0,
            PpextVbusSwConfig::Source => 1,
            PpextVbusSwConfig::Sink => 2,
            PpextVbusSwConfig::SinkWaitSrdyNonDeadBattery => 3,
            PpextVbusSwConfig::BiDirectional => 4,
            PpextVbusSwConfig::BiDirectionalWaitSrdy => 5,
            PpextVbusSwConfig::SinkWaitSrdy => 6,
            PpextVbusSwConfig::BiDirectionalPpextDisabled => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PpextVbusSwConfig {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IlimOverShoot {
    NoOvershoot = 0,
    #[doc(alias = "Overshoot100ma")]
    Overshoot100Ma = 1,
    #[doc(alias = "Overshoot200ma")]
    Overshoot200Ma = 2,
    Reserved(u8) = 3,
}
impl From<u8> for IlimOverShoot {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoOvershoot,
            1 => Self::Overshoot100Ma,
            2 => Self::Overshoot200Ma,
            val => Self::Reserved(val),
        }
    }
}
impl From<IlimOverShoot> for u8 {
    fn from(val: IlimOverShoot) -> Self {
        match val {
            IlimOverShoot::NoOvershoot => 0,
            IlimOverShoot::Overshoot100Ma => 1,
            IlimOverShoot::Overshoot200Ma => 2,
            IlimOverShoot::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for IlimOverShoot {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusSwConfig {
    Disabled = 0,
    Source = 1,
    Reserved(u8) = 2,
}
impl From<u8> for VbusSwConfig {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Disabled,
            1 => Self::Source,
            val => Self::Reserved(val),
        }
    }
}
impl From<VbusSwConfig> for u8 {
    fn from(val: VbusSwConfig) -> Self {
        match val {
            VbusSwConfig::Disabled => 0,
            VbusSwConfig::Source => 1,
            VbusSwConfig::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for VbusSwConfig {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PpPowerSource {
    Vin = 1,
    Vbus = 2,
    Unknown(u8) = 3,
}
impl From<u8> for PpPowerSource {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::Vin,
            2 => Self::Vbus,
            val => Self::Unknown(val),
        }
    }
}
impl From<PpPowerSource> for u8 {
    fn from(val: PpPowerSource) -> Self {
        match val {
            PpPowerSource::Vin => 1,
            PpPowerSource::Vbus => 2,
            PpPowerSource::Unknown(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PpPowerSource {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PpExtVbusSw {
    Disabled = 0,
    DisabledFault = 1,
    EnabledInput = 3,
    Unknown(u8) = 4,
}
impl From<u8> for PpExtVbusSw {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Disabled,
            1 => Self::DisabledFault,
            3 => Self::EnabledInput,
            val => Self::Unknown(val),
        }
    }
}
impl From<PpExtVbusSw> for u8 {
    fn from(val: PpExtVbusSw) -> Self {
        match val {
            PpExtVbusSw::Disabled => 0,
            PpExtVbusSw::DisabledFault => 1,
            PpExtVbusSw::EnabledInput => 3,
            PpExtVbusSw::Unknown(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PpExtVbusSw {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PpIntVbusSw {
    Disabled = 0,
    DisabledFault = 1,
    EnabledOutput = 2,
    Unknown(u8) = 3,
}
impl From<u8> for PpIntVbusSw {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Disabled,
            1 => Self::DisabledFault,
            2 => Self::EnabledOutput,
            val => Self::Unknown(val),
        }
    }
}
impl From<PpIntVbusSw> for u8 {
    fn from(val: PpIntVbusSw) -> Self {
        match val {
            PpIntVbusSw::Disabled => 0,
            PpIntVbusSw::DisabledFault => 1,
            PpIntVbusSw::EnabledOutput => 2,
            PpIntVbusSw::Unknown(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PpIntVbusSw {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PpVconnSw {
    Disabled = 0,
    DisabledFault = 1,
    Cc1 = 2,
    Cc2 = 3,
}
impl core::convert::TryFrom<u8> for PpVconnSw {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::DisabledFault),
            2 => Ok(Self::Cc1),
            3 => Ok(Self::Cc2),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PpVconnSw",
                })
            }
        }
    }
}
impl From<PpVconnSw> for u8 {
    fn from(val: PpVconnSw) -> Self {
        match val {
            PpVconnSw::Disabled => 0,
            PpVconnSw::DisabledFault => 1,
            PpVconnSw::Cc1 => 2,
            PpVconnSw::Cc2 => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PpVconnSw {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb4RequiredPlugMode {
    None = 0,
    Reserved = 1,
    #[doc(alias = "USB4")]
    Usb4 = 2,
    #[doc(alias = "TBT3")]
    Tbt3 = 3,
}
impl core::convert::TryFrom<u8> for Usb4RequiredPlugMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::None),
            1 => Ok(Self::Reserved),
            2 => Ok(Self::Usb4),
            3 => Ok(Self::Tbt3),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "Usb4RequiredPlugMode",
                })
            }
        }
    }
}
impl From<Usb4RequiredPlugMode> for u8 {
    fn from(val: Usb4RequiredPlugMode) -> Self {
        match val {
            Usb4RequiredPlugMode::None => 0,
            Usb4RequiredPlugMode::Reserved => 1,
            Usb4RequiredPlugMode::Usb4 => 2,
            Usb4RequiredPlugMode::Tbt3 => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Usb4RequiredPlugMode {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EudoSopSentStatus {
    NoEnterUsb = 0,
    EnterUsbTimeout = 1,
    EnterUsbFailure = 2,
    SuccessfulEnterUsb = 3,
}
impl core::convert::TryFrom<u8> for EudoSopSentStatus {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoEnterUsb),
            1 => Ok(Self::EnterUsbTimeout),
            2 => Ok(Self::EnterUsbFailure),
            3 => Ok(Self::SuccessfulEnterUsb),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "EudoSopSentStatus",
                })
            }
        }
    }
}
impl From<EudoSopSentStatus> for u8 {
    fn from(val: EudoSopSentStatus) -> Self {
        match val {
            EudoSopSentStatus::NoEnterUsb => 0,
            EudoSopSentStatus::EnterUsbTimeout => 1,
            EudoSopSentStatus::EnterUsbFailure => 2,
            EudoSopSentStatus::SuccessfulEnterUsb => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for EudoSopSentStatus {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AmStatus {
    NoneAttempted = 0,
    EntrySuccessful = 1,
    EntryFailed = 2,
    PartialSuccess = 3,
}
impl core::convert::TryFrom<u8> for AmStatus {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoneAttempted),
            1 => Ok(Self::EntrySuccessful),
            2 => Ok(Self::EntryFailed),
            3 => Ok(Self::PartialSuccess),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "AmStatus",
                })
            }
        }
    }
}
impl From<AmStatus> for u8 {
    fn from(val: AmStatus) -> Self {
        match val {
            AmStatus::NoneAttempted => 0,
            AmStatus::EntrySuccessful => 1,
            AmStatus::EntryFailed => 2,
            AmStatus::PartialSuccess => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AmStatus {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LegacyMode {
    NoLegacy = 0,
    LegacySink = 1,
    LegacySource = 2,
    LegacySinkDeadBattery = 3,
}
impl core::convert::TryFrom<u8> for LegacyMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoLegacy),
            1 => Ok(Self::LegacySink),
            2 => Ok(Self::LegacySource),
            3 => Ok(Self::LegacySinkDeadBattery),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "LegacyMode",
                })
            }
        }
    }
}
impl From<LegacyMode> for u8 {
    fn from(val: LegacyMode) -> Self {
        match val {
            LegacyMode::NoLegacy => 0,
            LegacyMode::LegacySink => 1,
            LegacyMode::LegacySource => 2,
            LegacyMode::LegacySinkDeadBattery => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for LegacyMode {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHostMode {
    NoHost = 0,
    AttachedNoData = 1,
    AttachedNoPd = 2,
    HostPresent = 3,
}
impl core::convert::TryFrom<u8> for UsbHostMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoHost),
            1 => Ok(Self::AttachedNoData),
            2 => Ok(Self::AttachedNoPd),
            3 => Ok(Self::HostPresent),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "UsbHostMode",
                })
            }
        }
    }
}
impl From<UsbHostMode> for u8 {
    fn from(val: UsbHostMode) -> Self {
        match val {
            UsbHostMode::NoHost => 0,
            UsbHostMode::AttachedNoData => 1,
            UsbHostMode::AttachedNoPd => 2,
            UsbHostMode::HostPresent => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for UsbHostMode {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusMode {
    AtVsafe0 = 0,
    Atvsafe5 = 1,
    Normal = 2,
    Other = 3,
}
impl core::convert::TryFrom<u8> for VbusMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::AtVsafe0),
            1 => Ok(Self::Atvsafe5),
            2 => Ok(Self::Normal),
            3 => Ok(Self::Other),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "VbusMode",
                })
            }
        }
    }
}
impl From<VbusMode> for u8 {
    fn from(val: VbusMode) -> Self {
        match val {
            VbusMode::AtVsafe0 => 0,
            VbusMode::Atvsafe5 => 1,
            VbusMode::Normal => 2,
            VbusMode::Other => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for VbusMode {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlugMode {
    NotConnected = 0,
    Disabled = 1,
    Audio = 2,
    DebugRdRd = 3,
    RaDetected = 4,
    DebugRpRp = 5,
    ConnectedNoRa = 6,
    Connected = 7,
}
impl core::convert::TryFrom<u8> for PlugMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NotConnected),
            1 => Ok(Self::Disabled),
            2 => Ok(Self::Audio),
            3 => Ok(Self::DebugRdRd),
            4 => Ok(Self::RaDetected),
            5 => Ok(Self::DebugRpRp),
            6 => Ok(Self::ConnectedNoRa),
            7 => Ok(Self::Connected),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PlugMode",
                })
            }
        }
    }
}
impl From<PlugMode> for u8 {
    fn from(val: PlugMode) -> Self {
        match val {
            PlugMode::NotConnected => 0,
            PlugMode::Disabled => 1,
            PlugMode::Audio => 2,
            PlugMode::DebugRdRd => 3,
            PlugMode::RaDetected => 4,
            PlugMode::DebugRpRp => 5,
            PlugMode::ConnectedNoRa => 6,
            PlugMode::Connected => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PlugMode {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemPowerState {
    S0 = 0,
    S3 = 1,
    S4 = 2,
    S5 = 3,
    #[doc(alias = "S0ix")]
    S0Ix = 4,
    Reserved(u8) = 5,
}
impl From<u8> for SystemPowerState {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::S0,
            1 => Self::S3,
            2 => Self::S4,
            3 => Self::S5,
            4 => Self::S0Ix,
            val => Self::Reserved(val),
        }
    }
}
impl From<SystemPowerState> for u8 {
    fn from(val: SystemPowerState) -> Self {
        match val {
            SystemPowerState::S0 => 0,
            SystemPowerState::S3 => 1,
            SystemPowerState::S4 => 2,
            SystemPowerState::S5 => 3,
            SystemPowerState::S0Ix => 4,
            SystemPowerState::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for SystemPowerState {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
