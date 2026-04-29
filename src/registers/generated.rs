/// Root block of the Registers driver
#[derive(Debug)]
pub struct Registers<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u8,
}
impl<I> Registers<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self { interface, base_address: 0 }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.mode().read()?;
        callback(3 + 0 * 0, "mode", reg.into());
        let reg = self.customer_use().read()?;
        callback(6 + 0 * 0, "customer_use", reg.into());
        let reg = self.cmd_1().read()?;
        callback(8 + 0 * 0, "cmd_1", reg.into());
        let reg = self.version().read()?;
        callback(15 + 0 * 0, "version", reg.into());
        let reg = self.int_event_bus_1().read()?;
        callback(20 + 0 * 0, "int_event_bus_1", reg.into());
        let reg = self.int_mask_bus_1().read()?;
        callback(22 + 0 * 0, "int_mask_bus_1", reg.into());
        let reg = self.sx_app_config().read()?;
        callback(32 + 0 * 0, "sx_app_config", reg.into());
        let reg = self.int_clear_bus_1().read()?;
        callback(24 + 0 * 0, "int_clear_bus_1", reg.into());
        let reg = self.status().read()?;
        callback(26 + 0 * 0, "status", reg.into());
        let reg = self.usb_status().read()?;
        callback(36 + 0 * 0, "usb_status", reg.into());
        let reg = self.power_path_status().read()?;
        callback(38 + 0 * 0, "power_path_status", reg.into());
        let reg = self.system_config().read()?;
        callback(39 + 0 * 0, "system_config", reg.into());
        let reg = self.port_control().read()?;
        callback(41 + 0 * 0, "port_control", reg.into());
        let reg = self.active_pdo_contract().read()?;
        callback(52 + 0 * 0, "active_pdo_contract", reg.into());
        let reg = self.active_rdo_contract().read()?;
        callback(53 + 0 * 0, "active_rdo_contract", reg.into());
        let reg = self.pd_status().read()?;
        callback(64 + 0 * 0, "pd_status", reg.into());
        let reg = self.dp_config().read()?;
        callback(81 + 0 * 0, "dp_config", reg.into());
        let reg = self.tbt_config().read()?;
        callback(82 + 0 * 0, "tbt_config", reg.into());
        let reg = self.user_vid_status().read()?;
        callback(87 + 0 * 0, "user_vid_status", reg.into());
        let reg = self.intel_vid_status().read()?;
        callback(89 + 0 * 0, "intel_vid_status", reg.into());
        let reg = self.rx_attn_vdm().read()?;
        callback(96 + 0 * 0, "rx_attn_vdm", reg.into());
        let reg = self.rx_ado().read()?;
        callback(116 + 0 * 0, "rx_ado", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.mode().read_async().await?;
        callback(3 + 0 * 0, "mode", reg.into());
        let reg = self.customer_use().read_async().await?;
        callback(6 + 0 * 0, "customer_use", reg.into());
        let reg = self.cmd_1().read_async().await?;
        callback(8 + 0 * 0, "cmd_1", reg.into());
        let reg = self.version().read_async().await?;
        callback(15 + 0 * 0, "version", reg.into());
        let reg = self.int_event_bus_1().read_async().await?;
        callback(20 + 0 * 0, "int_event_bus_1", reg.into());
        let reg = self.int_mask_bus_1().read_async().await?;
        callback(22 + 0 * 0, "int_mask_bus_1", reg.into());
        let reg = self.sx_app_config().read_async().await?;
        callback(32 + 0 * 0, "sx_app_config", reg.into());
        let reg = self.int_clear_bus_1().read_async().await?;
        callback(24 + 0 * 0, "int_clear_bus_1", reg.into());
        let reg = self.status().read_async().await?;
        callback(26 + 0 * 0, "status", reg.into());
        let reg = self.usb_status().read_async().await?;
        callback(36 + 0 * 0, "usb_status", reg.into());
        let reg = self.power_path_status().read_async().await?;
        callback(38 + 0 * 0, "power_path_status", reg.into());
        let reg = self.system_config().read_async().await?;
        callback(39 + 0 * 0, "system_config", reg.into());
        let reg = self.port_control().read_async().await?;
        callback(41 + 0 * 0, "port_control", reg.into());
        let reg = self.active_pdo_contract().read_async().await?;
        callback(52 + 0 * 0, "active_pdo_contract", reg.into());
        let reg = self.active_rdo_contract().read_async().await?;
        callback(53 + 0 * 0, "active_rdo_contract", reg.into());
        let reg = self.pd_status().read_async().await?;
        callback(64 + 0 * 0, "pd_status", reg.into());
        let reg = self.dp_config().read_async().await?;
        callback(81 + 0 * 0, "dp_config", reg.into());
        let reg = self.tbt_config().read_async().await?;
        callback(82 + 0 * 0, "tbt_config", reg.into());
        let reg = self.user_vid_status().read_async().await?;
        callback(87 + 0 * 0, "user_vid_status", reg.into());
        let reg = self.intel_vid_status().read_async().await?;
        callback(89 + 0 * 0, "intel_vid_status", reg.into());
        let reg = self.rx_attn_vdm().read_async().await?;
        callback(96 + 0 * 0, "rx_attn_vdm", reg.into());
        let reg = self.rx_ado().read_async().await?;
        callback(116 + 0 * 0, "rx_ado", reg.into());
        Ok(())
    }
    /// Controller operation mode
    pub fn mode(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Mode,
        ::device_driver::RO,
    > {
        let address = self.base_address + 3;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Mode,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::Mode::new)
    }
    /// Customer use
    pub fn customer_use(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::CustomerUse,
        ::device_driver::RW,
    > {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::CustomerUse,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::CustomerUse::new)
    }
    /// Command 1 register
    pub fn cmd_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Cmd1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Cmd1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Cmd1::new)
    }
    /// Boot FW version
    pub fn version(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Version,
        ::device_driver::RO,
    > {
        let address = self.base_address + 15;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Version,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::Version::new)
    }
    /// Asserted interrupts for I2C1
    pub fn int_event_bus_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IntEventBus1,
        ::device_driver::RO,
    > {
        let address = self.base_address + 20;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IntEventBus1,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::IntEventBus1::new)
    }
    /// Masked interrupts for I2C1
    pub fn int_mask_bus_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IntEventBus1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 22;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IntEventBus1,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::IntEventBus1::new_as_int_mask_bus_1,
        )
    }
    /// Set Sx App Config - system power state for application configuration
    pub fn sx_app_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::SxAppConfig,
        ::device_driver::RW,
    > {
        let address = self.base_address + 32;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::SxAppConfig,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::SxAppConfig::new)
    }
    /// Interrupt clear for I2C1
    pub fn int_clear_bus_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IntEventBus1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 24;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IntEventBus1,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::IntEventBus1::new_as_int_clear_bus_1,
        )
    }
    /// Port status
    pub fn status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Status,
        ::device_driver::RO,
    > {
        let address = self.base_address + 26;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Status,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::Status::new)
    }
    /// Power path status
    pub fn usb_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::UsbStatus,
        ::device_driver::RO,
    > {
        let address = self.base_address + 36;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::UsbStatus,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::UsbStatus::new)
    }
    /// Power path status
    pub fn power_path_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::PowerPathStatus,
        ::device_driver::RO,
    > {
        let address = self.base_address + 38;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::PowerPathStatus,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::PowerPathStatus::new)
    }
    /// Global system configuration
    pub fn system_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::SystemConfig,
        ::device_driver::RW,
    > {
        let address = self.base_address + 39;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::SystemConfig,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::SystemConfig::new)
    }
    /// Port control
    pub fn port_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::PortControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 41;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::PortControl,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::PortControl::new)
    }
    /// Active PDO contract
    pub fn active_pdo_contract(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ActivePdoContract,
        ::device_driver::RO,
    > {
        let address = self.base_address + 52;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ActivePdoContract,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::ActivePdoContract::new)
    }
    /// Active PDO contract
    pub fn active_rdo_contract(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ActiveRdoContract,
        ::device_driver::RO,
    > {
        let address = self.base_address + 53;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ActiveRdoContract,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::ActiveRdoContract::new)
    }
    /// PD status
    pub fn pd_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::PdStatus,
        ::device_driver::RO,
    > {
        let address = self.base_address + 64;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::PdStatus,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::PdStatus::new)
    }
    /// Display Port Configuration
    pub fn dp_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DpConfig,
        ::device_driver::RW,
    > {
        let address = self.base_address + 81;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DpConfig,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::DpConfig::new)
    }
    pub fn tbt_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::TbtConfig,
        ::device_driver::RW,
    > {
        let address = self.base_address + 82;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::TbtConfig,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::TbtConfig::new)
    }
    pub fn user_vid_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::UserVidStatus,
        ::device_driver::RO,
    > {
        let address = self.base_address + 87;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::UserVidStatus,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::UserVidStatus::new)
    }
    /// Intel VID status
    pub fn intel_vid_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IntelVidStatus,
        ::device_driver::RO,
    > {
        let address = self.base_address + 89;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IntelVidStatus,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::IntelVidStatus::new)
    }
    /// Received User SVID Attention VDM
    pub fn rx_attn_vdm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::RxAttnVdm,
        ::device_driver::RO,
    > {
        let address = self.base_address + 96;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::RxAttnVdm,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::RxAttnVdm::new)
    }
    /// Received ADO
    pub fn rx_ado(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::RxAdo,
        ::device_driver::RO,
    > {
        let address = self.base_address + 116;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::RxAdo,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::RxAdo::new)
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    #[allow(unused_imports)]
    use super::*;
    /// Controller operation mode
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mode {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for Mode {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Mode {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `mode` field of the register.
        ///
        /// Controller operation mode
        pub fn mode(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 32)
            };
            raw
        }
        ///Write the `mode` field of the register.
        ///
        /// Controller operation mode
        pub fn set_mode(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 0, 32, &mut self.bits)
            };
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
    /// Customer use
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CustomerUse {
        /// The internal bits
        bits: [u8; 8],
    }
    impl ::device_driver::FieldSet for CustomerUse {
        const SIZE_BITS: u32 = 64;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CustomerUse {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 8] }
        }
        ///Read the `customer_use` field of the register.
        ///
        /// Controller operation mode
        pub fn customer_use(&self) -> u64 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u64,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 64)
            };
            raw
        }
        ///Write the `customer_use` field of the register.
        ///
        /// Controller operation mode
        pub fn set_customer_use(&mut self, value: u64) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u64,
                    ::device_driver::ops::LE,
                >(raw, 0, 64, &mut self.bits)
            };
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
    /// Command 1 register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd1 {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for Cmd1 {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Cmd1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `command` field of the register.
        ///
        /// Command value
        pub fn command(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 32)
            };
            raw
        }
        ///Write the `command` field of the register.
        ///
        /// Command value
        pub fn set_command(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 0, 32, &mut self.bits)
            };
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
    /// Boot FW version
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Version {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for Version {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Version {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `version` field of the register.
        ///
        /// Boot FW version
        pub fn version(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 32)
            };
            raw
        }
        ///Write the `version` field of the register.
        ///
        /// Boot FW version
        pub fn set_version(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 0, 32, &mut self.bits)
            };
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
    /// Asserted interrupts for I2C1
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEventBus1 {
        /// The internal bits
        bits: [u8; 11],
    }
    impl ::device_driver::FieldSet for IntEventBus1 {
        const SIZE_BITS: u32 = 88;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl IntEventBus1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [8, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 11] }
        }
        ///Create a new instance, loaded with the reset value of the `IntMaskBus1` ref
        pub const fn new_as_int_mask_bus_1() -> Self {
            Self {
                bits: [10, 56, 48, 205, 0, 0, 0, 15, 0, 0, 0],
            }
        }
        ///Create a new instance, loaded with the reset value of the `IntClearBus1` ref
        pub const fn new_as_int_clear_bus_1() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        ///Read the `hard_reset` field of the register.
        ///
        /// A PD hard reset has been performed
        pub fn hard_reset(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `plug_event` field of the register.
        ///
        /// A plug has been inserted or removed
        pub fn plug_event(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `power_swap_completed` field of the register.
        ///
        /// Power swap completed
        pub fn power_swap_completed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `data_swap_completed` field of the register.
        ///
        /// Data swap completed
        pub fn data_swap_completed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `fast_role_swap_completed` field of the register.
        ///
        /// Fast role swap completed
        pub fn fast_role_swap_completed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `source_cap_updated` field of the register.
        ///
        /// Source capabilities updated
        pub fn source_cap_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `sink_ready` field of the register.
        ///
        /// Asserts under an implicit contract or an explicit contract when PS_RDY has been received
        pub fn sink_ready(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 8, 9)
            };
            raw > 0
        }
        ///Read the `overcurrent` field of the register.
        ///
        /// Overcurrent
        pub fn overcurrent(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 9, 10)
            };
            raw > 0
        }
        ///Read the `attention_received` field of the register.
        ///
        /// Attention received
        pub fn attention_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 10, 11)
            };
            raw > 0
        }
        ///Read the `vdm_received` field of the register.
        ///
        /// VDM received
        pub fn vdm_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 11, 12)
            };
            raw > 0
        }
        ///Read the `new_consumer_contract` field of the register.
        ///
        /// New contract as consumer
        pub fn new_consumer_contract(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 12, 13)
            };
            raw > 0
        }
        ///Read the `new_provider_contract` field of the register.
        ///
        /// New contract as provider
        pub fn new_provider_contract(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 13, 14)
            };
            raw > 0
        }
        ///Read the `source_caps_received` field of the register.
        ///
        /// Source capabilities received
        pub fn source_caps_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 14, 15)
            };
            raw > 0
        }
        ///Read the `sink_caps_received` field of the register.
        ///
        /// Sink capabilities received
        pub fn sink_caps_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 15, 16)
            };
            raw > 0
        }
        ///Read the `power_swap_requested` field of the register.
        ///
        /// Power swap requested
        pub fn power_swap_requested(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 17, 18)
            };
            raw > 0
        }
        ///Read the `data_swap_requested` field of the register.
        ///
        /// Data swap requested
        pub fn data_swap_requested(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 18, 19)
            };
            raw > 0
        }
        ///Read the `usb_host_present` field of the register.
        ///
        /// USB host present
        pub fn usb_host_present(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 20, 21)
            };
            raw > 0
        }
        ///Read the `usb_host_not_present` field of the register.
        ///
        /// Set when USB host status transitions to anything other than present
        pub fn usb_host_not_present(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 21, 22)
            };
            raw > 0
        }
        ///Read the `power_path_switch_changed` field of the register.
        ///
        /// Power path status register changed
        pub fn power_path_switch_changed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 23, 24)
            };
            raw > 0
        }
        ///Read the `data_status_updated` field of the register.
        ///
        /// Data status register changed
        pub fn data_status_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 25, 26)
            };
            raw > 0
        }
        ///Read the `status_updated` field of the register.
        ///
        /// Status register changed
        pub fn status_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 26, 27)
            };
            raw > 0
        }
        ///Read the `pd_status_updated` field of the register.
        ///
        /// PD status register changed
        pub fn pd_status_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 27, 28)
            };
            raw > 0
        }
        ///Read the `cmd_1_completed` field of the register.
        ///
        /// Command 1 completed
        pub fn cmd_1_completed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 30, 31)
            };
            raw > 0
        }
        ///Read the `cmd_2_completed` field of the register.
        ///
        /// Command 2 completed
        pub fn cmd_2_completed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 31, 32)
            };
            raw > 0
        }
        ///Read the `device_incompatible` field of the register.
        ///
        /// Device lacks PD or has incompatible PD version
        pub fn device_incompatible(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 32, 33)
            };
            raw > 0
        }
        ///Read the `cannot_source` field of the register.
        ///
        /// Source cannot supply requested voltage or current
        pub fn cannot_source(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 33, 34)
            };
            raw > 0
        }
        ///Read the `can_source_later` field of the register.
        ///
        /// Source can supply requested voltage or current later
        pub fn can_source_later(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 34, 35)
            };
            raw > 0
        }
        ///Read the `power_event_error` field of the register.
        ///
        /// Voltage or current exceeded
        pub fn power_event_error(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 35, 36)
            };
            raw > 0
        }
        ///Read the `no_caps_response` field of the register.
        ///
        /// Device did not response to get caps message
        pub fn no_caps_response(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 36, 37)
            };
            raw > 0
        }
        ///Read the `protocol_error` field of the register.
        ///
        /// Unexpected message received from partner
        pub fn protocol_error(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 38, 39)
            };
            raw > 0
        }
        ///Read the `sink_transition_completed` field of the register.
        ///
        /// Sink transition completed
        pub fn sink_transition_completed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 42, 43)
            };
            raw > 0
        }
        ///Read the `plug_early_notification` field of the register.
        ///
        /// Plug connected but not debounced
        pub fn plug_early_notification(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 43, 44)
            };
            raw > 0
        }
        ///Read the `prochot_notification` field of the register.
        ///
        /// Prochot asserted
        pub fn prochot_notification(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 44, 45)
            };
            raw > 0
        }
        ///Read the `source_cannot_provide` field of the register.
        ///
        /// Source cannot produce negociated voltage or current
        pub fn source_cannot_provide(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 46, 47)
            };
            raw > 0
        }
        ///Read the `am_entry_fail` field of the register.
        ///
        /// Alternate mode entry failed
        pub fn am_entry_fail(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 48, 49)
            };
            raw > 0
        }
        ///Read the `am_entered` field of the register.
        ///
        /// Alternate mode entered
        pub fn am_entered(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 49, 50)
            };
            raw > 0
        }
        ///Read the `discover_mode_completed` field of the register.
        ///
        /// Discover modes process completed
        pub fn discover_mode_completed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 51, 52)
            };
            raw > 0
        }
        ///Read the `exit_mode_completed` field of the register.
        ///
        /// Exit mode process completed
        pub fn exit_mode_completed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 52, 53)
            };
            raw > 0
        }
        ///Read the `data_reset_started` field of the register.
        ///
        /// Data reset process started
        pub fn data_reset_started(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 53, 54)
            };
            raw > 0
        }
        ///Read the `usb_status_updated` field of the register.
        ///
        /// USB status updated
        pub fn usb_status_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 54, 55)
            };
            raw > 0
        }
        ///Read the `connection_manager_updated` field of the register.
        ///
        /// Connection manager updated
        pub fn connection_manager_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 55, 56)
            };
            raw > 0
        }
        ///Read the `usvid_mode_entered` field of the register.
        ///
        /// User VID alternate mode entered
        pub fn usvid_mode_entered(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 56, 57)
            };
            raw > 0
        }
        ///Read the `usvid_mode_exited` field of the register.
        ///
        /// User VID alternate mode entered
        pub fn usvid_mode_exited(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 57, 58)
            };
            raw > 0
        }
        ///Read the `usvid_attention_vdm_received` field of the register.
        ///
        /// User VID SVDM attention received
        pub fn usvid_attention_vdm_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 58, 59)
            };
            raw > 0
        }
        ///Read the `usvid_other_vdm_received` field of the register.
        ///
        /// User VID SVDM non-attention or unstructured VDM received
        pub fn usvid_other_vdm_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 59, 60)
            };
            raw > 0
        }
        ///Read the `external_dc_dc_event` field of the register.
        ///
        /// External DCDC event
        pub fn external_dc_dc_event(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 61, 62)
            };
            raw > 0
        }
        ///Read the `dp_sid_status_updated` field of the register.
        ///
        /// DP SID status register changed
        pub fn dp_sid_status_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 62, 63)
            };
            raw > 0
        }
        ///Read the `intel_vid_status_updated` field of the register.
        ///
        /// Intel VID status register changed
        pub fn intel_vid_status_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 63, 64)
            };
            raw > 0
        }
        ///Read the `pd_3_status_updated` field of the register.
        ///
        /// PD3 status register changed
        pub fn pd_3_status_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 64, 65)
            };
            raw > 0
        }
        ///Read the `tx_memory_buffer_empty` field of the register.
        ///
        /// TX memory buffer empty
        pub fn tx_memory_buffer_empty(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 65, 66)
            };
            raw > 0
        }
        ///Read the `mbrd_buffer_ready` field of the register.
        ///
        /// Buffer for mbrd command received and ready
        pub fn mbrd_buffer_ready(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 66, 67)
            };
            raw > 0
        }
        ///Read the `soc_ack_timeout` field of the register.
        ///
        /// SOC ack timeout
        pub fn soc_ack_timeout(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 70, 71)
            };
            raw > 0
        }
        ///Read the `not_supported_received` field of the register.
        ///
        /// Not supported PD message received
        pub fn not_supported_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 71, 72)
            };
            raw > 0
        }
        ///Read the `crossbar_error` field of the register.
        ///
        /// Error configuring the crossbar mux
        pub fn crossbar_error(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 72, 73)
            };
            raw > 0
        }
        ///Read the `mailbox_updated` field of the register.
        ///
        /// Mailbox updated
        pub fn mailbox_updated(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 73, 74)
            };
            raw > 0
        }
        ///Read the `bus_error` field of the register.
        ///
        /// I2C error communicating with external bus
        pub fn bus_error(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 74, 75)
            };
            raw > 0
        }
        ///Read the `external_dc_dc_status_changed` field of the register.
        ///
        /// External DCDC status changed
        pub fn external_dc_dc_status_changed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 75, 76)
            };
            raw > 0
        }
        ///Read the `frs_signal_received` field of the register.
        ///
        /// Fast role swap signal received
        pub fn frs_signal_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 76, 77)
            };
            raw > 0
        }
        ///Read the `chunk_response_received` field of the register.
        ///
        /// Chunk response received
        pub fn chunk_response_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 77, 78)
            };
            raw > 0
        }
        ///Read the `chunk_request_received` field of the register.
        ///
        /// Chunk request received
        pub fn chunk_request_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 78, 79)
            };
            raw > 0
        }
        ///Read the `alert_message_received` field of the register.
        ///
        /// Alert message received
        pub fn alert_message_received(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 79, 80)
            };
            raw > 0
        }
        ///Read the `patch_loaded` field of the register.
        ///
        /// Patch loaded to device
        pub fn patch_loaded(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 80, 81)
            };
            raw > 0
        }
        ///Read the `ready_f_211` field of the register.
        ///
        /// Ready for F211 image
        pub fn ready_f_211(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 81, 82)
            };
            raw > 0
        }
        ///Read the `boot_error` field of the register.
        ///
        /// Boot error
        pub fn boot_error(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 84, 85)
            };
            raw > 0
        }
        ///Read the `ready_for_data_block` field of the register.
        ///
        /// Ready for data block
        pub fn ready_for_data_block(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 85, 86)
            };
            raw > 0
        }
        ///Write the `hard_reset` field of the register.
        ///
        /// A PD hard reset has been performed
        pub fn set_hard_reset(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `plug_event` field of the register.
        ///
        /// A plug has been inserted or removed
        pub fn set_plug_event(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `power_swap_completed` field of the register.
        ///
        /// Power swap completed
        pub fn set_power_swap_completed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `data_swap_completed` field of the register.
        ///
        /// Data swap completed
        pub fn set_data_swap_completed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `fast_role_swap_completed` field of the register.
        ///
        /// Fast role swap completed
        pub fn set_fast_role_swap_completed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `source_cap_updated` field of the register.
        ///
        /// Source capabilities updated
        pub fn set_source_cap_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `sink_ready` field of the register.
        ///
        /// Asserts under an implicit contract or an explicit contract when PS_RDY has been received
        pub fn set_sink_ready(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 8, 9, &mut self.bits)
            };
        }
        ///Write the `overcurrent` field of the register.
        ///
        /// Overcurrent
        pub fn set_overcurrent(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 9, 10, &mut self.bits)
            };
        }
        ///Write the `attention_received` field of the register.
        ///
        /// Attention received
        pub fn set_attention_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 10, 11, &mut self.bits)
            };
        }
        ///Write the `vdm_received` field of the register.
        ///
        /// VDM received
        pub fn set_vdm_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 11, 12, &mut self.bits)
            };
        }
        ///Write the `new_consumer_contract` field of the register.
        ///
        /// New contract as consumer
        pub fn set_new_consumer_contract(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 12, 13, &mut self.bits)
            };
        }
        ///Write the `new_provider_contract` field of the register.
        ///
        /// New contract as provider
        pub fn set_new_provider_contract(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 13, 14, &mut self.bits)
            };
        }
        ///Write the `source_caps_received` field of the register.
        ///
        /// Source capabilities received
        pub fn set_source_caps_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 14, 15, &mut self.bits)
            };
        }
        ///Write the `sink_caps_received` field of the register.
        ///
        /// Sink capabilities received
        pub fn set_sink_caps_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 15, 16, &mut self.bits)
            };
        }
        ///Write the `power_swap_requested` field of the register.
        ///
        /// Power swap requested
        pub fn set_power_swap_requested(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 17, 18, &mut self.bits)
            };
        }
        ///Write the `data_swap_requested` field of the register.
        ///
        /// Data swap requested
        pub fn set_data_swap_requested(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 18, 19, &mut self.bits)
            };
        }
        ///Write the `usb_host_present` field of the register.
        ///
        /// USB host present
        pub fn set_usb_host_present(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 20, 21, &mut self.bits)
            };
        }
        ///Write the `usb_host_not_present` field of the register.
        ///
        /// Set when USB host status transitions to anything other than present
        pub fn set_usb_host_not_present(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 21, 22, &mut self.bits)
            };
        }
        ///Write the `power_path_switch_changed` field of the register.
        ///
        /// Power path status register changed
        pub fn set_power_path_switch_changed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 23, 24, &mut self.bits)
            };
        }
        ///Write the `data_status_updated` field of the register.
        ///
        /// Data status register changed
        pub fn set_data_status_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 25, 26, &mut self.bits)
            };
        }
        ///Write the `status_updated` field of the register.
        ///
        /// Status register changed
        pub fn set_status_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 26, 27, &mut self.bits)
            };
        }
        ///Write the `pd_status_updated` field of the register.
        ///
        /// PD status register changed
        pub fn set_pd_status_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 27, 28, &mut self.bits)
            };
        }
        ///Write the `cmd_1_completed` field of the register.
        ///
        /// Command 1 completed
        pub fn set_cmd_1_completed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 30, 31, &mut self.bits)
            };
        }
        ///Write the `cmd_2_completed` field of the register.
        ///
        /// Command 2 completed
        pub fn set_cmd_2_completed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 31, 32, &mut self.bits)
            };
        }
        ///Write the `device_incompatible` field of the register.
        ///
        /// Device lacks PD or has incompatible PD version
        pub fn set_device_incompatible(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 32, 33, &mut self.bits)
            };
        }
        ///Write the `cannot_source` field of the register.
        ///
        /// Source cannot supply requested voltage or current
        pub fn set_cannot_source(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 33, 34, &mut self.bits)
            };
        }
        ///Write the `can_source_later` field of the register.
        ///
        /// Source can supply requested voltage or current later
        pub fn set_can_source_later(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 34, 35, &mut self.bits)
            };
        }
        ///Write the `power_event_error` field of the register.
        ///
        /// Voltage or current exceeded
        pub fn set_power_event_error(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 35, 36, &mut self.bits)
            };
        }
        ///Write the `no_caps_response` field of the register.
        ///
        /// Device did not response to get caps message
        pub fn set_no_caps_response(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 36, 37, &mut self.bits)
            };
        }
        ///Write the `protocol_error` field of the register.
        ///
        /// Unexpected message received from partner
        pub fn set_protocol_error(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 38, 39, &mut self.bits)
            };
        }
        ///Write the `sink_transition_completed` field of the register.
        ///
        /// Sink transition completed
        pub fn set_sink_transition_completed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 42, 43, &mut self.bits)
            };
        }
        ///Write the `plug_early_notification` field of the register.
        ///
        /// Plug connected but not debounced
        pub fn set_plug_early_notification(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 43, 44, &mut self.bits)
            };
        }
        ///Write the `prochot_notification` field of the register.
        ///
        /// Prochot asserted
        pub fn set_prochot_notification(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 44, 45, &mut self.bits)
            };
        }
        ///Write the `source_cannot_provide` field of the register.
        ///
        /// Source cannot produce negociated voltage or current
        pub fn set_source_cannot_provide(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 46, 47, &mut self.bits)
            };
        }
        ///Write the `am_entry_fail` field of the register.
        ///
        /// Alternate mode entry failed
        pub fn set_am_entry_fail(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 48, 49, &mut self.bits)
            };
        }
        ///Write the `am_entered` field of the register.
        ///
        /// Alternate mode entered
        pub fn set_am_entered(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 49, 50, &mut self.bits)
            };
        }
        ///Write the `discover_mode_completed` field of the register.
        ///
        /// Discover modes process completed
        pub fn set_discover_mode_completed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 51, 52, &mut self.bits)
            };
        }
        ///Write the `exit_mode_completed` field of the register.
        ///
        /// Exit mode process completed
        pub fn set_exit_mode_completed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 52, 53, &mut self.bits)
            };
        }
        ///Write the `data_reset_started` field of the register.
        ///
        /// Data reset process started
        pub fn set_data_reset_started(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 53, 54, &mut self.bits)
            };
        }
        ///Write the `usb_status_updated` field of the register.
        ///
        /// USB status updated
        pub fn set_usb_status_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 54, 55, &mut self.bits)
            };
        }
        ///Write the `connection_manager_updated` field of the register.
        ///
        /// Connection manager updated
        pub fn set_connection_manager_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 55, 56, &mut self.bits)
            };
        }
        ///Write the `usvid_mode_entered` field of the register.
        ///
        /// User VID alternate mode entered
        pub fn set_usvid_mode_entered(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 56, 57, &mut self.bits)
            };
        }
        ///Write the `usvid_mode_exited` field of the register.
        ///
        /// User VID alternate mode entered
        pub fn set_usvid_mode_exited(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 57, 58, &mut self.bits)
            };
        }
        ///Write the `usvid_attention_vdm_received` field of the register.
        ///
        /// User VID SVDM attention received
        pub fn set_usvid_attention_vdm_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 58, 59, &mut self.bits)
            };
        }
        ///Write the `usvid_other_vdm_received` field of the register.
        ///
        /// User VID SVDM non-attention or unstructured VDM received
        pub fn set_usvid_other_vdm_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 59, 60, &mut self.bits)
            };
        }
        ///Write the `external_dc_dc_event` field of the register.
        ///
        /// External DCDC event
        pub fn set_external_dc_dc_event(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 61, 62, &mut self.bits)
            };
        }
        ///Write the `dp_sid_status_updated` field of the register.
        ///
        /// DP SID status register changed
        pub fn set_dp_sid_status_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 62, 63, &mut self.bits)
            };
        }
        ///Write the `intel_vid_status_updated` field of the register.
        ///
        /// Intel VID status register changed
        pub fn set_intel_vid_status_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 63, 64, &mut self.bits)
            };
        }
        ///Write the `pd_3_status_updated` field of the register.
        ///
        /// PD3 status register changed
        pub fn set_pd_3_status_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 64, 65, &mut self.bits)
            };
        }
        ///Write the `tx_memory_buffer_empty` field of the register.
        ///
        /// TX memory buffer empty
        pub fn set_tx_memory_buffer_empty(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 65, 66, &mut self.bits)
            };
        }
        ///Write the `mbrd_buffer_ready` field of the register.
        ///
        /// Buffer for mbrd command received and ready
        pub fn set_mbrd_buffer_ready(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 66, 67, &mut self.bits)
            };
        }
        ///Write the `soc_ack_timeout` field of the register.
        ///
        /// SOC ack timeout
        pub fn set_soc_ack_timeout(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 70, 71, &mut self.bits)
            };
        }
        ///Write the `not_supported_received` field of the register.
        ///
        /// Not supported PD message received
        pub fn set_not_supported_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 71, 72, &mut self.bits)
            };
        }
        ///Write the `crossbar_error` field of the register.
        ///
        /// Error configuring the crossbar mux
        pub fn set_crossbar_error(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 72, 73, &mut self.bits)
            };
        }
        ///Write the `mailbox_updated` field of the register.
        ///
        /// Mailbox updated
        pub fn set_mailbox_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 73, 74, &mut self.bits)
            };
        }
        ///Write the `bus_error` field of the register.
        ///
        /// I2C error communicating with external bus
        pub fn set_bus_error(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 74, 75, &mut self.bits)
            };
        }
        ///Write the `external_dc_dc_status_changed` field of the register.
        ///
        /// External DCDC status changed
        pub fn set_external_dc_dc_status_changed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 75, 76, &mut self.bits)
            };
        }
        ///Write the `frs_signal_received` field of the register.
        ///
        /// Fast role swap signal received
        pub fn set_frs_signal_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 76, 77, &mut self.bits)
            };
        }
        ///Write the `chunk_response_received` field of the register.
        ///
        /// Chunk response received
        pub fn set_chunk_response_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 77, 78, &mut self.bits)
            };
        }
        ///Write the `chunk_request_received` field of the register.
        ///
        /// Chunk request received
        pub fn set_chunk_request_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 78, 79, &mut self.bits)
            };
        }
        ///Write the `alert_message_received` field of the register.
        ///
        /// Alert message received
        pub fn set_alert_message_received(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 79, 80, &mut self.bits)
            };
        }
        ///Write the `patch_loaded` field of the register.
        ///
        /// Patch loaded to device
        pub fn set_patch_loaded(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 80, 81, &mut self.bits)
            };
        }
        ///Write the `ready_f_211` field of the register.
        ///
        /// Ready for F211 image
        pub fn set_ready_f_211(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 81, 82, &mut self.bits)
            };
        }
        ///Write the `boot_error` field of the register.
        ///
        /// Boot error
        pub fn set_boot_error(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 84, 85, &mut self.bits)
            };
        }
        ///Write the `ready_for_data_block` field of the register.
        ///
        /// Ready for data block
        pub fn set_ready_for_data_block(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 85, 86, &mut self.bits)
            };
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
            d.field(
                "usvid_attention_vdm_received",
                &self.usvid_attention_vdm_received(),
            );
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
            d.field(
                "external_dc_dc_status_changed",
                &self.external_dc_dc_status_changed(),
            );
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
            defmt::write!(
                f, "data_swap_completed: {=bool}, ", & self.data_swap_completed()
            );
            defmt::write!(
                f, "fast_role_swap_completed: {=bool}, ", & self
                .fast_role_swap_completed()
            );
            defmt::write!(
                f, "source_cap_updated: {=bool}, ", & self.source_cap_updated()
            );
            defmt::write!(f, "sink_ready: {=bool}, ", & self.sink_ready());
            defmt::write!(f, "overcurrent: {=bool}, ", & self.overcurrent());
            defmt::write!(
                f, "attention_received: {=bool}, ", & self.attention_received()
            );
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
            defmt::write!(
                f, "sink_caps_received: {=bool}, ", & self.sink_caps_received()
            );
            defmt::write!(
                f, "power_swap_requested: {=bool}, ", & self.power_swap_requested()
            );
            defmt::write!(
                f, "data_swap_requested: {=bool}, ", & self.data_swap_requested()
            );
            defmt::write!(f, "usb_host_present: {=bool}, ", & self.usb_host_present());
            defmt::write!(
                f, "usb_host_not_present: {=bool}, ", & self.usb_host_not_present()
            );
            defmt::write!(
                f, "power_path_switch_changed: {=bool}, ", & self
                .power_path_switch_changed()
            );
            defmt::write!(
                f, "data_status_updated: {=bool}, ", & self.data_status_updated()
            );
            defmt::write!(f, "status_updated: {=bool}, ", & self.status_updated());
            defmt::write!(f, "pd_status_updated: {=bool}, ", & self.pd_status_updated());
            defmt::write!(f, "cmd_1_completed: {=bool}, ", & self.cmd_1_completed());
            defmt::write!(f, "cmd_2_completed: {=bool}, ", & self.cmd_2_completed());
            defmt::write!(
                f, "device_incompatible: {=bool}, ", & self.device_incompatible()
            );
            defmt::write!(f, "cannot_source: {=bool}, ", & self.cannot_source());
            defmt::write!(f, "can_source_later: {=bool}, ", & self.can_source_later());
            defmt::write!(f, "power_event_error: {=bool}, ", & self.power_event_error());
            defmt::write!(f, "no_caps_response: {=bool}, ", & self.no_caps_response());
            defmt::write!(f, "protocol_error: {=bool}, ", & self.protocol_error());
            defmt::write!(
                f, "sink_transition_completed: {=bool}, ", & self
                .sink_transition_completed()
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
            defmt::write!(
                f, "exit_mode_completed: {=bool}, ", & self.exit_mode_completed()
            );
            defmt::write!(
                f, "data_reset_started: {=bool}, ", & self.data_reset_started()
            );
            defmt::write!(
                f, "usb_status_updated: {=bool}, ", & self.usb_status_updated()
            );
            defmt::write!(
                f, "connection_manager_updated: {=bool}, ", & self
                .connection_manager_updated()
            );
            defmt::write!(
                f, "usvid_mode_entered: {=bool}, ", & self.usvid_mode_entered()
            );
            defmt::write!(f, "usvid_mode_exited: {=bool}, ", & self.usvid_mode_exited());
            defmt::write!(
                f, "usvid_attention_vdm_received: {=bool}, ", & self
                .usvid_attention_vdm_received()
            );
            defmt::write!(
                f, "usvid_other_vdm_received: {=bool}, ", & self
                .usvid_other_vdm_received()
            );
            defmt::write!(
                f, "external_dc_dc_event: {=bool}, ", & self.external_dc_dc_event()
            );
            defmt::write!(
                f, "dp_sid_status_updated: {=bool}, ", & self.dp_sid_status_updated()
            );
            defmt::write!(
                f, "intel_vid_status_updated: {=bool}, ", & self
                .intel_vid_status_updated()
            );
            defmt::write!(
                f, "pd_3_status_updated: {=bool}, ", & self.pd_3_status_updated()
            );
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
            defmt::write!(
                f, "frs_signal_received: {=bool}, ", & self.frs_signal_received()
            );
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
    /// Set Sx App Config - system power state for application configuration
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SxAppConfig {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for SxAppConfig {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl SxAppConfig {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `sleep_state` field of the register.
        ///
        /// Current system power state
        pub fn sleep_state(&self) -> super::SystemPowerState {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `sleep_state` field of the register.
        ///
        /// Current system power state
        pub fn set_sleep_state(&mut self, value: super::SystemPowerState) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 3, &mut self.bits)
            };
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
    /// Port status
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status {
        /// The internal bits
        bits: [u8; 5],
    }
    impl ::device_driver::FieldSet for Status {
        const SIZE_BITS: u32 = 40;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Status {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 5] }
        }
        ///Read the `plug_present` field of the register.
        ///
        /// Plug present
        pub fn plug_present(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Read the `connection_state` field of the register.
        ///
        /// Connection state
        pub fn connection_state(&self) -> super::PlugMode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 1, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `plug_orientation` field of the register.
        ///
        /// Connector oreintation, 0 for normal, 1 for flipped
        pub fn plug_orientation(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `port_role` field of the register.
        ///
        /// PD role, 0 for sink, 1 for source
        pub fn port_role(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `data_role` field of the register.
        ///
        /// Data role, 0 for UFP, 1 for DFP
        pub fn data_role(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `erp_mode` field of the register.
        ///
        /// Is EPR mode active
        pub fn erp_mode(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `vbus_status` field of the register.
        ///
        /// Vbus status
        pub fn vbus_status(&self) -> super::VbusMode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 20, 22)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `usb_host` field of the register.
        ///
        /// USB host mode
        pub fn usb_host(&self) -> super::UsbHostMode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 22, 24)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `legacy` field of the register.
        ///
        /// Legacy mode stotus
        pub fn legacy(&self) -> super::LegacyMode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 24, 26)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `bist_in_progress` field of the register.
        ///
        /// If a BIST is in progress
        pub fn bist_in_progress(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 27, 28)
            };
            raw > 0
        }
        ///Read the `soc_ack_timeout` field of the register.
        ///
        /// Set when the SOC acknolwedgement has timed out
        pub fn soc_ack_timeout(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 30, 31)
            };
            raw > 0
        }
        ///Read the `am_status` field of the register.
        ///
        /// Alternate mode entry status
        pub fn am_status(&self) -> super::AmStatus {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 32, 34)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `plug_present` field of the register.
        ///
        /// Plug present
        pub fn set_plug_present(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
        ///Write the `connection_state` field of the register.
        ///
        /// Connection state
        pub fn set_connection_state(&mut self, value: super::PlugMode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 1, 4, &mut self.bits)
            };
        }
        ///Write the `plug_orientation` field of the register.
        ///
        /// Connector oreintation, 0 for normal, 1 for flipped
        pub fn set_plug_orientation(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `port_role` field of the register.
        ///
        /// PD role, 0 for sink, 1 for source
        pub fn set_port_role(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `data_role` field of the register.
        ///
        /// Data role, 0 for UFP, 1 for DFP
        pub fn set_data_role(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `erp_mode` field of the register.
        ///
        /// Is EPR mode active
        pub fn set_erp_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `vbus_status` field of the register.
        ///
        /// Vbus status
        pub fn set_vbus_status(&mut self, value: super::VbusMode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 20, 22, &mut self.bits)
            };
        }
        ///Write the `usb_host` field of the register.
        ///
        /// USB host mode
        pub fn set_usb_host(&mut self, value: super::UsbHostMode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 22, 24, &mut self.bits)
            };
        }
        ///Write the `legacy` field of the register.
        ///
        /// Legacy mode stotus
        pub fn set_legacy(&mut self, value: super::LegacyMode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 24, 26, &mut self.bits)
            };
        }
        ///Write the `bist_in_progress` field of the register.
        ///
        /// If a BIST is in progress
        pub fn set_bist_in_progress(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 27, 28, &mut self.bits)
            };
        }
        ///Write the `soc_ack_timeout` field of the register.
        ///
        /// Set when the SOC acknolwedgement has timed out
        pub fn set_soc_ack_timeout(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 30, 31, &mut self.bits)
            };
        }
        ///Write the `am_status` field of the register.
        ///
        /// Alternate mode entry status
        pub fn set_am_status(&mut self, value: super::AmStatus) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 32, 34, &mut self.bits)
            };
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
    /// Power path status
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbStatus {
        /// The internal bits
        bits: [u8; 9],
    }
    impl ::device_driver::FieldSet for UsbStatus {
        const SIZE_BITS: u32 = 72;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl UsbStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 9] }
        }
        ///Read the `eudo_sop_sent_status` field of the register.
        ///
        /// Enter USB4 mode status
        pub fn eudo_sop_sent_status(&self) -> super::EudoSopSentStatus {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `usb_4_required_plug_mode` field of the register.
        ///
        /// USB4 plug mode requirement
        pub fn usb_4_required_plug_mode(&self) -> super::Usb4RequiredPlugMode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `usb_mode_active_on_plug` field of the register.
        ///
        /// USB4 mode active on plug
        pub fn usb_mode_active_on_plug(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `vpro_entry_failed` field of the register.
        ///
        /// vPro mode error. This bit is asserted ifa n error occurred while trying to enter the vPro mode.
        pub fn vpro_entry_failed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `usb_reentry_needed` field of the register.
        ///
        /// USB re-entry is needed
        pub fn usb_reentry_needed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `enter_usb_data_object` field of the register.
        ///
        /// Enter_USB Data Object (EUDO)
        pub fn enter_usb_data_object(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 8, 40)
            };
            raw
        }
        ///Read the `tbt_enter_mode_vdo` field of the register.
        ///
        /// vPro mode VDO.
        pub fn tbt_enter_mode_vdo(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 40, 72)
            };
            raw
        }
        ///Write the `eudo_sop_sent_status` field of the register.
        ///
        /// Enter USB4 mode status
        pub fn set_eudo_sop_sent_status(&mut self, value: super::EudoSopSentStatus) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
        ///Write the `usb_4_required_plug_mode` field of the register.
        ///
        /// USB4 plug mode requirement
        pub fn set_usb_4_required_plug_mode(
            &mut self,
            value: super::Usb4RequiredPlugMode,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 4, &mut self.bits)
            };
        }
        ///Write the `usb_mode_active_on_plug` field of the register.
        ///
        /// USB4 mode active on plug
        pub fn set_usb_mode_active_on_plug(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `vpro_entry_failed` field of the register.
        ///
        /// vPro mode error. This bit is asserted ifa n error occurred while trying to enter the vPro mode.
        pub fn set_vpro_entry_failed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `usb_reentry_needed` field of the register.
        ///
        /// USB re-entry is needed
        pub fn set_usb_reentry_needed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `enter_usb_data_object` field of the register.
        ///
        /// Enter_USB Data Object (EUDO)
        pub fn set_enter_usb_data_object(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 8, 40, &mut self.bits)
            };
        }
        ///Write the `tbt_enter_mode_vdo` field of the register.
        ///
        /// vPro mode VDO.
        pub fn set_tbt_enter_mode_vdo(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 40, 72, &mut self.bits)
            };
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
            defmt::write!(
                f, "eudo_sop_sent_status: {}, ", & self.eudo_sop_sent_status()
            );
            defmt::write!(
                f, "usb_4_required_plug_mode: {}, ", & self.usb_4_required_plug_mode()
            );
            defmt::write!(
                f, "usb_mode_active_on_plug: {=bool}, ", & self.usb_mode_active_on_plug()
            );
            defmt::write!(f, "vpro_entry_failed: {=bool}, ", & self.vpro_entry_failed());
            defmt::write!(
                f, "usb_reentry_needed: {=bool}, ", & self.usb_reentry_needed()
            );
            defmt::write!(
                f, "enter_usb_data_object: {=u32}, ", & self.enter_usb_data_object()
            );
            defmt::write!(
                f, "tbt_enter_mode_vdo: {=u32}, ", & self.tbt_enter_mode_vdo()
            );
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
    /// Power path status
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerPathStatus {
        /// The internal bits
        bits: [u8; 5],
    }
    impl ::device_driver::FieldSet for PowerPathStatus {
        const SIZE_BITS: u32 = 40;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PowerPathStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 5] }
        }
        ///Read the `pa_vconn_sw` field of the register.
        ///
        /// PA Vconn switch status
        pub fn pa_vconn_sw(&self) -> super::PpVconnSw {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pb_vconn_sw` field of the register.
        ///
        /// PA Vconn switch status
        pub fn pb_vconn_sw(&self) -> super::PpVconnSw {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pa_int_vbus_sw` field of the register.
        ///
        /// PA int vbus switch status
        pub fn pa_int_vbus_sw(&self) -> super::PpIntVbusSw {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 6, 9)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pb_int_vbus_sw` field of the register.
        ///
        /// PB int vbus switch status
        pub fn pb_int_vbus_sw(&self) -> super::PpIntVbusSw {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 9, 12)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pa_ext_vbus_sw` field of the register.
        ///
        /// PA ext vbus switch status
        pub fn pa_ext_vbus_sw(&self) -> super::PpExtVbusSw {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 12, 15)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pb_ext_vbus_sw` field of the register.
        ///
        /// PB ext vbus switch status
        pub fn pb_ext_vbus_sw(&self) -> super::PpExtVbusSw {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 15, 18)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pa_int_vbus_oc` field of the register.
        ///
        /// PA int vbus overcurrent
        pub fn pa_int_vbus_oc(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 28, 29)
            };
            raw > 0
        }
        ///Read the `pb_int_vbus_oc` field of the register.
        ///
        /// PB int vbus overcurrent
        pub fn pb_int_vbus_oc(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 29, 30)
            };
            raw > 0
        }
        ///Read the `pa_vconn_oc` field of the register.
        ///
        /// PA vconn overcurrent
        pub fn pa_vconn_oc(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 34, 35)
            };
            raw > 0
        }
        ///Read the `pb_vconn_oc` field of the register.
        ///
        /// PB vconn overcurrent
        pub fn pb_vconn_oc(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 35, 36)
            };
            raw > 0
        }
        ///Read the `power_source` field of the register.
        ///
        /// How the PD controller is powered
        pub fn power_source(&self) -> super::PpPowerSource {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 38, 40)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `pa_vconn_sw` field of the register.
        ///
        /// PA Vconn switch status
        pub fn set_pa_vconn_sw(&mut self, value: super::PpVconnSw) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
        ///Write the `pb_vconn_sw` field of the register.
        ///
        /// PA Vconn switch status
        pub fn set_pb_vconn_sw(&mut self, value: super::PpVconnSw) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 4, &mut self.bits)
            };
        }
        ///Write the `pa_int_vbus_sw` field of the register.
        ///
        /// PA int vbus switch status
        pub fn set_pa_int_vbus_sw(&mut self, value: super::PpIntVbusSw) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 6, 9, &mut self.bits)
            };
        }
        ///Write the `pb_int_vbus_sw` field of the register.
        ///
        /// PB int vbus switch status
        pub fn set_pb_int_vbus_sw(&mut self, value: super::PpIntVbusSw) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 9, 12, &mut self.bits)
            };
        }
        ///Write the `pa_ext_vbus_sw` field of the register.
        ///
        /// PA ext vbus switch status
        pub fn set_pa_ext_vbus_sw(&mut self, value: super::PpExtVbusSw) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 12, 15, &mut self.bits)
            };
        }
        ///Write the `pb_ext_vbus_sw` field of the register.
        ///
        /// PB ext vbus switch status
        pub fn set_pb_ext_vbus_sw(&mut self, value: super::PpExtVbusSw) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 15, 18, &mut self.bits)
            };
        }
        ///Write the `pa_int_vbus_oc` field of the register.
        ///
        /// PA int vbus overcurrent
        pub fn set_pa_int_vbus_oc(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 28, 29, &mut self.bits)
            };
        }
        ///Write the `pb_int_vbus_oc` field of the register.
        ///
        /// PB int vbus overcurrent
        pub fn set_pb_int_vbus_oc(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 29, 30, &mut self.bits)
            };
        }
        ///Write the `pa_vconn_oc` field of the register.
        ///
        /// PA vconn overcurrent
        pub fn set_pa_vconn_oc(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 34, 35, &mut self.bits)
            };
        }
        ///Write the `pb_vconn_oc` field of the register.
        ///
        /// PB vconn overcurrent
        pub fn set_pb_vconn_oc(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 35, 36, &mut self.bits)
            };
        }
        ///Write the `power_source` field of the register.
        ///
        /// How the PD controller is powered
        pub fn set_power_source(&mut self, value: super::PpPowerSource) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 38, 40, &mut self.bits)
            };
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
    /// Global system configuration
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SystemConfig {
        /// The internal bits
        bits: [u8; 15],
    }
    impl ::device_driver::FieldSet for SystemConfig {
        const SIZE_BITS: u32 = 119;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl SystemConfig {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [5, 137, 51, 140, 25, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 15] }
        }
        ///Read the `pa_vconn_config` field of the register.
        ///
        /// Enable PA VCONN
        pub fn pa_vconn_config(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Read the `pb_vconn_config` field of the register.
        ///
        /// Enable PB VCONN
        pub fn pb_vconn_config(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `pa_pp_5_v_vbus_sw_config` field of the register.
        ///
        /// PA PP5V VBUS configuration
        pub fn pa_pp_5_v_vbus_sw_config(&self) -> super::VbusSwConfig {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 8, 11)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pb_pp_5_v_vbus_sw_config` field of the register.
        ///
        /// PB PP5V VBUS configuration
        pub fn pb_pp_5_v_vbus_sw_config(&self) -> super::VbusSwConfig {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 11, 14)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ilim_over_shoot` field of the register.
        ///
        /// PP_5V ILIM configuration
        pub fn ilim_over_shoot(&self) -> super::IlimOverShoot {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 14, 16)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pa_ppext_vbus_sw_config` field of the register.
        ///
        /// PA PPEXT configuration
        pub fn pa_ppext_vbus_sw_config(&self) -> super::PpextVbusSwConfig {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 16, 19)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pb_ppext_vbus_sw_config` field of the register.
        ///
        /// PB PPEXT configuration
        pub fn pb_ppext_vbus_sw_config(&self) -> super::PpextVbusSwConfig {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 19, 22)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `rcp_threshold` field of the register.
        ///
        /// Threshold used for RCP on PP_EXT
        pub fn rcp_threshold(&self) -> super::RcpThreshold {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 22, 24)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `multi_port_sink_policy_highest_power` field of the register.
        ///
        /// Automatic sink-path coordination, true for highest power, false for no sink management
        pub fn multi_port_sink_policy_highest_power(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 24, 25)
            };
            raw > 0
        }
        ///Read the `tbt_controller_type` field of the register.
        ///
        /// Type of TBT controller
        pub fn tbt_controller_type(&self) -> super::TbtControllerType {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 26, 29)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `enable_one_ufp_policy` field of the register.
        ///
        /// Enable bit for simple UFP policy manager
        pub fn enable_one_ufp_policy(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 29, 30)
            };
            raw > 0
        }
        ///Read the `enable_spm` field of the register.
        ///
        /// Enable bit for simple source power management
        pub fn enable_spm(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 30, 31)
            };
            raw > 0
        }
        ///Read the `multi_port_sink_non_overlap_time` field of the register.
        ///
        /// Delay configuration for MultiPortSinkPolicy
        pub fn multi_port_sink_non_overlap_time(
            &self,
        ) -> super::MultiPortSinkNonOverlapTime {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 31, 33)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `enable_i_2_c_multi_controller_mode` field of the register.
        ///
        /// Enables I2C Multi Controller mode
        pub fn enable_i_2_c_multi_controller_mode(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 33, 34)
            };
            raw > 0
        }
        ///Read the `i_2_c_timeout` field of the register.
        ///
        /// I2C bus timeout
        pub fn i_2_c_timeout(&self) -> super::I2CTimeout {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 34, 37)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `disable_eeprom_updates` field of the register.
        ///
        /// EEPROM updates not allowed if this bit asserted
        pub fn disable_eeprom_updates(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 37, 38)
            };
            raw > 0
        }
        ///Read the `emulate_single_port` field of the register.
        ///
        /// Enable only port A
        pub fn emulate_single_port(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 38, 39)
            };
            raw > 0
        }
        ///Read the `minimum_current_advertisement_1_a_5` field of the register.
        ///
        /// SPM minimum current advertisement, true for 1.5 A, false for USB default
        pub fn minimum_current_advertisement_1_a_5(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 39, 40)
            };
            raw > 0
        }
        ///Read the `usb_default_current` field of the register.
        ///
        /// Value for USB default current
        pub fn usb_default_current(&self) -> super::UsbDefaultCurrent {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 43, 45)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `epr_supported_as_source` field of the register.
        ///
        /// EPR supported as source
        pub fn epr_supported_as_source(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 45, 46)
            };
            raw > 0
        }
        ///Read the `epr_supported_as_sink` field of the register.
        ///
        /// EPR supported as sink
        pub fn epr_supported_as_sink(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 46, 47)
            };
            raw > 0
        }
        ///Read the `enable_low_power_mode_am_entry_exit` field of the register.
        ///
        /// Enable AM entry/exit on low-power mode exit/entry
        pub fn enable_low_power_mode_am_entry_exit(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 47, 48)
            };
            raw > 0
        }
        ///Read the `crossbar_polling_mode` field of the register.
        ///
        /// Enable crossbar polling mode
        pub fn crossbar_polling_mode(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 54, 55)
            };
            raw > 0
        }
        ///Read the `crossbar_config_type_1_extended` field of the register.
        ///
        /// Enable crossbar type 1 extended write
        pub fn crossbar_config_type_1_extended(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 55, 56)
            };
            raw > 0
        }
        ///Read the `external_dcdc_status_polling_interval` field of the register.
        ///
        /// External DCDC Status Polling Interval
        pub fn external_dcdc_status_polling_interval(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 56, 64)
            };
            raw
        }
        ///Read the `port_1_i_2_c_2_target_address` field of the register.
        ///
        /// Target address for Port 1 on I2C2s
        pub fn port_1_i_2_c_2_target_address(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 64, 72)
            };
            raw
        }
        ///Read the `port_2_i_2_c_2_target_address` field of the register.
        ///
        /// Target address for Port 2 on I2C2s
        pub fn port_2_i_2_c_2_target_address(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 72, 80)
            };
            raw
        }
        ///Read the `vsys_prevents_high_power` field of the register.
        ///
        /// Halts setting up external DCDC configuration until 5V power is present from the system
        pub fn vsys_prevents_high_power(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 80, 81)
            };
            raw > 0
        }
        ///Read the `wait_for_vin_3_v_3` field of the register.
        ///
        /// Stalls the PD in PTCH mode until Vsys is present
        pub fn wait_for_vin_3_v_3(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 81, 82)
            };
            raw > 0
        }
        ///Read the `wait_for_minimum_power` field of the register.
        ///
        /// Stalls the PD in PTCH mode until a power connection is made that meets the needed conditions
        pub fn wait_for_minimum_power(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 82, 83)
            };
            raw > 0
        }
        ///Read the `auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3` field of the register.
        ///
        /// On detecting VIN_3V3, auto clear the dead battery flag and reset the connection on the source port to update PD contract negotiated when the battery was dead
        pub fn auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 86, 87)
            };
            raw > 0
        }
        ///Read the `source_policy_mode` field of the register.
        ///
        /// Source Policy Mode
        pub fn source_policy_mode(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 103, 105)
            };
            raw
        }
        ///Write the `pa_vconn_config` field of the register.
        ///
        /// Enable PA VCONN
        pub fn set_pa_vconn_config(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
        ///Write the `pb_vconn_config` field of the register.
        ///
        /// Enable PB VCONN
        pub fn set_pb_vconn_config(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `pa_pp_5_v_vbus_sw_config` field of the register.
        ///
        /// PA PP5V VBUS configuration
        pub fn set_pa_pp_5_v_vbus_sw_config(&mut self, value: super::VbusSwConfig) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 8, 11, &mut self.bits)
            };
        }
        ///Write the `pb_pp_5_v_vbus_sw_config` field of the register.
        ///
        /// PB PP5V VBUS configuration
        pub fn set_pb_pp_5_v_vbus_sw_config(&mut self, value: super::VbusSwConfig) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 11, 14, &mut self.bits)
            };
        }
        ///Write the `ilim_over_shoot` field of the register.
        ///
        /// PP_5V ILIM configuration
        pub fn set_ilim_over_shoot(&mut self, value: super::IlimOverShoot) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 14, 16, &mut self.bits)
            };
        }
        ///Write the `pa_ppext_vbus_sw_config` field of the register.
        ///
        /// PA PPEXT configuration
        pub fn set_pa_ppext_vbus_sw_config(&mut self, value: super::PpextVbusSwConfig) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 16, 19, &mut self.bits)
            };
        }
        ///Write the `pb_ppext_vbus_sw_config` field of the register.
        ///
        /// PB PPEXT configuration
        pub fn set_pb_ppext_vbus_sw_config(&mut self, value: super::PpextVbusSwConfig) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 19, 22, &mut self.bits)
            };
        }
        ///Write the `rcp_threshold` field of the register.
        ///
        /// Threshold used for RCP on PP_EXT
        pub fn set_rcp_threshold(&mut self, value: super::RcpThreshold) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 22, 24, &mut self.bits)
            };
        }
        ///Write the `multi_port_sink_policy_highest_power` field of the register.
        ///
        /// Automatic sink-path coordination, true for highest power, false for no sink management
        pub fn set_multi_port_sink_policy_highest_power(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 24, 25, &mut self.bits)
            };
        }
        ///Write the `tbt_controller_type` field of the register.
        ///
        /// Type of TBT controller
        pub fn set_tbt_controller_type(&mut self, value: super::TbtControllerType) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 26, 29, &mut self.bits)
            };
        }
        ///Write the `enable_one_ufp_policy` field of the register.
        ///
        /// Enable bit for simple UFP policy manager
        pub fn set_enable_one_ufp_policy(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 29, 30, &mut self.bits)
            };
        }
        ///Write the `enable_spm` field of the register.
        ///
        /// Enable bit for simple source power management
        pub fn set_enable_spm(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 30, 31, &mut self.bits)
            };
        }
        ///Write the `multi_port_sink_non_overlap_time` field of the register.
        ///
        /// Delay configuration for MultiPortSinkPolicy
        pub fn set_multi_port_sink_non_overlap_time(
            &mut self,
            value: super::MultiPortSinkNonOverlapTime,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 31, 33, &mut self.bits)
            };
        }
        ///Write the `enable_i_2_c_multi_controller_mode` field of the register.
        ///
        /// Enables I2C Multi Controller mode
        pub fn set_enable_i_2_c_multi_controller_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 33, 34, &mut self.bits)
            };
        }
        ///Write the `i_2_c_timeout` field of the register.
        ///
        /// I2C bus timeout
        pub fn set_i_2_c_timeout(&mut self, value: super::I2CTimeout) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 34, 37, &mut self.bits)
            };
        }
        ///Write the `disable_eeprom_updates` field of the register.
        ///
        /// EEPROM updates not allowed if this bit asserted
        pub fn set_disable_eeprom_updates(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 37, 38, &mut self.bits)
            };
        }
        ///Write the `emulate_single_port` field of the register.
        ///
        /// Enable only port A
        pub fn set_emulate_single_port(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 38, 39, &mut self.bits)
            };
        }
        ///Write the `minimum_current_advertisement_1_a_5` field of the register.
        ///
        /// SPM minimum current advertisement, true for 1.5 A, false for USB default
        pub fn set_minimum_current_advertisement_1_a_5(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 39, 40, &mut self.bits)
            };
        }
        ///Write the `usb_default_current` field of the register.
        ///
        /// Value for USB default current
        pub fn set_usb_default_current(&mut self, value: super::UsbDefaultCurrent) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 43, 45, &mut self.bits)
            };
        }
        ///Write the `epr_supported_as_source` field of the register.
        ///
        /// EPR supported as source
        pub fn set_epr_supported_as_source(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 45, 46, &mut self.bits)
            };
        }
        ///Write the `epr_supported_as_sink` field of the register.
        ///
        /// EPR supported as sink
        pub fn set_epr_supported_as_sink(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 46, 47, &mut self.bits)
            };
        }
        ///Write the `enable_low_power_mode_am_entry_exit` field of the register.
        ///
        /// Enable AM entry/exit on low-power mode exit/entry
        pub fn set_enable_low_power_mode_am_entry_exit(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 47, 48, &mut self.bits)
            };
        }
        ///Write the `crossbar_polling_mode` field of the register.
        ///
        /// Enable crossbar polling mode
        pub fn set_crossbar_polling_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 54, 55, &mut self.bits)
            };
        }
        ///Write the `crossbar_config_type_1_extended` field of the register.
        ///
        /// Enable crossbar type 1 extended write
        pub fn set_crossbar_config_type_1_extended(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 55, 56, &mut self.bits)
            };
        }
        ///Write the `external_dcdc_status_polling_interval` field of the register.
        ///
        /// External DCDC Status Polling Interval
        pub fn set_external_dcdc_status_polling_interval(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 56, 64, &mut self.bits)
            };
        }
        ///Write the `port_1_i_2_c_2_target_address` field of the register.
        ///
        /// Target address for Port 1 on I2C2s
        pub fn set_port_1_i_2_c_2_target_address(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 64, 72, &mut self.bits)
            };
        }
        ///Write the `port_2_i_2_c_2_target_address` field of the register.
        ///
        /// Target address for Port 2 on I2C2s
        pub fn set_port_2_i_2_c_2_target_address(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 72, 80, &mut self.bits)
            };
        }
        ///Write the `vsys_prevents_high_power` field of the register.
        ///
        /// Halts setting up external DCDC configuration until 5V power is present from the system
        pub fn set_vsys_prevents_high_power(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 80, 81, &mut self.bits)
            };
        }
        ///Write the `wait_for_vin_3_v_3` field of the register.
        ///
        /// Stalls the PD in PTCH mode until Vsys is present
        pub fn set_wait_for_vin_3_v_3(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 81, 82, &mut self.bits)
            };
        }
        ///Write the `wait_for_minimum_power` field of the register.
        ///
        /// Stalls the PD in PTCH mode until a power connection is made that meets the needed conditions
        pub fn set_wait_for_minimum_power(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 82, 83, &mut self.bits)
            };
        }
        ///Write the `auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3` field of the register.
        ///
        /// On detecting VIN_3V3, auto clear the dead battery flag and reset the connection on the source port to update PD contract negotiated when the battery was dead
        pub fn set_auto_clr_dead_battery_flag_and_reset_on_vin_3_v_3(
            &mut self,
            value: bool,
        ) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 86, 87, &mut self.bits)
            };
        }
        ///Write the `source_policy_mode` field of the register.
        ///
        /// Source Policy Mode
        pub fn set_source_policy_mode(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 103, 105, &mut self.bits)
            };
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
            d.field(
                "port_1_i_2_c_2_target_address",
                &self.port_1_i_2_c_2_target_address(),
            );
            d.field(
                "port_2_i_2_c_2_target_address",
                &self.port_2_i_2_c_2_target_address(),
            );
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
            defmt::write!(
                f, "emulate_single_port: {=bool}, ", & self.emulate_single_port()
            );
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
                f, "vsys_prevents_high_power: {=bool}, ", & self
                .vsys_prevents_high_power()
            );
            defmt::write!(
                f, "wait_for_vin_3_v_3: {=bool}, ", & self.wait_for_vin_3_v_3()
            );
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
    /// Port control
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortControl {
        /// The internal bits
        bits: [u8; 8],
    }
    impl ::device_driver::FieldSet for PortControl {
        const SIZE_BITS: u32 = 64;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PortControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [17, 195, 65, 0, 0, 6, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 8] }
        }
        ///Read the `typec_current` field of the register.
        ///
        /// Type-C current limit
        pub fn typec_current(&self) -> super::TypecCurrent {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `process_swap_to_sink` field of the register.
        ///
        /// Process swap to sink
        pub fn process_swap_to_sink(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `initiate_swap_to_sink` field of the register.
        ///
        /// Initiate swap to sink
        pub fn initiate_swap_to_sink(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `process_swap_to_source` field of the register.
        ///
        /// Process swap to source
        pub fn process_swap_to_source(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `initiate_swap_to_source` field of the register.
        ///
        /// Initiate swap to source
        pub fn initiate_swap_to_source(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `auto_alert_enable` field of the register.
        ///
        /// Automatically initiate alert messaging
        pub fn auto_alert_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 8, 9)
            };
            raw > 0
        }
        ///Read the `auto_pps_status_enable` field of the register.
        ///
        /// Automatically return PPS_Status
        pub fn auto_pps_status_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 10, 11)
            };
            raw > 0
        }
        ///Read the `retimer_fw_update` field of the register.
        ///
        /// Enable retimer firmware update
        pub fn retimer_fw_update(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 11, 12)
            };
            raw > 0
        }
        ///Read the `process_swap_to_ufp` field of the register.
        ///
        /// Process swap to UFP
        pub fn process_swap_to_ufp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 12, 13)
            };
            raw > 0
        }
        ///Read the `initiate_swap_to_ufp` field of the register.
        ///
        /// Initiate swap to UFP
        pub fn initiate_swap_to_ufp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 13, 14)
            };
            raw > 0
        }
        ///Read the `process_swap_to_dfp` field of the register.
        ///
        /// Process swap to DFP
        pub fn process_swap_to_dfp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 14, 15)
            };
            raw > 0
        }
        ///Read the `initiate_swap_to_dfp` field of the register.
        ///
        /// Initiate swap to DFP
        pub fn initiate_swap_to_dfp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 15, 16)
            };
            raw > 0
        }
        ///Read the `automatic_id_request` field of the register.
        ///
        /// Automatically issue discover identity VDMs to appropriate SOPs
        pub fn automatic_id_request(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 16, 17)
            };
            raw > 0
        }
        ///Read the `am_intrusive_mode` field of the register.
        ///
        /// Allow host to manage alt mode process
        pub fn am_intrusive_mode(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 17, 18)
            };
            raw > 0
        }
        ///Read the `force_usb_3_gen_1` field of the register.
        ///
        /// Force USB3 Gen1 mode
        pub fn force_usb_3_gen_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 18, 19)
            };
            raw > 0
        }
        ///Read the `unconstrained_power` field of the register.
        ///
        /// External power present
        pub fn unconstrained_power(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 19, 20)
            };
            raw > 0
        }
        ///Read the `enable_current_monitor` field of the register.
        ///
        /// Enable current monitor using onboard ADC
        pub fn enable_current_monitor(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 20, 21)
            };
            raw > 0
        }
        ///Read the `sink_control` field of the register.
        ///
        /// Disable PP3/4 switches automatically
        pub fn sink_control(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 21, 22)
            };
            raw > 0
        }
        ///Read the `fr_swap_enabled` field of the register.
        ///
        /// Enable fast role swap
        pub fn fr_swap_enabled(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 22, 23)
            };
            raw > 0
        }
        ///Read the `usb_disable` field of the register.
        ///
        /// Disable USB data
        pub fn usb_disable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 29, 30)
            };
            raw > 0
        }
        ///Read the `vconn_current_limit` field of the register.
        ///
        /// Vconn current limit
        pub fn vconn_current_limit(&self) -> super::VconnCurrentLimit {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 43, 45)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `active_dbg_channel` field of the register.
        ///
        /// SBU Channel Control
        pub fn active_dbg_channel(&self) -> super::ActiveDbgChannel {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 45, 47)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `typec_current` field of the register.
        ///
        /// Type-C current limit
        pub fn set_typec_current(&mut self, value: super::TypecCurrent) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
        ///Write the `process_swap_to_sink` field of the register.
        ///
        /// Process swap to sink
        pub fn set_process_swap_to_sink(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `initiate_swap_to_sink` field of the register.
        ///
        /// Initiate swap to sink
        pub fn set_initiate_swap_to_sink(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `process_swap_to_source` field of the register.
        ///
        /// Process swap to source
        pub fn set_process_swap_to_source(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `initiate_swap_to_source` field of the register.
        ///
        /// Initiate swap to source
        pub fn set_initiate_swap_to_source(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `auto_alert_enable` field of the register.
        ///
        /// Automatically initiate alert messaging
        pub fn set_auto_alert_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 8, 9, &mut self.bits)
            };
        }
        ///Write the `auto_pps_status_enable` field of the register.
        ///
        /// Automatically return PPS_Status
        pub fn set_auto_pps_status_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 10, 11, &mut self.bits)
            };
        }
        ///Write the `retimer_fw_update` field of the register.
        ///
        /// Enable retimer firmware update
        pub fn set_retimer_fw_update(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 11, 12, &mut self.bits)
            };
        }
        ///Write the `process_swap_to_ufp` field of the register.
        ///
        /// Process swap to UFP
        pub fn set_process_swap_to_ufp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 12, 13, &mut self.bits)
            };
        }
        ///Write the `initiate_swap_to_ufp` field of the register.
        ///
        /// Initiate swap to UFP
        pub fn set_initiate_swap_to_ufp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 13, 14, &mut self.bits)
            };
        }
        ///Write the `process_swap_to_dfp` field of the register.
        ///
        /// Process swap to DFP
        pub fn set_process_swap_to_dfp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 14, 15, &mut self.bits)
            };
        }
        ///Write the `initiate_swap_to_dfp` field of the register.
        ///
        /// Initiate swap to DFP
        pub fn set_initiate_swap_to_dfp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 15, 16, &mut self.bits)
            };
        }
        ///Write the `automatic_id_request` field of the register.
        ///
        /// Automatically issue discover identity VDMs to appropriate SOPs
        pub fn set_automatic_id_request(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 16, 17, &mut self.bits)
            };
        }
        ///Write the `am_intrusive_mode` field of the register.
        ///
        /// Allow host to manage alt mode process
        pub fn set_am_intrusive_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 17, 18, &mut self.bits)
            };
        }
        ///Write the `force_usb_3_gen_1` field of the register.
        ///
        /// Force USB3 Gen1 mode
        pub fn set_force_usb_3_gen_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 18, 19, &mut self.bits)
            };
        }
        ///Write the `unconstrained_power` field of the register.
        ///
        /// External power present
        pub fn set_unconstrained_power(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 19, 20, &mut self.bits)
            };
        }
        ///Write the `enable_current_monitor` field of the register.
        ///
        /// Enable current monitor using onboard ADC
        pub fn set_enable_current_monitor(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 20, 21, &mut self.bits)
            };
        }
        ///Write the `sink_control` field of the register.
        ///
        /// Disable PP3/4 switches automatically
        pub fn set_sink_control(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 21, 22, &mut self.bits)
            };
        }
        ///Write the `fr_swap_enabled` field of the register.
        ///
        /// Enable fast role swap
        pub fn set_fr_swap_enabled(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 22, 23, &mut self.bits)
            };
        }
        ///Write the `usb_disable` field of the register.
        ///
        /// Disable USB data
        pub fn set_usb_disable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 29, 30, &mut self.bits)
            };
        }
        ///Write the `vconn_current_limit` field of the register.
        ///
        /// Vconn current limit
        pub fn set_vconn_current_limit(&mut self, value: super::VconnCurrentLimit) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 43, 45, &mut self.bits)
            };
        }
        ///Write the `active_dbg_channel` field of the register.
        ///
        /// SBU Channel Control
        pub fn set_active_dbg_channel(&mut self, value: super::ActiveDbgChannel) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 45, 47, &mut self.bits)
            };
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
            defmt::write!(
                f, "process_swap_to_ufp: {=bool}, ", & self.process_swap_to_ufp()
            );
            defmt::write!(
                f, "initiate_swap_to_ufp: {=bool}, ", & self.initiate_swap_to_ufp()
            );
            defmt::write!(
                f, "process_swap_to_dfp: {=bool}, ", & self.process_swap_to_dfp()
            );
            defmt::write!(
                f, "initiate_swap_to_dfp: {=bool}, ", & self.initiate_swap_to_dfp()
            );
            defmt::write!(
                f, "automatic_id_request: {=bool}, ", & self.automatic_id_request()
            );
            defmt::write!(f, "am_intrusive_mode: {=bool}, ", & self.am_intrusive_mode());
            defmt::write!(f, "force_usb_3_gen_1: {=bool}, ", & self.force_usb_3_gen_1());
            defmt::write!(
                f, "unconstrained_power: {=bool}, ", & self.unconstrained_power()
            );
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
    /// Active PDO contract
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ActivePdoContract {
        /// The internal bits
        bits: [u8; 6],
    }
    impl ::device_driver::FieldSet for ActivePdoContract {
        const SIZE_BITS: u32 = 48;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ActivePdoContract {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 6] }
        }
        ///Read the `active_pdo` field of the register.
        ///
        /// Active PDO
        pub fn active_pdo(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 32)
            };
            raw
        }
        ///Read the `first_pdo_control` field of the register.
        ///
        /// Bits 20-29 of the first PDO
        pub fn first_pdo_control(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::LE,
                >(&self.bits, 32, 42)
            };
            raw
        }
        ///Write the `active_pdo` field of the register.
        ///
        /// Active PDO
        pub fn set_active_pdo(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 0, 32, &mut self.bits)
            };
        }
        ///Write the `first_pdo_control` field of the register.
        ///
        /// Bits 20-29 of the first PDO
        pub fn set_first_pdo_control(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::LE,
                >(raw, 32, 42, &mut self.bits)
            };
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
    /// Active PDO contract
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ActiveRdoContract {
        /// The internal bits
        bits: [u8; 16],
    }
    impl ::device_driver::FieldSet for ActiveRdoContract {
        const SIZE_BITS: u32 = 128;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ActiveRdoContract {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 16] }
        }
        ///Read the `active_rdo` field of the register.
        ///
        /// Active RDO
        pub fn active_rdo(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 32)
            };
            raw
        }
        ///Read the `source_epr_mode_do` field of the register.
        ///
        /// Source EPR mode data object
        pub fn source_epr_mode_do(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 32, 64)
            };
            raw
        }
        ///Read the `sink_epr_mode_do` field of the register.
        ///
        /// Sink EPR mode data object
        pub fn sink_epr_mode_do(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 64, 96)
            };
            raw
        }
        ///Read the `accepted_active_rdo` field of the register.
        ///
        /// Accepted active RDO contract
        pub fn accepted_active_rdo(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 96, 128)
            };
            raw
        }
        ///Write the `active_rdo` field of the register.
        ///
        /// Active RDO
        pub fn set_active_rdo(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 0, 32, &mut self.bits)
            };
        }
        ///Write the `source_epr_mode_do` field of the register.
        ///
        /// Source EPR mode data object
        pub fn set_source_epr_mode_do(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 32, 64, &mut self.bits)
            };
        }
        ///Write the `sink_epr_mode_do` field of the register.
        ///
        /// Sink EPR mode data object
        pub fn set_sink_epr_mode_do(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 64, 96, &mut self.bits)
            };
        }
        ///Write the `accepted_active_rdo` field of the register.
        ///
        /// Accepted active RDO contract
        pub fn set_accepted_active_rdo(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 96, 128, &mut self.bits)
            };
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
            defmt::write!(
                f, "source_epr_mode_do: {=u32}, ", & self.source_epr_mode_do()
            );
            defmt::write!(f, "sink_epr_mode_do: {=u32}, ", & self.sink_epr_mode_do());
            defmt::write!(
                f, "accepted_active_rdo: {=u32}, ", & self.accepted_active_rdo()
            );
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
    /// PD status
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdStatus {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for PdStatus {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PdStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `cc_pull_up` field of the register.
        ///
        /// CC pull up value
        pub fn cc_pull_up(&self) -> super::PdCcPullUp {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `port_type` field of the register.
        ///
        /// Port type
        pub fn port_type(&self) -> super::PdPortType {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 4, 6)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `is_source` field of the register.
        ///
        /// Present role
        pub fn is_source(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `soft_reset_details` field of the register.
        ///
        /// Soft reset details
        pub fn soft_reset_details(&self) -> super::PdSoftResetDetails {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 8, 13)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `hard_reset_details` field of the register.
        ///
        /// Soft reset details
        pub fn hard_reset_details(&self) -> super::PdHardResetDetails {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 16, 22)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `error_recovery_details` field of the register.
        ///
        /// Soft reset details
        pub fn error_recovery_details(&self) -> super::PdErrorRecoveryDetails {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 22, 28)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `data_reset_details` field of the register.
        ///
        /// Data reset details
        pub fn data_reset_details(&self) -> super::PdDataResetDetails {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 28, 31)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `cc_pull_up` field of the register.
        ///
        /// CC pull up value
        pub fn set_cc_pull_up(&mut self, value: super::PdCcPullUp) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 4, &mut self.bits)
            };
        }
        ///Write the `port_type` field of the register.
        ///
        /// Port type
        pub fn set_port_type(&mut self, value: super::PdPortType) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 4, 6, &mut self.bits)
            };
        }
        ///Write the `is_source` field of the register.
        ///
        /// Present role
        pub fn set_is_source(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `soft_reset_details` field of the register.
        ///
        /// Soft reset details
        pub fn set_soft_reset_details(&mut self, value: super::PdSoftResetDetails) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 8, 13, &mut self.bits)
            };
        }
        ///Write the `hard_reset_details` field of the register.
        ///
        /// Soft reset details
        pub fn set_hard_reset_details(&mut self, value: super::PdHardResetDetails) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 16, 22, &mut self.bits)
            };
        }
        ///Write the `error_recovery_details` field of the register.
        ///
        /// Soft reset details
        pub fn set_error_recovery_details(
            &mut self,
            value: super::PdErrorRecoveryDetails,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 22, 28, &mut self.bits)
            };
        }
        ///Write the `data_reset_details` field of the register.
        ///
        /// Data reset details
        pub fn set_data_reset_details(&mut self, value: super::PdDataResetDetails) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 28, 31, &mut self.bits)
            };
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
    /// Display Port Configuration
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpConfig {
        /// The internal bits
        bits: [u8; 10],
    }
    impl ::device_driver::FieldSet for DpConfig {
        const SIZE_BITS: u32 = 80;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DpConfig {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [3, 6, 28, 0, 0, 1, 1, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 10] }
        }
        ///Read the `enable_dp_svid` field of the register.
        ///
        /// Assert this bit to enable DisplayPort SVID.
        pub fn enable_dp_svid(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Read the `enable_dp_mode` field of the register.
        ///
        /// Assert this bit to enable DisplayPort Alternate mode.
        pub fn enable_dp_mode(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `dp_port_capability` field of the register.
        ///
        /// Display port capabilities
        pub fn dp_port_capability(&self) -> super::DpPortCapability {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 8, 10)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `dp_transport_signalling` field of the register.
        ///
        /// Signaling for transport of DisplayPort protocol.
        pub fn dp_transport_signalling(&self) -> super::DpTransportSignalling {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 10, 14)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `usb_data_path` field of the register.
        ///
        /// USB data path support.
        pub fn usb_data_path(&self) -> super::DpUsbDataPath {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 15, 16)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `dfpd_pin_assignment` field of the register.
        ///
        /// DFP_D Pin Assignments Supported. Each bit corresponds to an allowed pin assignment. Multiple pin assignments may be allowed.
        pub fn dfpd_pin_assignment(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 16, 24)
            };
            raw
        }
        ///Read the `ufpd_pin_assignment` field of the register.
        ///
        /// UFP_D Pin Assignments Supported. Each bit corresponds to an allowed pin assignment. Multiple pin assignments may be allowed.
        pub fn ufpd_pin_assignment(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 24, 32)
            };
            raw
        }
        ///Read the `multi_function_preferred` field of the register.
        ///
        /// Assert this bit if multi-function is preferred.
        pub fn multi_function_preferred(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 32, 33)
            };
            raw > 0
        }
        ///Read the `dfpd_ufpd_connection_status` field of the register.
        ///
        /// This field indicates the status of the connection.
        pub fn dfpd_ufpd_connection_status(&self) -> super::DfpdUfpdConnected {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 35, 37)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `dp_vdo_version` field of the register.
        ///
        /// DP VDO Version
        pub fn dp_vdo_version(&self) -> super::DpVdoVersion {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 37, 39)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `dp_mode_auto_entry_allowed` field of the register.
        ///
        /// Assert this bit to enable auto-entry.
        pub fn dp_mode_auto_entry_allowed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 40, 41)
            };
            raw > 0
        }
        ///Read the `port_capability` field of the register.
        ///
        /// Port Capability
        pub fn port_capability(&self) -> super::PortCapability {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 48, 50)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `transport_signalling` field of the register.
        ///
        /// Transport Signalling
        pub fn transport_signalling(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 50, 54)
            };
            raw
        }
        ///Read the `receptacle_indication` field of the register.
        ///
        /// Receptacle Indication.
        pub fn receptacle_indication(&self) -> super::ReceptacleIndication {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 54, 55)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `usb_2_signalling_not_used` field of the register.
        ///
        /// USB2 signaling requirement on A6 - A7 or B6 - B7 (D+/D-) while in DP configuration.
        pub fn usb_2_signalling_not_used(&self) -> super::Usb2SignalingNotUsed {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 55, 56)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `dp_source_device_pin_assignments_supported` field of the register.
        ///
        /// DP Source DevicePinAssignments Supported
        pub fn dp_source_device_pin_assignments_supported(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 56, 64)
            };
            raw
        }
        ///Read the `dp_sink_device_pin_assignments_supported` field of the register.
        ///
        /// DP Sink DevicePinAssignments Supported
        pub fn dp_sink_device_pin_assignments_supported(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 64, 72)
            };
            raw
        }
        ///Read the `uhbr_13` field of the register.
        ///
        /// UHBR13
        pub fn uhbr_13(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 74, 75)
            };
            raw > 0
        }
        ///Read the `active_component` field of the register.
        ///
        /// ActiveComponent
        pub fn active_component(&self) -> super::ActiveComponent {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 76, 78)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `dpam_version` field of the register.
        ///
        /// DPAM version
        pub fn dpam_version(&self) -> super::DpamVersion {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 78, 80)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `enable_dp_svid` field of the register.
        ///
        /// Assert this bit to enable DisplayPort SVID.
        pub fn set_enable_dp_svid(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
        ///Write the `enable_dp_mode` field of the register.
        ///
        /// Assert this bit to enable DisplayPort Alternate mode.
        pub fn set_enable_dp_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `dp_port_capability` field of the register.
        ///
        /// Display port capabilities
        pub fn set_dp_port_capability(&mut self, value: super::DpPortCapability) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 8, 10, &mut self.bits)
            };
        }
        ///Write the `dp_transport_signalling` field of the register.
        ///
        /// Signaling for transport of DisplayPort protocol.
        pub fn set_dp_transport_signalling(
            &mut self,
            value: super::DpTransportSignalling,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 10, 14, &mut self.bits)
            };
        }
        ///Write the `usb_data_path` field of the register.
        ///
        /// USB data path support.
        pub fn set_usb_data_path(&mut self, value: super::DpUsbDataPath) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 15, 16, &mut self.bits)
            };
        }
        ///Write the `dfpd_pin_assignment` field of the register.
        ///
        /// DFP_D Pin Assignments Supported. Each bit corresponds to an allowed pin assignment. Multiple pin assignments may be allowed.
        pub fn set_dfpd_pin_assignment(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 16, 24, &mut self.bits)
            };
        }
        ///Write the `ufpd_pin_assignment` field of the register.
        ///
        /// UFP_D Pin Assignments Supported. Each bit corresponds to an allowed pin assignment. Multiple pin assignments may be allowed.
        pub fn set_ufpd_pin_assignment(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 24, 32, &mut self.bits)
            };
        }
        ///Write the `multi_function_preferred` field of the register.
        ///
        /// Assert this bit if multi-function is preferred.
        pub fn set_multi_function_preferred(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 32, 33, &mut self.bits)
            };
        }
        ///Write the `dfpd_ufpd_connection_status` field of the register.
        ///
        /// This field indicates the status of the connection.
        pub fn set_dfpd_ufpd_connection_status(
            &mut self,
            value: super::DfpdUfpdConnected,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 35, 37, &mut self.bits)
            };
        }
        ///Write the `dp_vdo_version` field of the register.
        ///
        /// DP VDO Version
        pub fn set_dp_vdo_version(&mut self, value: super::DpVdoVersion) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 37, 39, &mut self.bits)
            };
        }
        ///Write the `dp_mode_auto_entry_allowed` field of the register.
        ///
        /// Assert this bit to enable auto-entry.
        pub fn set_dp_mode_auto_entry_allowed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 40, 41, &mut self.bits)
            };
        }
        ///Write the `port_capability` field of the register.
        ///
        /// Port Capability
        pub fn set_port_capability(&mut self, value: super::PortCapability) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 48, 50, &mut self.bits)
            };
        }
        ///Write the `transport_signalling` field of the register.
        ///
        /// Transport Signalling
        pub fn set_transport_signalling(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 50, 54, &mut self.bits)
            };
        }
        ///Write the `receptacle_indication` field of the register.
        ///
        /// Receptacle Indication.
        pub fn set_receptacle_indication(&mut self, value: super::ReceptacleIndication) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 54, 55, &mut self.bits)
            };
        }
        ///Write the `usb_2_signalling_not_used` field of the register.
        ///
        /// USB2 signaling requirement on A6 - A7 or B6 - B7 (D+/D-) while in DP configuration.
        pub fn set_usb_2_signalling_not_used(
            &mut self,
            value: super::Usb2SignalingNotUsed,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 55, 56, &mut self.bits)
            };
        }
        ///Write the `dp_source_device_pin_assignments_supported` field of the register.
        ///
        /// DP Source DevicePinAssignments Supported
        pub fn set_dp_source_device_pin_assignments_supported(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 56, 64, &mut self.bits)
            };
        }
        ///Write the `dp_sink_device_pin_assignments_supported` field of the register.
        ///
        /// DP Sink DevicePinAssignments Supported
        pub fn set_dp_sink_device_pin_assignments_supported(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 64, 72, &mut self.bits)
            };
        }
        ///Write the `uhbr_13` field of the register.
        ///
        /// UHBR13
        pub fn set_uhbr_13(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 74, 75, &mut self.bits)
            };
        }
        ///Write the `active_component` field of the register.
        ///
        /// ActiveComponent
        pub fn set_active_component(&mut self, value: super::ActiveComponent) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 76, 78, &mut self.bits)
            };
        }
        ///Write the `dpam_version` field of the register.
        ///
        /// DPAM version
        pub fn set_dpam_version(&mut self, value: super::DpamVersion) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 78, 80, &mut self.bits)
            };
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
            defmt::write!(
                f, "dfpd_pin_assignment: {=u8}, ", & self.dfpd_pin_assignment()
            );
            defmt::write!(
                f, "ufpd_pin_assignment: {=u8}, ", & self.ufpd_pin_assignment()
            );
            defmt::write!(
                f, "multi_function_preferred: {=bool}, ", & self
                .multi_function_preferred()
            );
            defmt::write!(
                f, "dfpd_ufpd_connection_status: {}, ", & self
                .dfpd_ufpd_connection_status()
            );
            defmt::write!(f, "dp_vdo_version: {}, ", & self.dp_vdo_version());
            defmt::write!(
                f, "dp_mode_auto_entry_allowed: {=bool}, ", & self
                .dp_mode_auto_entry_allowed()
            );
            defmt::write!(f, "port_capability: {}, ", & self.port_capability());
            defmt::write!(
                f, "transport_signalling: {=u8}, ", & self.transport_signalling()
            );
            defmt::write!(
                f, "receptacle_indication: {}, ", & self.receptacle_indication()
            );
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
    pub struct TbtConfig {
        /// The internal bits
        bits: [u8; 8],
    }
    impl ::device_driver::FieldSet for TbtConfig {
        const SIZE_BITS: u32 = 64;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl TbtConfig {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 8] }
        }
        ///Read the `tbt_vid_en` field of the register.
        ///
        /// Assert this bit to enable Thunderbolt VID.
        pub fn tbt_vid_en(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Read the `tbt_mode_en` field of the register.
        ///
        /// Assert this bit to enable TBT mode.
        pub fn tbt_mode_en(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `advertise_900_ma_implicit_contract` field of the register.
        ///
        /// Advertise 900mA Implicit Contract.
        pub fn advertise_900_ma_implicit_contract(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `i_2_c_3_power_on_delay` field of the register.
        ///
        /// Delay for the Controller I2C commands at power on.
        pub fn i_2_c_3_power_on_delay(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 3, 7)
            };
            raw
        }
        ///Read the `pl_4_handling_en` field of the register.
        ///
        /// Enable PL4 Handling.
        pub fn pl_4_handling_en(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `tbt_emarker_override` field of the register.
        ///
        /// Configuration for non-responsive Cable Plug.
        pub fn tbt_emarker_override(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 9, 10)
            };
            raw > 0
        }
        ///Read the `an_min_power_required` field of the register.
        ///
        /// Power required for TBT mode entry.
        pub fn an_min_power_required(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 10, 11)
            };
            raw > 0
        }
        ///Read the `dual_tbt_retimer_present` field of the register.
        ///
        /// Assert this bit when there is a second TBT retimer on this port.
        pub fn dual_tbt_retimer_present(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 12, 13)
            };
            raw > 0
        }
        ///Read the `tbt_retimer_present` field of the register.
        ///
        /// Assert this bit when there is a TBT retimer on this port.
        pub fn tbt_retimer_present(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 13, 14)
            };
            raw > 0
        }
        ///Read the `data_status_hpd_events` field of the register.
        ///
        /// This bit controls how HPD events are configured.
        pub fn data_status_hpd_events(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 14, 15)
            };
            raw > 0
        }
        ///Read the `retimer_compliance_support` field of the register.
        ///
        /// Assert this bit causes the PD controller to place an attached Intel Retimer into compliance mode.
        pub fn retimer_compliance_support(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 15, 16)
            };
            raw > 0
        }
        ///Read the `legacy_tbt_adapter` field of the register.
        ///
        /// Legacy TBT Adapter.
        pub fn legacy_tbt_adapter(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 16, 17)
            };
            raw > 0
        }
        ///Read the `tbt_auto_entry_allowed` field of the register.
        ///
        /// Assert this bit to enable TBT auto-entry.
        pub fn tbt_auto_entry_allowed(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 49, 50)
            };
            raw > 0
        }
        ///Read the `usb_data_path` field of the register.
        ///
        /// USB data path support.
        pub fn usb_data_path(&self) -> super::TbtUsbDataPath {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 54, 55)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `source_vconn_delay` field of the register.
        ///
        /// Configurable delay for BR.
        pub fn source_vconn_delay(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 56, 63)
            };
            raw
        }
        ///Write the `tbt_vid_en` field of the register.
        ///
        /// Assert this bit to enable Thunderbolt VID.
        pub fn set_tbt_vid_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
        ///Write the `tbt_mode_en` field of the register.
        ///
        /// Assert this bit to enable TBT mode.
        pub fn set_tbt_mode_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `advertise_900_ma_implicit_contract` field of the register.
        ///
        /// Advertise 900mA Implicit Contract.
        pub fn set_advertise_900_ma_implicit_contract(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `i_2_c_3_power_on_delay` field of the register.
        ///
        /// Delay for the Controller I2C commands at power on.
        pub fn set_i_2_c_3_power_on_delay(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 3, 7, &mut self.bits)
            };
        }
        ///Write the `pl_4_handling_en` field of the register.
        ///
        /// Enable PL4 Handling.
        pub fn set_pl_4_handling_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `tbt_emarker_override` field of the register.
        ///
        /// Configuration for non-responsive Cable Plug.
        pub fn set_tbt_emarker_override(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 9, 10, &mut self.bits)
            };
        }
        ///Write the `an_min_power_required` field of the register.
        ///
        /// Power required for TBT mode entry.
        pub fn set_an_min_power_required(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 10, 11, &mut self.bits)
            };
        }
        ///Write the `dual_tbt_retimer_present` field of the register.
        ///
        /// Assert this bit when there is a second TBT retimer on this port.
        pub fn set_dual_tbt_retimer_present(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 12, 13, &mut self.bits)
            };
        }
        ///Write the `tbt_retimer_present` field of the register.
        ///
        /// Assert this bit when there is a TBT retimer on this port.
        pub fn set_tbt_retimer_present(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 13, 14, &mut self.bits)
            };
        }
        ///Write the `data_status_hpd_events` field of the register.
        ///
        /// This bit controls how HPD events are configured.
        pub fn set_data_status_hpd_events(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 14, 15, &mut self.bits)
            };
        }
        ///Write the `retimer_compliance_support` field of the register.
        ///
        /// Assert this bit causes the PD controller to place an attached Intel Retimer into compliance mode.
        pub fn set_retimer_compliance_support(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 15, 16, &mut self.bits)
            };
        }
        ///Write the `legacy_tbt_adapter` field of the register.
        ///
        /// Legacy TBT Adapter.
        pub fn set_legacy_tbt_adapter(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 16, 17, &mut self.bits)
            };
        }
        ///Write the `tbt_auto_entry_allowed` field of the register.
        ///
        /// Assert this bit to enable TBT auto-entry.
        pub fn set_tbt_auto_entry_allowed(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 49, 50, &mut self.bits)
            };
        }
        ///Write the `usb_data_path` field of the register.
        ///
        /// USB data path support.
        pub fn set_usb_data_path(&mut self, value: super::TbtUsbDataPath) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 54, 55, &mut self.bits)
            };
        }
        ///Write the `source_vconn_delay` field of the register.
        ///
        /// Configurable delay for BR.
        pub fn set_source_vconn_delay(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 56, 63, &mut self.bits)
            };
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
                f, "dual_tbt_retimer_present: {=bool}, ", & self
                .dual_tbt_retimer_present()
            );
            defmt::write!(
                f, "tbt_retimer_present: {=bool}, ", & self.tbt_retimer_present()
            );
            defmt::write!(
                f, "data_status_hpd_events: {=bool}, ", & self.data_status_hpd_events()
            );
            defmt::write!(
                f, "retimer_compliance_support: {=bool}, ", & self
                .retimer_compliance_support()
            );
            defmt::write!(
                f, "legacy_tbt_adapter: {=bool}, ", & self.legacy_tbt_adapter()
            );
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
    pub struct UserVidStatus {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for UserVidStatus {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl UserVidStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `usvid_detected` field of the register.
        ///
        /// Asserted when a User VID has been detected.
        pub fn usvid_detected(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Read the `usvid_active` field of the register.
        ///
        /// Asserted when a User VID is active.
        pub fn usvid_active(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `usvid_error_code` field of the register.
        ///
        /// Error code
        pub fn usvid_error_code(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 5)
            };
            raw
        }
        ///Read the `mode_1` field of the register.
        ///
        /// Asserted when Mode1 has been entered
        pub fn mode_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 9, 10)
            };
            raw > 0
        }
        ///Read the `mode_2` field of the register.
        ///
        /// Asserted when Mode2 has been entered
        pub fn mode_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 10, 11)
            };
            raw > 0
        }
        ///Read the `mode_3` field of the register.
        ///
        /// Asserted when Mode3 has been entered
        pub fn mode_3(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 11, 12)
            };
            raw > 0
        }
        ///Read the `mode_4` field of the register.
        ///
        /// Asserted when Mode4 has been entered
        pub fn mode_4(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 12, 13)
            };
            raw > 0
        }
        ///Write the `usvid_detected` field of the register.
        ///
        /// Asserted when a User VID has been detected.
        pub fn set_usvid_detected(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
        ///Write the `usvid_active` field of the register.
        ///
        /// Asserted when a User VID is active.
        pub fn set_usvid_active(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `usvid_error_code` field of the register.
        ///
        /// Error code
        pub fn set_usvid_error_code(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 5, &mut self.bits)
            };
        }
        ///Write the `mode_1` field of the register.
        ///
        /// Asserted when Mode1 has been entered
        pub fn set_mode_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 9, 10, &mut self.bits)
            };
        }
        ///Write the `mode_2` field of the register.
        ///
        /// Asserted when Mode2 has been entered
        pub fn set_mode_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 10, 11, &mut self.bits)
            };
        }
        ///Write the `mode_3` field of the register.
        ///
        /// Asserted when Mode3 has been entered
        pub fn set_mode_3(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 11, 12, &mut self.bits)
            };
        }
        ///Write the `mode_4` field of the register.
        ///
        /// Asserted when Mode4 has been entered
        pub fn set_mode_4(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 12, 13, &mut self.bits)
            };
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
    /// Intel VID status
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntelVidStatus {
        /// The internal bits
        bits: [u8; 11],
    }
    impl ::device_driver::FieldSet for IntelVidStatus {
        const SIZE_BITS: u32 = 88;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl IntelVidStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 11] }
        }
        ///Read the `intel_vid_detected` field of the register.
        ///
        /// Indicates if Intel VID is detected.
        pub fn intel_vid_detected(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Read the `tbt_mode_active` field of the register.
        ///
        /// Indicates if TBT Mode is active.
        pub fn tbt_mode_active(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `forced_tbt_mode` field of the register.
        ///
        /// Retimer in TBT state and ready for FW update
        pub fn forced_tbt_mode(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `tbt_attention_data` field of the register.
        ///
        /// Attention message contents
        pub fn tbt_attention_data(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 8, 39)
            };
            raw
        }
        ///Read the `tbt_enter_mode_data` field of the register.
        ///
        /// Data for TBT Enter mode message
        pub fn tbt_enter_mode_data(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::LE,
                >(&self.bits, 40, 56)
            };
            raw
        }
        ///Read the `tbt_mode_data_rx_sop` field of the register.
        ///
        /// Data for Discover Modes response
        pub fn tbt_mode_data_rx_sop(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::LE,
                >(&self.bits, 56, 72)
            };
            raw
        }
        ///Read the `tbt_mode_data_rx_sop_prime` field of the register.
        ///
        /// Data for Discover Modes (SOP')
        pub fn tbt_mode_data_rx_sop_prime(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::LE,
                >(&self.bits, 72, 88)
            };
            raw
        }
        ///Write the `intel_vid_detected` field of the register.
        ///
        /// Indicates if Intel VID is detected.
        pub fn set_intel_vid_detected(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
        ///Write the `tbt_mode_active` field of the register.
        ///
        /// Indicates if TBT Mode is active.
        pub fn set_tbt_mode_active(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `forced_tbt_mode` field of the register.
        ///
        /// Retimer in TBT state and ready for FW update
        pub fn set_forced_tbt_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `tbt_attention_data` field of the register.
        ///
        /// Attention message contents
        pub fn set_tbt_attention_data(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 8, 39, &mut self.bits)
            };
        }
        ///Write the `tbt_enter_mode_data` field of the register.
        ///
        /// Data for TBT Enter mode message
        pub fn set_tbt_enter_mode_data(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::LE,
                >(raw, 40, 56, &mut self.bits)
            };
        }
        ///Write the `tbt_mode_data_rx_sop` field of the register.
        ///
        /// Data for Discover Modes response
        pub fn set_tbt_mode_data_rx_sop(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::LE,
                >(raw, 56, 72, &mut self.bits)
            };
        }
        ///Write the `tbt_mode_data_rx_sop_prime` field of the register.
        ///
        /// Data for Discover Modes (SOP')
        pub fn set_tbt_mode_data_rx_sop_prime(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::LE,
                >(raw, 72, 88, &mut self.bits)
            };
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
            defmt::write!(
                f, "intel_vid_detected: {=bool}, ", & self.intel_vid_detected()
            );
            defmt::write!(f, "tbt_mode_active: {=bool}, ", & self.tbt_mode_active());
            defmt::write!(f, "forced_tbt_mode: {=bool}, ", & self.forced_tbt_mode());
            defmt::write!(
                f, "tbt_attention_data: {=u32}, ", & self.tbt_attention_data()
            );
            defmt::write!(
                f, "tbt_enter_mode_data: {=u16}, ", & self.tbt_enter_mode_data()
            );
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
    /// Received User SVID Attention VDM
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxAttnVdm {
        /// The internal bits
        bits: [u8; 9],
    }
    impl ::device_driver::FieldSet for RxAttnVdm {
        const SIZE_BITS: u32 = 72;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl RxAttnVdm {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 9] }
        }
        ///Read the `num_of_valid_vdos` field of the register.
        ///
        /// Number of valid Vdos received
        pub fn num_of_valid_vdos(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 3)
            };
            raw
        }
        ///Read the `seq_num` field of the register.
        ///
        /// Increments by one every time this register is updated
        pub fn seq_num(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 5, 8)
            };
            raw
        }
        ///Read the `vdm_header` field of the register.
        ///
        /// VDM header, Rx Vdm data object 1
        pub fn vdm_header(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 8, 40)
            };
            raw
        }
        ///Read the `vdo` field of the register.
        ///
        /// VDM data object, Rx Vdm data object 2
        pub fn vdo(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 40, 72)
            };
            raw
        }
        ///Write the `num_of_valid_vdos` field of the register.
        ///
        /// Number of valid Vdos received
        pub fn set_num_of_valid_vdos(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 3, &mut self.bits)
            };
        }
        ///Write the `seq_num` field of the register.
        ///
        /// Increments by one every time this register is updated
        pub fn set_seq_num(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 5, 8, &mut self.bits)
            };
        }
        ///Write the `vdm_header` field of the register.
        ///
        /// VDM header, Rx Vdm data object 1
        pub fn set_vdm_header(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 8, 40, &mut self.bits)
            };
        }
        ///Write the `vdo` field of the register.
        ///
        /// VDM data object, Rx Vdm data object 2
        pub fn set_vdo(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 40, 72, &mut self.bits)
            };
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
    /// Received ADO
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxAdo {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for RxAdo {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl RxAdo {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `ado` field of the register.
        ///
        /// ADO
        pub fn ado(&self) -> u32 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 32)
            };
            raw
        }
        ///Write the `ado` field of the register.
        ///
        /// ADO
        pub fn set_ado(&mut self, value: u32) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u32,
                    ::device_driver::ops::LE,
                >(raw, 0, 32, &mut self.bits)
            };
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
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        /// Controller operation mode
        Mode(Mode),
        /// Customer use
        CustomerUse(CustomerUse),
        /// Command 1 register
        Cmd1(Cmd1),
        /// Boot FW version
        Version(Version),
        /// Asserted interrupts for I2C1
        IntEventBus1(IntEventBus1),
        /// Set Sx App Config - system power state for application configuration
        SxAppConfig(SxAppConfig),
        /// Port status
        Status(Status),
        /// Power path status
        UsbStatus(UsbStatus),
        /// Power path status
        PowerPathStatus(PowerPathStatus),
        /// Global system configuration
        SystemConfig(SystemConfig),
        /// Port control
        PortControl(PortControl),
        /// Active PDO contract
        ActivePdoContract(ActivePdoContract),
        /// Active PDO contract
        ActiveRdoContract(ActiveRdoContract),
        /// PD status
        PdStatus(PdStatus),
        /// Display Port Configuration
        DpConfig(DpConfig),
        TbtConfig(TbtConfig),
        UserVidStatus(UserVidStatus),
        /// Intel VID status
        IntelVidStatus(IntelVidStatus),
        /// Received User SVID Attention VDM
        RxAttnVdm(RxAttnVdm),
        /// Received ADO
        RxAdo(RxAdo),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Mode(val) => core::fmt::Debug::fmt(val, f),
                Self::CustomerUse(val) => core::fmt::Debug::fmt(val, f),
                Self::Cmd1(val) => core::fmt::Debug::fmt(val, f),
                Self::Version(val) => core::fmt::Debug::fmt(val, f),
                Self::IntEventBus1(val) => core::fmt::Debug::fmt(val, f),
                Self::SxAppConfig(val) => core::fmt::Debug::fmt(val, f),
                Self::Status(val) => core::fmt::Debug::fmt(val, f),
                Self::UsbStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::PowerPathStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::SystemConfig(val) => core::fmt::Debug::fmt(val, f),
                Self::PortControl(val) => core::fmt::Debug::fmt(val, f),
                Self::ActivePdoContract(val) => core::fmt::Debug::fmt(val, f),
                Self::ActiveRdoContract(val) => core::fmt::Debug::fmt(val, f),
                Self::PdStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::DpConfig(val) => core::fmt::Debug::fmt(val, f),
                Self::TbtConfig(val) => core::fmt::Debug::fmt(val, f),
                Self::UserVidStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::IntelVidStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::RxAttnVdm(val) => core::fmt::Debug::fmt(val, f),
                Self::RxAdo(val) => core::fmt::Debug::fmt(val, f),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::Mode(val) => defmt::Format::format(val, f),
                Self::CustomerUse(val) => defmt::Format::format(val, f),
                Self::Cmd1(val) => defmt::Format::format(val, f),
                Self::Version(val) => defmt::Format::format(val, f),
                Self::IntEventBus1(val) => defmt::Format::format(val, f),
                Self::SxAppConfig(val) => defmt::Format::format(val, f),
                Self::Status(val) => defmt::Format::format(val, f),
                Self::UsbStatus(val) => defmt::Format::format(val, f),
                Self::PowerPathStatus(val) => defmt::Format::format(val, f),
                Self::SystemConfig(val) => defmt::Format::format(val, f),
                Self::PortControl(val) => defmt::Format::format(val, f),
                Self::ActivePdoContract(val) => defmt::Format::format(val, f),
                Self::ActiveRdoContract(val) => defmt::Format::format(val, f),
                Self::PdStatus(val) => defmt::Format::format(val, f),
                Self::DpConfig(val) => defmt::Format::format(val, f),
                Self::TbtConfig(val) => defmt::Format::format(val, f),
                Self::UserVidStatus(val) => defmt::Format::format(val, f),
                Self::IntelVidStatus(val) => defmt::Format::format(val, f),
                Self::RxAttnVdm(val) => defmt::Format::format(val, f),
                Self::RxAdo(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<Mode> for FieldSetValue {
        fn from(val: Mode) -> Self {
            Self::Mode(val)
        }
    }
    impl From<CustomerUse> for FieldSetValue {
        fn from(val: CustomerUse) -> Self {
            Self::CustomerUse(val)
        }
    }
    impl From<Cmd1> for FieldSetValue {
        fn from(val: Cmd1) -> Self {
            Self::Cmd1(val)
        }
    }
    impl From<Version> for FieldSetValue {
        fn from(val: Version) -> Self {
            Self::Version(val)
        }
    }
    impl From<IntEventBus1> for FieldSetValue {
        fn from(val: IntEventBus1) -> Self {
            Self::IntEventBus1(val)
        }
    }
    impl From<SxAppConfig> for FieldSetValue {
        fn from(val: SxAppConfig) -> Self {
            Self::SxAppConfig(val)
        }
    }
    impl From<Status> for FieldSetValue {
        fn from(val: Status) -> Self {
            Self::Status(val)
        }
    }
    impl From<UsbStatus> for FieldSetValue {
        fn from(val: UsbStatus) -> Self {
            Self::UsbStatus(val)
        }
    }
    impl From<PowerPathStatus> for FieldSetValue {
        fn from(val: PowerPathStatus) -> Self {
            Self::PowerPathStatus(val)
        }
    }
    impl From<SystemConfig> for FieldSetValue {
        fn from(val: SystemConfig) -> Self {
            Self::SystemConfig(val)
        }
    }
    impl From<PortControl> for FieldSetValue {
        fn from(val: PortControl) -> Self {
            Self::PortControl(val)
        }
    }
    impl From<ActivePdoContract> for FieldSetValue {
        fn from(val: ActivePdoContract) -> Self {
            Self::ActivePdoContract(val)
        }
    }
    impl From<ActiveRdoContract> for FieldSetValue {
        fn from(val: ActiveRdoContract) -> Self {
            Self::ActiveRdoContract(val)
        }
    }
    impl From<PdStatus> for FieldSetValue {
        fn from(val: PdStatus) -> Self {
            Self::PdStatus(val)
        }
    }
    impl From<DpConfig> for FieldSetValue {
        fn from(val: DpConfig) -> Self {
            Self::DpConfig(val)
        }
    }
    impl From<TbtConfig> for FieldSetValue {
        fn from(val: TbtConfig) -> Self {
            Self::TbtConfig(val)
        }
    }
    impl From<UserVidStatus> for FieldSetValue {
        fn from(val: UserVidStatus) -> Self {
            Self::UserVidStatus(val)
        }
    }
    impl From<IntelVidStatus> for FieldSetValue {
        fn from(val: IntelVidStatus) -> Self {
            Self::IntelVidStatus(val)
        }
    }
    impl From<RxAttnVdm> for FieldSetValue {
        fn from(val: RxAttnVdm) -> Self {
            Self::RxAttnVdm(val)
        }
    }
    impl From<RxAdo> for FieldSetValue {
        fn from(val: RxAdo) -> Self {
            Self::RxAdo(val)
        }
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlugMode {
    NotConnected = 0,
    Disabled = 1,
    Audio = 2,
    Debug = 3,
    RaDetected = 4,
    Reserved = 5,
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
            3 => Ok(Self::Debug),
            4 => Ok(Self::RaDetected),
            5 => Ok(Self::Reserved),
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
            PlugMode::Debug => 3,
            PlugMode::RaDetected => 4,
            PlugMode::Reserved => 5,
            PlugMode::ConnectedNoRa => 6,
            PlugMode::Connected => 7,
        }
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb4RequiredPlugMode {
    None = 0,
    Reserved = 1,
    Usb4 = 2,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IlimOverShoot {
    NoOvershoot = 0,
    Overshoot100Ma = 1,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcpThreshold {
    Threshold6Mv = 0,
    Threshold8Mv = 1,
    Threshold10Mv = 2,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiPortSinkNonOverlapTime {
    Delay1Ms = 0,
    Delay5Ms = 1,
    Delay10Ms = 2,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2CTimeout {
    Timeout25Ms = 0,
    Timeout50Ms = 1,
    Timeout75Ms = 2,
    Timeout100Ms = 3,
    Timeout125Ms = 4,
    Timeout150Ms = 5,
    Timeout175Ms = 6,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbDefaultCurrent {
    UsbDefault = 0,
    Current900Ma = 1,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VconnCurrentLimit {
    Current410Ma = 0,
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
    PrSwapNoGoodCrc = 17,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpamVersion {
    Version2OrEarlier = 0,
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
