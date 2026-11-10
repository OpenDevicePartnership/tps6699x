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
    /// Command data 1 register
    ///
    /// Register operation:
    /// - Address: `9`
    /// - Reset value: `0`
    #[doc(alias = "Data1")]
    pub fn data_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, Data1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 9;
        ::device_driver::RegisterOperation::new(self, address as u8, Data1::default)
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
    /// Discovered SVIDs
    ///
    /// Register operation:
    /// - Address: `33`
    /// - Reset value: `0x0`
    #[doc(alias = "DiscoveredSvids")]
    pub fn discovered_svids(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        DiscoveredSvids,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 33;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || DiscoveredSvids::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
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
    /// Port configuration
    ///
    /// Register operation:
    /// - Address: `40`
    /// - Reset value: `0x0`
    #[doc(alias = "PortConfig")]
    pub fn port_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        PortConfig,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 40;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || PortConfig::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
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
    /// Boot flags
    ///
    /// Register operation:
    /// - Address: `45`
    /// - Reset value: `0x0`
    #[doc(alias = "BootFlags")]
    pub fn boot_flags(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        BootFlags,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 45;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || BootFlags::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
        )
    }
    /// Received source capabilities
    ///
    /// Register operation:
    /// - Address: `48`
    /// - Reset value: `0x0`
    #[doc(alias = "RxSrcCaps")]
    pub fn rx_src_caps(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        RxSrcCaps,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 48;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || RxSrcCaps::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
        )
    }
    /// Received sink capabilities
    ///
    /// Register operation:
    /// - Address: `49`
    /// - Reset value: `0x0`
    #[doc(alias = "RxSnkCaps")]
    pub fn rx_snk_caps(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        RxSrcCaps,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 49;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || RxSrcCaps::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
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
    /// Autonegotiate Sink
    ///
    /// Register operation:
    /// - Address: `55`
    /// - Reset value: `[0xBE, 0x40, 0x1F, 0x0F, 0x90, 0x91, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]`
    #[doc(alias = "AutonegotiateSink")]
    pub fn autonegotiate_sink(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        AutonegotiateSink,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 55;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || AutonegotiateSink::from([
                190, 64, 31, 15, 144, 145, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0,
            ]),
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
    /// Tx Identity
    ///
    /// Register operation:
    /// - Address: `71`
    /// - Reset value: `[0x06, 0x51, 0x04, 0x00, 0xD5, 0x51, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]`
    #[doc(alias = "TxIdentity")]
    pub fn tx_identity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        TxIdentity,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 71;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || TxIdentity::from([
                6, 81, 4, 0, 213, 81, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0,
            ]),
        )
    }
    /// Received SOP Identity Data Object
    ///
    /// Register operation:
    /// - Address: `72`
    /// - Reset value: `0x0`
    #[doc(alias = "ReceivedSopIdentityData")]
    pub fn received_sop_identity_data(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReceivedSopIdentityData,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 72;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || ReceivedSopIdentityData::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
        )
    }
    /// Received SOP Prime Identity Data Object
    ///
    /// Register operation:
    /// - Address: `73`
    /// - Reset value: `0x0`
    #[doc(alias = "ReceivedSopPrimeIdentityData")]
    pub fn received_sop_prime_identity_data(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReceivedSopPrimeIdentityData,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 73;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || ReceivedSopPrimeIdentityData::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
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
    /// DP status
    ///
    /// Register operation:
    /// - Address: `88`
    /// - Reset value: `0x0`
    #[doc(alias = "DpStatus")]
    pub fn dp_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        DpStatus,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 88;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || DpStatus::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
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
    /// Received User SVID Other VDM
    ///
    /// Register operation:
    /// - Address: `97`
    /// - Reset value: `0x0`
    #[doc(alias = "RxOtherVdm")]
    pub fn rx_other_vdm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        RxOtherVdm,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 97;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            || RxOtherVdm::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ]),
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
pub struct RxOtherVdm {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 29],
}
unsafe impl ::device_driver::Fieldset for RxOtherVdm {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 29] };
}
impl RxOtherVdm {
    /// `2:0` - Read the `num_of_valid_vdos` field.
    ///
    /// Number of valid VDOs
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
    /// `4:3` - Read the `sop_type` field.
    ///
    /// Frame Type of the message in the register
    #[doc(alias = "SopType")]
    #[must_use]
    pub fn sop_type(&self) -> u8 {
        let start = 3;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `7:5` - Read the `sequence_number` field.
    ///
    /// Sequence Number
    #[doc(alias = "SequenceNumber")]
    #[must_use]
    pub fn sequence_number(&self) -> u8 {
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
    /// `39:8` - Read the `vdo_1` field.
    ///
    /// Rx Vdm data object 1
    #[doc(alias = "Vdo1")]
    #[must_use]
    pub fn vdo_1(&self) -> u32 {
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
    /// `71:40` - Read the `vdo_2` field.
    ///
    /// Rx Vdm data object 2
    #[doc(alias = "Vdo2")]
    #[must_use]
    pub fn vdo_2(&self) -> u32 {
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
    /// `103:72` - Read the `vdo_3` field.
    ///
    /// Rx Vdm data object 3
    #[doc(alias = "Vdo3")]
    #[must_use]
    pub fn vdo_3(&self) -> u32 {
        let start = 72;
        let end = 103;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `135:104` - Read the `vdo_4` field.
    ///
    /// Rx Vdm data object 4
    #[doc(alias = "Vdo4")]
    #[must_use]
    pub fn vdo_4(&self) -> u32 {
        let start = 104;
        let end = 135;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `167:136` - Read the `vdo_5` field.
    ///
    /// Rx Vdm data object 5
    #[doc(alias = "Vdo5")]
    #[must_use]
    pub fn vdo_5(&self) -> u32 {
        let start = 136;
        let end = 167;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `199:168` - Read the `vdo_6` field.
    ///
    /// Rx Vdm data object 6
    #[doc(alias = "Vdo6")]
    #[must_use]
    pub fn vdo_6(&self) -> u32 {
        let start = 168;
        let end = 199;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `231:200` - Read the `vdo_7` field.
    ///
    /// Rx Vdm data object 7
    #[doc(alias = "Vdo7")]
    #[must_use]
    pub fn vdo_7(&self) -> u32 {
        let start = 200;
        let end = 231;
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
    /// Number of valid VDOs
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
    /// `4:3` - Set the `sop_type` field.
    ///
    /// Frame Type of the message in the register
    #[doc(alias = "SopType")]
    pub fn set_sop_type(&mut self, value: u8) {
        let start = 3;
        let end = 4;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `7:5` - Set the `sequence_number` field.
    ///
    /// Sequence Number
    #[doc(alias = "SequenceNumber")]
    pub fn set_sequence_number(&mut self, value: u8) {
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
    /// `39:8` - Set the `vdo_1` field.
    ///
    /// Rx Vdm data object 1
    #[doc(alias = "Vdo1")]
    pub fn set_vdo_1(&mut self, value: u32) {
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
    /// `71:40` - Set the `vdo_2` field.
    ///
    /// Rx Vdm data object 2
    #[doc(alias = "Vdo2")]
    pub fn set_vdo_2(&mut self, value: u32) {
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
    /// `103:72` - Set the `vdo_3` field.
    ///
    /// Rx Vdm data object 3
    #[doc(alias = "Vdo3")]
    pub fn set_vdo_3(&mut self, value: u32) {
        let start = 72;
        let end = 103;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `135:104` - Set the `vdo_4` field.
    ///
    /// Rx Vdm data object 4
    #[doc(alias = "Vdo4")]
    pub fn set_vdo_4(&mut self, value: u32) {
        let start = 104;
        let end = 135;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `167:136` - Set the `vdo_5` field.
    ///
    /// Rx Vdm data object 5
    #[doc(alias = "Vdo5")]
    pub fn set_vdo_5(&mut self, value: u32) {
        let start = 136;
        let end = 167;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `199:168` - Set the `vdo_6` field.
    ///
    /// Rx Vdm data object 6
    #[doc(alias = "Vdo6")]
    pub fn set_vdo_6(&mut self, value: u32) {
        let start = 168;
        let end = 199;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `231:200` - Set the `vdo_7` field.
    ///
    /// Rx Vdm data object 7
    #[doc(alias = "Vdo7")]
    pub fn set_vdo_7(&mut self, value: u32) {
        let start = 200;
        let end = 231;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for RxOtherVdm {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 29]> for RxOtherVdm {
    fn from(bits: [u8; 29]) -> Self {
        Self { bits }
    }
}
impl From<RxOtherVdm> for [u8; 29] {
    fn from(val: RxOtherVdm) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RxOtherVdm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RxOtherVdm");
        d.field("num_of_valid_vdos", &self.num_of_valid_vdos());
        d.field("sop_type", &self.sop_type());
        d.field("sequence_number", &self.sequence_number());
        d.field("vdo_1", &self.vdo_1());
        d.field("vdo_2", &self.vdo_2());
        d.field("vdo_3", &self.vdo_3());
        d.field("vdo_4", &self.vdo_4());
        d.field("vdo_5", &self.vdo_5());
        d.field("vdo_6", &self.vdo_6());
        d.field("vdo_7", &self.vdo_7());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxOtherVdm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxOtherVdm {{ ");
        defmt::write!(f, "num_of_valid_vdos: {=u8}, ", & self.num_of_valid_vdos());
        defmt::write!(f, "sop_type: {=u8}, ", & self.sop_type());
        defmt::write!(f, "sequence_number: {=u8}, ", & self.sequence_number());
        defmt::write!(f, "vdo_1: {=u32}, ", & self.vdo_1());
        defmt::write!(f, "vdo_2: {=u32}, ", & self.vdo_2());
        defmt::write!(f, "vdo_3: {=u32}, ", & self.vdo_3());
        defmt::write!(f, "vdo_4: {=u32}, ", & self.vdo_4());
        defmt::write!(f, "vdo_5: {=u32}, ", & self.vdo_5());
        defmt::write!(f, "vdo_6: {=u32}, ", & self.vdo_6());
        defmt::write!(f, "vdo_7: {=u32}, ", & self.vdo_7());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RxOtherVdm {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RxOtherVdm {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RxOtherVdm {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RxOtherVdm {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RxOtherVdm {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RxOtherVdm {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RxOtherVdm {
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
pub struct DpStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 38],
}
unsafe impl ::device_driver::Fieldset for DpStatus {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 38] };
}
impl DpStatus {
    /// `bit 0` - Read the `dp_detected` field.
    ///
    /// DP Detected
    #[doc(alias = "DpDetected")]
    #[must_use]
    pub fn dp_detected(&self) -> bool {
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
    /// `bit 1` - Read the `dp_mode_active` field.
    ///
    /// DP Mode Active
    #[doc(alias = "DpModeActive")]
    #[must_use]
    pub fn dp_mode_active(&self) -> bool {
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
    /// `39:8` - Read the `dp_status_tx` field.
    ///
    /// DP Status TX
    #[doc(alias = "DpStatusTx")]
    #[must_use]
    pub fn dp_status_tx(&self) -> u32 {
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
    /// `71:40` - Read the `dp_status_rx` field.
    ///
    /// DP Status RX
    #[doc(alias = "DpStatusRx")]
    #[must_use]
    pub fn dp_status_rx(&self) -> u32 {
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
    /// `103:72` - Read the `dp_configure_message` field.
    ///
    /// DP Configure Message
    #[doc(alias = "DpConfigureMessage")]
    #[must_use]
    pub fn dp_configure_message(&self) -> u32 {
        let start = 72;
        let end = 103;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `135:104` - Read the `dp_mode_data` field.
    ///
    /// DP Mode Data
    #[doc(alias = "DpModeData")]
    #[must_use]
    pub fn dp_mode_data(&self) -> u32 {
        let start = 104;
        let end = 135;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `167:136` - Read the `dp_status_to_plug` field.
    ///
    /// DP Status to Plug
    #[doc(alias = "DpStatusToPlug")]
    #[must_use]
    pub fn dp_status_to_plug(&self) -> u32 {
        let start = 136;
        let end = 167;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `199:168` - Read the `dp_status_ack_from_plug` field.
    ///
    /// DP Status ACK from Plug
    #[doc(alias = "DpStatusAckFromPlug")]
    #[must_use]
    pub fn dp_status_ack_from_plug(&self) -> u32 {
        let start = 168;
        let end = 199;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `231:200` - Read the `dp_config_to_plug` field.
    ///
    /// DP Config to Plug
    #[doc(alias = "DpConfigToPlug")]
    #[must_use]
    pub fn dp_config_to_plug(&self) -> u32 {
        let start = 200;
        let end = 231;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `263:232` - Read the `dp_config_from_plug` field.
    ///
    /// DP Config from Plug
    #[doc(alias = "DpConfigFromPlug")]
    #[must_use]
    pub fn dp_config_from_plug(&self) -> u32 {
        let start = 232;
        let end = 263;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `295:264` - Read the `dp_mode_data_sopprime` field.
    ///
    /// DP Mode Data SOPPrime
    #[doc(alias = "DpModeDataSopprime")]
    #[must_use]
    pub fn dp_mode_data_sopprime(&self) -> u32 {
        let start = 264;
        let end = 295;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `299:296` - Read the `dp_signalling_rate` field.
    ///
    /// DP Signalling Rate
    #[doc(alias = "DpSignallingRate")]
    #[must_use]
    pub fn dp_signalling_rate(&self) -> u8 {
        let start = 296;
        let end = 299;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 300` - Read the `cable_uhbr_13_5_support` field.
    ///
    /// Cable UHBR13.5 Support
    #[doc(alias = "CableUhbr13_5Support")]
    #[must_use]
    pub fn cable_uhbr_13_5_support(&self) -> bool {
        let start = 300;
        let end = 300;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `302:301` - Read the `cable_active_component` field.
    ///
    /// Cable Active Component
    #[doc(alias = "CableActiveComponent")]
    #[must_use]
    pub fn cable_active_component(&self) -> u8 {
        let start = 301;
        let end = 302;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 303` - Read the `dp_ufp_vdo_version` field.
    ///
    /// DP UFP VDO Version
    #[doc(alias = "DpUfpVdoVersion")]
    #[must_use]
    pub fn dp_ufp_vdo_version(&self) -> bool {
        let start = 303;
        let end = 303;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 0` - Set the `dp_detected` field.
    ///
    /// DP Detected
    #[doc(alias = "DpDetected")]
    pub fn set_dp_detected(&mut self, value: bool) {
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
    /// `bit 1` - Set the `dp_mode_active` field.
    ///
    /// DP Mode Active
    #[doc(alias = "DpModeActive")]
    pub fn set_dp_mode_active(&mut self, value: bool) {
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
    /// `39:8` - Set the `dp_status_tx` field.
    ///
    /// DP Status TX
    #[doc(alias = "DpStatusTx")]
    pub fn set_dp_status_tx(&mut self, value: u32) {
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
    /// `71:40` - Set the `dp_status_rx` field.
    ///
    /// DP Status RX
    #[doc(alias = "DpStatusRx")]
    pub fn set_dp_status_rx(&mut self, value: u32) {
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
    /// `103:72` - Set the `dp_configure_message` field.
    ///
    /// DP Configure Message
    #[doc(alias = "DpConfigureMessage")]
    pub fn set_dp_configure_message(&mut self, value: u32) {
        let start = 72;
        let end = 103;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `135:104` - Set the `dp_mode_data` field.
    ///
    /// DP Mode Data
    #[doc(alias = "DpModeData")]
    pub fn set_dp_mode_data(&mut self, value: u32) {
        let start = 104;
        let end = 135;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `167:136` - Set the `dp_status_to_plug` field.
    ///
    /// DP Status to Plug
    #[doc(alias = "DpStatusToPlug")]
    pub fn set_dp_status_to_plug(&mut self, value: u32) {
        let start = 136;
        let end = 167;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `199:168` - Set the `dp_status_ack_from_plug` field.
    ///
    /// DP Status ACK from Plug
    #[doc(alias = "DpStatusAckFromPlug")]
    pub fn set_dp_status_ack_from_plug(&mut self, value: u32) {
        let start = 168;
        let end = 199;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `231:200` - Set the `dp_config_to_plug` field.
    ///
    /// DP Config to Plug
    #[doc(alias = "DpConfigToPlug")]
    pub fn set_dp_config_to_plug(&mut self, value: u32) {
        let start = 200;
        let end = 231;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `263:232` - Set the `dp_config_from_plug` field.
    ///
    /// DP Config from Plug
    #[doc(alias = "DpConfigFromPlug")]
    pub fn set_dp_config_from_plug(&mut self, value: u32) {
        let start = 232;
        let end = 263;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `295:264` - Set the `dp_mode_data_sopprime` field.
    ///
    /// DP Mode Data SOPPrime
    #[doc(alias = "DpModeDataSopprime")]
    pub fn set_dp_mode_data_sopprime(&mut self, value: u32) {
        let start = 264;
        let end = 295;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `299:296` - Set the `dp_signalling_rate` field.
    ///
    /// DP Signalling Rate
    #[doc(alias = "DpSignallingRate")]
    pub fn set_dp_signalling_rate(&mut self, value: u8) {
        let start = 296;
        let end = 299;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 300` - Set the `cable_uhbr_13_5_support` field.
    ///
    /// Cable UHBR13.5 Support
    #[doc(alias = "CableUhbr13_5Support")]
    pub fn set_cable_uhbr_13_5_support(&mut self, value: bool) {
        let start = 300;
        let end = 300;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `302:301` - Set the `cable_active_component` field.
    ///
    /// Cable Active Component
    #[doc(alias = "CableActiveComponent")]
    pub fn set_cable_active_component(&mut self, value: u8) {
        let start = 301;
        let end = 302;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 303` - Set the `dp_ufp_vdo_version` field.
    ///
    /// DP UFP VDO Version
    #[doc(alias = "DpUfpVdoVersion")]
    pub fn set_dp_ufp_vdo_version(&mut self, value: bool) {
        let start = 303;
        let end = 303;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for DpStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 38]> for DpStatus {
    fn from(bits: [u8; 38]) -> Self {
        Self { bits }
    }
}
impl From<DpStatus> for [u8; 38] {
    fn from(val: DpStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for DpStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("DpStatus");
        d.field("dp_detected", &self.dp_detected());
        d.field("dp_mode_active", &self.dp_mode_active());
        d.field("dp_status_tx", &self.dp_status_tx());
        d.field("dp_status_rx", &self.dp_status_rx());
        d.field("dp_configure_message", &self.dp_configure_message());
        d.field("dp_mode_data", &self.dp_mode_data());
        d.field("dp_status_to_plug", &self.dp_status_to_plug());
        d.field("dp_status_ack_from_plug", &self.dp_status_ack_from_plug());
        d.field("dp_config_to_plug", &self.dp_config_to_plug());
        d.field("dp_config_from_plug", &self.dp_config_from_plug());
        d.field("dp_mode_data_sopprime", &self.dp_mode_data_sopprime());
        d.field("dp_signalling_rate", &self.dp_signalling_rate());
        d.field("cable_uhbr_13_5_support", &self.cable_uhbr_13_5_support());
        d.field("cable_active_component", &self.cable_active_component());
        d.field("dp_ufp_vdo_version", &self.dp_ufp_vdo_version());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DpStatus {{ ");
        defmt::write!(f, "dp_detected: {=bool}, ", & self.dp_detected());
        defmt::write!(f, "dp_mode_active: {=bool}, ", & self.dp_mode_active());
        defmt::write!(f, "dp_status_tx: {=u32}, ", & self.dp_status_tx());
        defmt::write!(f, "dp_status_rx: {=u32}, ", & self.dp_status_rx());
        defmt::write!(
            f, "dp_configure_message: {=u32}, ", & self.dp_configure_message()
        );
        defmt::write!(f, "dp_mode_data: {=u32}, ", & self.dp_mode_data());
        defmt::write!(f, "dp_status_to_plug: {=u32}, ", & self.dp_status_to_plug());
        defmt::write!(
            f, "dp_status_ack_from_plug: {=u32}, ", & self.dp_status_ack_from_plug()
        );
        defmt::write!(f, "dp_config_to_plug: {=u32}, ", & self.dp_config_to_plug());
        defmt::write!(f, "dp_config_from_plug: {=u32}, ", & self.dp_config_from_plug());
        defmt::write!(
            f, "dp_mode_data_sopprime: {=u32}, ", & self.dp_mode_data_sopprime()
        );
        defmt::write!(f, "dp_signalling_rate: {=u8}, ", & self.dp_signalling_rate());
        defmt::write!(
            f, "cable_uhbr_13_5_support: {=bool}, ", & self.cable_uhbr_13_5_support()
        );
        defmt::write!(
            f, "cable_active_component: {=u8}, ", & self.cable_active_component()
        );
        defmt::write!(f, "dp_ufp_vdo_version: {=bool}, ", & self.dp_ufp_vdo_version());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for DpStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for DpStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for DpStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for DpStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for DpStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for DpStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for DpStatus {
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
pub struct ReceivedSopPrimeIdentityData {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 25],
}
unsafe impl ::device_driver::Fieldset for ReceivedSopPrimeIdentityData {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 25] };
}
impl ReceivedSopPrimeIdentityData {
    /// `2:0` - Read the `number_valid_vdos` field.
    ///
    /// Number of valid VDOs in this register (max of 6)
    #[doc(alias = "NumberValidVdos")]
    #[must_use]
    pub fn number_valid_vdos(&self) -> u8 {
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
    /// `7:6` - Read the `response_type` field.
    ///
    /// Type of response received
    #[doc(alias = "ResponseType")]
    #[must_use]
    pub fn response_type(&self) -> u8 {
        let start = 6;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `39:8` - Read the `vdo_1` field.
    ///
    /// VDO 1
    #[doc(alias = "Vdo1")]
    #[must_use]
    pub fn vdo_1(&self) -> u32 {
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
    /// `71:40` - Read the `vdo_2` field.
    ///
    /// VDO 2
    #[doc(alias = "Vdo2")]
    #[must_use]
    pub fn vdo_2(&self) -> u32 {
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
    /// `103:72` - Read the `vdo_3` field.
    ///
    /// VDO 3
    #[doc(alias = "Vdo3")]
    #[must_use]
    pub fn vdo_3(&self) -> u32 {
        let start = 72;
        let end = 103;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `135:104` - Read the `vdo_4` field.
    ///
    /// VDO 4
    #[doc(alias = "Vdo4")]
    #[must_use]
    pub fn vdo_4(&self) -> u32 {
        let start = 104;
        let end = 135;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `167:136` - Read the `vdo_5` field.
    ///
    /// VDO 5
    #[doc(alias = "Vdo5")]
    #[must_use]
    pub fn vdo_5(&self) -> u32 {
        let start = 136;
        let end = 167;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `199:168` - Read the `vdo_6` field.
    ///
    /// VDO 6
    #[doc(alias = "Vdo6")]
    #[must_use]
    pub fn vdo_6(&self) -> u32 {
        let start = 168;
        let end = 199;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `2:0` - Set the `number_valid_vdos` field.
    ///
    /// Number of valid VDOs in this register (max of 6)
    #[doc(alias = "NumberValidVdos")]
    pub fn set_number_valid_vdos(&mut self, value: u8) {
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
    /// `7:6` - Set the `response_type` field.
    ///
    /// Type of response received
    #[doc(alias = "ResponseType")]
    pub fn set_response_type(&mut self, value: u8) {
        let start = 6;
        let end = 7;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `39:8` - Set the `vdo_1` field.
    ///
    /// VDO 1
    #[doc(alias = "Vdo1")]
    pub fn set_vdo_1(&mut self, value: u32) {
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
    /// `71:40` - Set the `vdo_2` field.
    ///
    /// VDO 2
    #[doc(alias = "Vdo2")]
    pub fn set_vdo_2(&mut self, value: u32) {
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
    /// `103:72` - Set the `vdo_3` field.
    ///
    /// VDO 3
    #[doc(alias = "Vdo3")]
    pub fn set_vdo_3(&mut self, value: u32) {
        let start = 72;
        let end = 103;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `135:104` - Set the `vdo_4` field.
    ///
    /// VDO 4
    #[doc(alias = "Vdo4")]
    pub fn set_vdo_4(&mut self, value: u32) {
        let start = 104;
        let end = 135;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `167:136` - Set the `vdo_5` field.
    ///
    /// VDO 5
    #[doc(alias = "Vdo5")]
    pub fn set_vdo_5(&mut self, value: u32) {
        let start = 136;
        let end = 167;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `199:168` - Set the `vdo_6` field.
    ///
    /// VDO 6
    #[doc(alias = "Vdo6")]
    pub fn set_vdo_6(&mut self, value: u32) {
        let start = 168;
        let end = 199;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReceivedSopPrimeIdentityData {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 25]> for ReceivedSopPrimeIdentityData {
    fn from(bits: [u8; 25]) -> Self {
        Self { bits }
    }
}
impl From<ReceivedSopPrimeIdentityData> for [u8; 25] {
    fn from(val: ReceivedSopPrimeIdentityData) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReceivedSopPrimeIdentityData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReceivedSopPrimeIdentityData");
        d.field("number_valid_vdos", &self.number_valid_vdos());
        d.field("response_type", &self.response_type());
        d.field("vdo_1", &self.vdo_1());
        d.field("vdo_2", &self.vdo_2());
        d.field("vdo_3", &self.vdo_3());
        d.field("vdo_4", &self.vdo_4());
        d.field("vdo_5", &self.vdo_5());
        d.field("vdo_6", &self.vdo_6());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReceivedSopPrimeIdentityData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ReceivedSopPrimeIdentityData {{ ");
        defmt::write!(f, "number_valid_vdos: {=u8}, ", & self.number_valid_vdos());
        defmt::write!(f, "response_type: {=u8}, ", & self.response_type());
        defmt::write!(f, "vdo_1: {=u32}, ", & self.vdo_1());
        defmt::write!(f, "vdo_2: {=u32}, ", & self.vdo_2());
        defmt::write!(f, "vdo_3: {=u32}, ", & self.vdo_3());
        defmt::write!(f, "vdo_4: {=u32}, ", & self.vdo_4());
        defmt::write!(f, "vdo_5: {=u32}, ", & self.vdo_5());
        defmt::write!(f, "vdo_6: {=u32}, ", & self.vdo_6());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ReceivedSopPrimeIdentityData {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReceivedSopPrimeIdentityData {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReceivedSopPrimeIdentityData {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReceivedSopPrimeIdentityData {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReceivedSopPrimeIdentityData {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReceivedSopPrimeIdentityData {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReceivedSopPrimeIdentityData {
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
pub struct ReceivedSopIdentityData {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 25],
}
unsafe impl ::device_driver::Fieldset for ReceivedSopIdentityData {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 25] };
}
impl ReceivedSopIdentityData {
    /// `2:0` - Read the `number_valid_vdos` field.
    ///
    /// Number of valid VDOs in this register (max of 6)
    #[doc(alias = "NumberValidVdos")]
    #[must_use]
    pub fn number_valid_vdos(&self) -> u8 {
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
    /// `7:6` - Read the `response_type` field.
    ///
    /// Type of response received
    #[doc(alias = "ResponseType")]
    #[must_use]
    pub fn response_type(&self) -> u8 {
        let start = 6;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `39:8` - Read the `vdo_1` field.
    ///
    /// VDO 1
    #[doc(alias = "Vdo1")]
    #[must_use]
    pub fn vdo_1(&self) -> u32 {
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
    /// `71:40` - Read the `vdo_2` field.
    ///
    /// VDO 2
    #[doc(alias = "Vdo2")]
    #[must_use]
    pub fn vdo_2(&self) -> u32 {
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
    /// `103:72` - Read the `vdo_3` field.
    ///
    /// VDO 3
    #[doc(alias = "Vdo3")]
    #[must_use]
    pub fn vdo_3(&self) -> u32 {
        let start = 72;
        let end = 103;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `135:104` - Read the `vdo_4` field.
    ///
    /// VDO 4
    #[doc(alias = "Vdo4")]
    #[must_use]
    pub fn vdo_4(&self) -> u32 {
        let start = 104;
        let end = 135;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `167:136` - Read the `vdo_5` field.
    ///
    /// VDO 5
    #[doc(alias = "Vdo5")]
    #[must_use]
    pub fn vdo_5(&self) -> u32 {
        let start = 136;
        let end = 167;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `199:168` - Read the `vdo_6` field.
    ///
    /// VDO 6
    #[doc(alias = "Vdo6")]
    #[must_use]
    pub fn vdo_6(&self) -> u32 {
        let start = 168;
        let end = 199;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `2:0` - Set the `number_valid_vdos` field.
    ///
    /// Number of valid VDOs in this register (max of 6)
    #[doc(alias = "NumberValidVdos")]
    pub fn set_number_valid_vdos(&mut self, value: u8) {
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
    /// `7:6` - Set the `response_type` field.
    ///
    /// Type of response received
    #[doc(alias = "ResponseType")]
    pub fn set_response_type(&mut self, value: u8) {
        let start = 6;
        let end = 7;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `39:8` - Set the `vdo_1` field.
    ///
    /// VDO 1
    #[doc(alias = "Vdo1")]
    pub fn set_vdo_1(&mut self, value: u32) {
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
    /// `71:40` - Set the `vdo_2` field.
    ///
    /// VDO 2
    #[doc(alias = "Vdo2")]
    pub fn set_vdo_2(&mut self, value: u32) {
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
    /// `103:72` - Set the `vdo_3` field.
    ///
    /// VDO 3
    #[doc(alias = "Vdo3")]
    pub fn set_vdo_3(&mut self, value: u32) {
        let start = 72;
        let end = 103;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `135:104` - Set the `vdo_4` field.
    ///
    /// VDO 4
    #[doc(alias = "Vdo4")]
    pub fn set_vdo_4(&mut self, value: u32) {
        let start = 104;
        let end = 135;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `167:136` - Set the `vdo_5` field.
    ///
    /// VDO 5
    #[doc(alias = "Vdo5")]
    pub fn set_vdo_5(&mut self, value: u32) {
        let start = 136;
        let end = 167;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `199:168` - Set the `vdo_6` field.
    ///
    /// VDO 6
    #[doc(alias = "Vdo6")]
    pub fn set_vdo_6(&mut self, value: u32) {
        let start = 168;
        let end = 199;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReceivedSopIdentityData {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 25]> for ReceivedSopIdentityData {
    fn from(bits: [u8; 25]) -> Self {
        Self { bits }
    }
}
impl From<ReceivedSopIdentityData> for [u8; 25] {
    fn from(val: ReceivedSopIdentityData) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReceivedSopIdentityData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReceivedSopIdentityData");
        d.field("number_valid_vdos", &self.number_valid_vdos());
        d.field("response_type", &self.response_type());
        d.field("vdo_1", &self.vdo_1());
        d.field("vdo_2", &self.vdo_2());
        d.field("vdo_3", &self.vdo_3());
        d.field("vdo_4", &self.vdo_4());
        d.field("vdo_5", &self.vdo_5());
        d.field("vdo_6", &self.vdo_6());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReceivedSopIdentityData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ReceivedSopIdentityData {{ ");
        defmt::write!(f, "number_valid_vdos: {=u8}, ", & self.number_valid_vdos());
        defmt::write!(f, "response_type: {=u8}, ", & self.response_type());
        defmt::write!(f, "vdo_1: {=u32}, ", & self.vdo_1());
        defmt::write!(f, "vdo_2: {=u32}, ", & self.vdo_2());
        defmt::write!(f, "vdo_3: {=u32}, ", & self.vdo_3());
        defmt::write!(f, "vdo_4: {=u32}, ", & self.vdo_4());
        defmt::write!(f, "vdo_5: {=u32}, ", & self.vdo_5());
        defmt::write!(f, "vdo_6: {=u32}, ", & self.vdo_6());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ReceivedSopIdentityData {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReceivedSopIdentityData {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReceivedSopIdentityData {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReceivedSopIdentityData {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReceivedSopIdentityData {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReceivedSopIdentityData {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReceivedSopIdentityData {
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
pub struct TxIdentity {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 25],
}
unsafe impl ::device_driver::Fieldset for TxIdentity {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 25] };
}
impl TxIdentity {
    /// `2:0` - Read the `number_valid_vdos` field.
    ///
    /// Number of valid VDOs in this register
    #[doc(alias = "NumberValidVdos")]
    #[must_use]
    pub fn number_valid_vdos(&self) -> u8 {
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
    /// `23:8` - Read the `vendor_id` field.
    ///
    /// Vendor ID as defined in USB PD specification.
    /// This value is also used to populate VID in TX_MIDB_SOP, TX_SCEDB, and TX_SKEDB registers.
    #[doc(alias = "VendorId")]
    #[must_use]
    pub fn vendor_id(&self) -> u16 {
        let start = 8;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `33:31` - Read the `product_type_dfp` field.
    ///
    /// Product Type DFP as defined in USB PD specification
    #[doc(alias = "ProductTypeDfp")]
    #[must_use]
    pub fn product_type_dfp(&self) -> ProductTypeDfp {
        let start = 31;
        let end = 33;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 34` - Read the `modal_operation_supported` field.
    ///
    /// Assert this bit if Alternate Modes are supported
    #[doc(alias = "ModalOperationSupported")]
    #[must_use]
    pub fn modal_operation_supported(&self) -> bool {
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
    /// `37:35` - Read the `product_type_ufp` field.
    ///
    /// Product Type UFP as defined in USB PD specification
    #[doc(alias = "ProductTypeUfp")]
    #[must_use]
    pub fn product_type_ufp(&self) -> ProductTypeUfp {
        let start = 35;
        let end = 37;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 38` - Read the `usb_communication_capable_as_device` field.
    ///
    /// Assert if USB communications capable as a device
    #[doc(alias = "UsbCommunicationCapableAsDevice")]
    #[must_use]
    pub fn usb_communication_capable_as_device(&self) -> bool {
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
    /// `bit 39` - Read the `usb_communication_capable_as_host` field.
    ///
    /// Assert if USB communications capable as a host
    #[doc(alias = "UsbCommunicationCapableAsHost")]
    #[must_use]
    pub fn usb_communication_capable_as_host(&self) -> bool {
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
    /// `71:40` - Read the `certification_test_id` field.
    ///
    /// 32-bit XID assigned by USB-IF.
    /// This value is also used to populate XID in TX_SCEDB and TX_SKEDB registers.
    #[doc(alias = "CertificationTestId")]
    #[must_use]
    pub fn certification_test_id(&self) -> u32 {
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
    /// `87:72` - Read the `bcd_device` field.
    ///
    /// FW version for the PD controller (read-only)
    #[doc(alias = "BcdDevice")]
    #[must_use]
    pub fn bcd_device(&self) -> u16 {
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
    /// `103:88` - Read the `usb_product_id` field.
    ///
    /// Product ID used to populate PID in other registers.
    /// This value is also used to populate PID in TX_MIDB_SOP, TX_SCEDB, and TX_SKEDB registers.
    #[doc(alias = "UsbProductId")]
    #[must_use]
    pub fn usb_product_id(&self) -> u16 {
        let start = 88;
        let end = 103;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `135:104` - Read the `ufp_1_vdo` field.
    ///
    /// UFP1 VDO
    #[doc(alias = "Ufp1Vdo")]
    #[must_use]
    pub fn ufp_1_vdo(&self) -> u32 {
        let start = 104;
        let end = 135;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `199:168` - Read the `dfp_1_vdo` field.
    ///
    /// DFP1 VDO
    #[doc(alias = "Dfp1Vdo")]
    #[must_use]
    pub fn dfp_1_vdo(&self) -> u32 {
        let start = 168;
        let end = 199;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `2:0` - Set the `number_valid_vdos` field.
    ///
    /// Number of valid VDOs in this register
    #[doc(alias = "NumberValidVdos")]
    pub fn set_number_valid_vdos(&mut self, value: u8) {
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
    /// `23:8` - Set the `vendor_id` field.
    ///
    /// Vendor ID as defined in USB PD specification.
    /// This value is also used to populate VID in TX_MIDB_SOP, TX_SCEDB, and TX_SKEDB registers.
    #[doc(alias = "VendorId")]
    pub fn set_vendor_id(&mut self, value: u16) {
        let start = 8;
        let end = 23;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `33:31` - Set the `product_type_dfp` field.
    ///
    /// Product Type DFP as defined in USB PD specification
    #[doc(alias = "ProductTypeDfp")]
    pub fn set_product_type_dfp(&mut self, value: ProductTypeDfp) {
        let start = 31;
        let end = 33;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 34` - Set the `modal_operation_supported` field.
    ///
    /// Assert this bit if Alternate Modes are supported
    #[doc(alias = "ModalOperationSupported")]
    pub fn set_modal_operation_supported(&mut self, value: bool) {
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
    /// `37:35` - Set the `product_type_ufp` field.
    ///
    /// Product Type UFP as defined in USB PD specification
    #[doc(alias = "ProductTypeUfp")]
    pub fn set_product_type_ufp(&mut self, value: ProductTypeUfp) {
        let start = 35;
        let end = 37;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 38` - Set the `usb_communication_capable_as_device` field.
    ///
    /// Assert if USB communications capable as a device
    #[doc(alias = "UsbCommunicationCapableAsDevice")]
    pub fn set_usb_communication_capable_as_device(&mut self, value: bool) {
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
    /// `bit 39` - Set the `usb_communication_capable_as_host` field.
    ///
    /// Assert if USB communications capable as a host
    #[doc(alias = "UsbCommunicationCapableAsHost")]
    pub fn set_usb_communication_capable_as_host(&mut self, value: bool) {
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
    /// `71:40` - Set the `certification_test_id` field.
    ///
    /// 32-bit XID assigned by USB-IF.
    /// This value is also used to populate XID in TX_SCEDB and TX_SKEDB registers.
    #[doc(alias = "CertificationTestId")]
    pub fn set_certification_test_id(&mut self, value: u32) {
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
    /// `87:72` - Set the `bcd_device` field.
    ///
    /// FW version for the PD controller (read-only)
    #[doc(alias = "BcdDevice")]
    pub fn set_bcd_device(&mut self, value: u16) {
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
    /// `103:88` - Set the `usb_product_id` field.
    ///
    /// Product ID used to populate PID in other registers.
    /// This value is also used to populate PID in TX_MIDB_SOP, TX_SCEDB, and TX_SKEDB registers.
    #[doc(alias = "UsbProductId")]
    pub fn set_usb_product_id(&mut self, value: u16) {
        let start = 88;
        let end = 103;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `135:104` - Set the `ufp_1_vdo` field.
    ///
    /// UFP1 VDO
    #[doc(alias = "Ufp1Vdo")]
    pub fn set_ufp_1_vdo(&mut self, value: u32) {
        let start = 104;
        let end = 135;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `199:168` - Set the `dfp_1_vdo` field.
    ///
    /// DFP1 VDO
    #[doc(alias = "Dfp1Vdo")]
    pub fn set_dfp_1_vdo(&mut self, value: u32) {
        let start = 168;
        let end = 199;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for TxIdentity {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 25]> for TxIdentity {
    fn from(bits: [u8; 25]) -> Self {
        Self { bits }
    }
}
impl From<TxIdentity> for [u8; 25] {
    fn from(val: TxIdentity) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for TxIdentity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("TxIdentity");
        d.field("number_valid_vdos", &self.number_valid_vdos());
        d.field("vendor_id", &self.vendor_id());
        d.field("product_type_dfp", &self.product_type_dfp());
        d.field("modal_operation_supported", &self.modal_operation_supported());
        d.field("product_type_ufp", &self.product_type_ufp());
        d.field(
            "usb_communication_capable_as_device",
            &self.usb_communication_capable_as_device(),
        );
        d.field(
            "usb_communication_capable_as_host",
            &self.usb_communication_capable_as_host(),
        );
        d.field("certification_test_id", &self.certification_test_id());
        d.field("bcd_device", &self.bcd_device());
        d.field("usb_product_id", &self.usb_product_id());
        d.field("ufp_1_vdo", &self.ufp_1_vdo());
        d.field("dfp_1_vdo", &self.dfp_1_vdo());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxIdentity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxIdentity {{ ");
        defmt::write!(f, "number_valid_vdos: {=u8}, ", & self.number_valid_vdos());
        defmt::write!(f, "vendor_id: {=u16}, ", & self.vendor_id());
        defmt::write!(f, "product_type_dfp: {}, ", & self.product_type_dfp());
        defmt::write!(
            f, "modal_operation_supported: {=bool}, ", & self.modal_operation_supported()
        );
        defmt::write!(f, "product_type_ufp: {}, ", & self.product_type_ufp());
        defmt::write!(
            f, "usb_communication_capable_as_device: {=bool}, ", & self
            .usb_communication_capable_as_device()
        );
        defmt::write!(
            f, "usb_communication_capable_as_host: {=bool}, ", & self
            .usb_communication_capable_as_host()
        );
        defmt::write!(
            f, "certification_test_id: {=u32}, ", & self.certification_test_id()
        );
        defmt::write!(f, "bcd_device: {=u16}, ", & self.bcd_device());
        defmt::write!(f, "usb_product_id: {=u16}, ", & self.usb_product_id());
        defmt::write!(f, "ufp_1_vdo: {=u32}, ", & self.ufp_1_vdo());
        defmt::write!(f, "dfp_1_vdo: {=u32}, ", & self.dfp_1_vdo());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for TxIdentity {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for TxIdentity {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for TxIdentity {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for TxIdentity {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for TxIdentity {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for TxIdentity {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for TxIdentity {
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
pub struct AutonegotiateSink {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 24],
}
unsafe impl ::device_driver::Fieldset for AutonegotiateSink {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 24] };
}
impl AutonegotiateSink {
    /// `bit 0` - Read the `auto_neg_rdo_priority` field.
    ///
    /// Configuration for tie-breaker in PDO selection
    #[doc(alias = "AutoNegRdoPriority")]
    #[must_use]
    pub fn auto_neg_rdo_priority(&self) -> AutoNegRdoPriority {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 1` - Read the `no_usb_suspend` field.
    ///
    /// Value used for the NoUSBSusp Flag in the RDO
    #[doc(alias = "NoUsbSuspend")]
    #[must_use]
    pub fn no_usb_suspend(&self) -> bool {
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
    /// `bit 2` - Read the `auto_compute_sink_min_power` field.
    ///
    /// How to compute the minimum power that the sink requires
    #[doc(alias = "AutoComputeSinkMinPower")]
    #[must_use]
    pub fn auto_compute_sink_min_power(&self) -> AutoComputeSinkMinPower {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 3` - Read the `no_capability_mismatch` field.
    ///
    /// Configuration for capability mismatch in RDO
    #[doc(alias = "NoCapabilityMismatch")]
    #[must_use]
    pub fn no_capability_mismatch(&self) -> NoCapabilityMismatch {
        let start = 3;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 4` - Read the `auto_compute_sink_min_voltage` field.
    ///
    /// How to compute the minimum voltage that the sink requires
    #[doc(alias = "AutoComputeSinkMinVoltage")]
    #[must_use]
    pub fn auto_compute_sink_min_voltage(&self) -> AutoComputeSinkMinVoltage {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 5` - Read the `auto_compute_sink_max_voltage` field.
    ///
    /// How to compute the maximum voltage that the sink requires
    #[doc(alias = "AutoComputeSinkMaxVoltage")]
    #[must_use]
    pub fn auto_compute_sink_max_voltage(&self) -> AutoComputeSinkMaxVoltage {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 6` - Read the `auto_disable_sink_upon_capability_mismatch` field.
    ///
    /// Sink path and capability mismatch setting
    #[doc(alias = "AutoDisableSinkUponCapabilityMismatch")]
    #[must_use]
    pub fn auto_disable_sink_upon_capability_mismatch(&self) -> bool {
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
    /// `bit 7` - Read the `auto_disable_input_for_sink_standby` field.
    ///
    /// Disable the sinking path during contract negotiation, and enable it back after receiving PS-RDY from SRC
    #[doc(alias = "AutoDisableInputForSinkStandby")]
    #[must_use]
    pub fn auto_disable_input_for_sink_standby(&self) -> bool {
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
    /// `bit 8` - Read the `auto_disable_input_for_sink_standby_in_dbm` field.
    ///
    /// Disable the sinking path during contract negotiation, and enable it back after receiving PS-RDY from SRC when in DBM
    #[doc(alias = "AutoDisableInputForSinkStandbyInDbm")]
    #[must_use]
    pub fn auto_disable_input_for_sink_standby_in_dbm(&self) -> bool {
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
    /// `bit 9` - Read the `auto_enable_standby_srdy` field.
    ///
    /// Automatically re-enable the sink path for Sink Standby when PS-RDY is received
    #[doc(alias = "AutoEnableStandbySrdy")]
    #[must_use]
    pub fn auto_enable_standby_srdy(&self) -> bool {
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
    /// `bit 10` - Read the `auto_enable_input_after_snk_ready_in_dbm` field.
    ///
    /// Keep the sink path disabled until final PD contract is set
    #[doc(alias = "AutoEnableInputAfterSnkReadyInDbm")]
    #[must_use]
    pub fn auto_enable_input_after_snk_ready_in_dbm(&self) -> bool {
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
    /// `21:12` - Read the `auto_neg_max_current` field.
    ///
    /// Maximum current to request (10 mA per LSB)
    #[doc(alias = "AutoNegMaxCurrent")]
    #[must_use]
    pub fn auto_neg_max_current(&self) -> u16 {
        let start = 12;
        let end = 21;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:22` - Read the `auto_neg_sink_min_required_power` field.
    ///
    /// Minimum operating power required by the sink (250 mW per LSB)
    #[doc(alias = "AutoNegSinkMinRequiredPower")]
    #[must_use]
    pub fn auto_neg_sink_min_required_power(&self) -> u16 {
        let start = 22;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `41:32` - Read the `auto_neg_max_voltage` field.
    ///
    /// Maximum voltage to request (50 mV per LSB)
    #[doc(alias = "AutoNegMaxVoltage")]
    #[must_use]
    pub fn auto_neg_max_voltage(&self) -> u16 {
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
    /// `51:42` - Read the `auto_neg_min_voltage` field.
    ///
    /// Minimum voltage to request (50 mV per LSB)
    #[doc(alias = "AutoNegMinVoltage")]
    #[must_use]
    pub fn auto_neg_min_voltage(&self) -> u16 {
        let start = 42;
        let end = 51;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `61:52` - Read the `auto_neg_capabilities_mismatch_power` field.
    ///
    /// Capabilities mismatch power threshold (250 mW per LSB)
    #[doc(alias = "AutoNegCapabilitiesMismatchPower")]
    #[must_use]
    pub fn auto_neg_capabilities_mismatch_power(&self) -> u16 {
        let start = 52;
        let end = 61;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 64` - Read the `pps_enable_sink_mode` field.
    ///
    /// Enable Sink PPS mode
    #[doc(alias = "PpsEnableSinkMode")]
    #[must_use]
    pub fn pps_enable_sink_mode(&self) -> bool {
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
    /// `66:65` - Read the `pps_request_interval` field.
    ///
    /// Sink PPS request interval
    #[doc(alias = "PpsRequestInterval")]
    #[must_use]
    pub fn pps_request_interval(&self) -> PpsRequestInterval {
        let start = 65;
        let end = 66;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 67` - Read the `pps_source_operating_mode` field.
    ///
    /// Selection for CV or CC mode
    #[doc(alias = "PpsSourceOperatingMode")]
    #[must_use]
    pub fn pps_source_operating_mode(&self) -> bool {
        let start = 67;
        let end = 67;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 68` - Read the `pps_required_full_voltage_range` field.
    ///
    /// Select only a source with full voltage range
    #[doc(alias = "PpsRequiredFullVoltageRange")]
    #[must_use]
    pub fn pps_required_full_voltage_range(&self) -> bool {
        let start = 68;
        let end = 68;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 69` - Read the `pps_disable_sink_upon_non_apdo_contract` field.
    ///
    /// Sink path handling during supply type transition
    #[doc(alias = "PpsDisableSinkUponNonApdoContract")]
    #[must_use]
    pub fn pps_disable_sink_upon_non_apdo_contract(&self) -> bool {
        let start = 69;
        let end = 69;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 83` - Read the `enable_sink_path_above_pps_sink_voltage_threshold` field.
    ///
    /// Enable Sink Path above PPS Sink Voltage threshold
    #[doc(alias = "EnableSinkPathAbovePpsSinkVoltageThreshold")]
    #[must_use]
    pub fn enable_sink_path_above_pps_sink_voltage_threshold(&self) -> bool {
        let start = 83;
        let end = 83;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 84` - Read the `disable_sink_if_not_in_pps_contract` field.
    ///
    /// Disable Sink Path if not in PPS contract
    #[doc(alias = "DisableSinkIfNotInPpsContract")]
    #[must_use]
    pub fn disable_sink_if_not_in_pps_contract(&self) -> bool {
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
    /// `95:85` - Read the `pps_sink_voltage_threshold` field.
    ///
    /// Operation current in Sink PPS mode
    #[doc(alias = "PpsSinkVoltageThreshold")]
    #[must_use]
    pub fn pps_sink_voltage_threshold(&self) -> u16 {
        let start = 85;
        let end = 95;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `102:96` - Read the `pps_operating_current` field.
    ///
    /// Operation current in Sink PPS mode (50 mA per LSB)
    #[doc(alias = "PpsOperatingCurrent")]
    #[must_use]
    pub fn pps_operating_current(&self) -> u8 {
        let start = 96;
        let end = 102;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `115:105` - Read the `pps_output_voltage` field.
    ///
    /// Desired output voltage (20 mV per LSB)
    #[doc(alias = "PpsOutputVoltage")]
    #[must_use]
    pub fn pps_output_voltage(&self) -> u16 {
        let start = 105;
        let end = 115;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 128` - Read the `epr_avs_enable_sink_mode` field.
    ///
    /// Enable Sink EPR AVS mode
    #[doc(alias = "EprAvsEnableSinkMode")]
    #[must_use]
    pub fn epr_avs_enable_sink_mode(&self) -> bool {
        let start = 128;
        let end = 128;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `166:160` - Read the `epr_avs_operating_current` field.
    ///
    /// AVS operating current
    #[doc(alias = "EprAvsOperatingCurrent")]
    #[must_use]
    pub fn epr_avs_operating_current(&self) -> u8 {
        let start = 160;
        let end = 166;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `180:169` - Read the `epr_avs_output_voltage` field.
    ///
    /// AVS operating voltage
    #[doc(alias = "EprAvsOutputVoltage")]
    #[must_use]
    pub fn epr_avs_output_voltage(&self) -> u16 {
        let start = 169;
        let end = 180;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 0` - Set the `auto_neg_rdo_priority` field.
    ///
    /// Configuration for tie-breaker in PDO selection
    #[doc(alias = "AutoNegRdoPriority")]
    pub fn set_auto_neg_rdo_priority(&mut self, value: AutoNegRdoPriority) {
        let start = 0;
        let end = 0;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `no_usb_suspend` field.
    ///
    /// Value used for the NoUSBSusp Flag in the RDO
    #[doc(alias = "NoUsbSuspend")]
    pub fn set_no_usb_suspend(&mut self, value: bool) {
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
    /// `bit 2` - Set the `auto_compute_sink_min_power` field.
    ///
    /// How to compute the minimum power that the sink requires
    #[doc(alias = "AutoComputeSinkMinPower")]
    pub fn set_auto_compute_sink_min_power(&mut self, value: AutoComputeSinkMinPower) {
        let start = 2;
        let end = 2;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 3` - Set the `no_capability_mismatch` field.
    ///
    /// Configuration for capability mismatch in RDO
    #[doc(alias = "NoCapabilityMismatch")]
    pub fn set_no_capability_mismatch(&mut self, value: NoCapabilityMismatch) {
        let start = 3;
        let end = 3;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `auto_compute_sink_min_voltage` field.
    ///
    /// How to compute the minimum voltage that the sink requires
    #[doc(alias = "AutoComputeSinkMinVoltage")]
    pub fn set_auto_compute_sink_min_voltage(
        &mut self,
        value: AutoComputeSinkMinVoltage,
    ) {
        let start = 4;
        let end = 4;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `auto_compute_sink_max_voltage` field.
    ///
    /// How to compute the maximum voltage that the sink requires
    #[doc(alias = "AutoComputeSinkMaxVoltage")]
    pub fn set_auto_compute_sink_max_voltage(
        &mut self,
        value: AutoComputeSinkMaxVoltage,
    ) {
        let start = 5;
        let end = 5;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `auto_disable_sink_upon_capability_mismatch` field.
    ///
    /// Sink path and capability mismatch setting
    #[doc(alias = "AutoDisableSinkUponCapabilityMismatch")]
    pub fn set_auto_disable_sink_upon_capability_mismatch(&mut self, value: bool) {
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
    /// `bit 7` - Set the `auto_disable_input_for_sink_standby` field.
    ///
    /// Disable the sinking path during contract negotiation, and enable it back after receiving PS-RDY from SRC
    #[doc(alias = "AutoDisableInputForSinkStandby")]
    pub fn set_auto_disable_input_for_sink_standby(&mut self, value: bool) {
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
    /// `bit 8` - Set the `auto_disable_input_for_sink_standby_in_dbm` field.
    ///
    /// Disable the sinking path during contract negotiation, and enable it back after receiving PS-RDY from SRC when in DBM
    #[doc(alias = "AutoDisableInputForSinkStandbyInDbm")]
    pub fn set_auto_disable_input_for_sink_standby_in_dbm(&mut self, value: bool) {
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
    /// `bit 9` - Set the `auto_enable_standby_srdy` field.
    ///
    /// Automatically re-enable the sink path for Sink Standby when PS-RDY is received
    #[doc(alias = "AutoEnableStandbySrdy")]
    pub fn set_auto_enable_standby_srdy(&mut self, value: bool) {
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
    /// `bit 10` - Set the `auto_enable_input_after_snk_ready_in_dbm` field.
    ///
    /// Keep the sink path disabled until final PD contract is set
    #[doc(alias = "AutoEnableInputAfterSnkReadyInDbm")]
    pub fn set_auto_enable_input_after_snk_ready_in_dbm(&mut self, value: bool) {
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
    /// `21:12` - Set the `auto_neg_max_current` field.
    ///
    /// Maximum current to request (10 mA per LSB)
    #[doc(alias = "AutoNegMaxCurrent")]
    pub fn set_auto_neg_max_current(&mut self, value: u16) {
        let start = 12;
        let end = 21;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `31:22` - Set the `auto_neg_sink_min_required_power` field.
    ///
    /// Minimum operating power required by the sink (250 mW per LSB)
    #[doc(alias = "AutoNegSinkMinRequiredPower")]
    pub fn set_auto_neg_sink_min_required_power(&mut self, value: u16) {
        let start = 22;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `41:32` - Set the `auto_neg_max_voltage` field.
    ///
    /// Maximum voltage to request (50 mV per LSB)
    #[doc(alias = "AutoNegMaxVoltage")]
    pub fn set_auto_neg_max_voltage(&mut self, value: u16) {
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
    /// `51:42` - Set the `auto_neg_min_voltage` field.
    ///
    /// Minimum voltage to request (50 mV per LSB)
    #[doc(alias = "AutoNegMinVoltage")]
    pub fn set_auto_neg_min_voltage(&mut self, value: u16) {
        let start = 42;
        let end = 51;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `61:52` - Set the `auto_neg_capabilities_mismatch_power` field.
    ///
    /// Capabilities mismatch power threshold (250 mW per LSB)
    #[doc(alias = "AutoNegCapabilitiesMismatchPower")]
    pub fn set_auto_neg_capabilities_mismatch_power(&mut self, value: u16) {
        let start = 52;
        let end = 61;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 64` - Set the `pps_enable_sink_mode` field.
    ///
    /// Enable Sink PPS mode
    #[doc(alias = "PpsEnableSinkMode")]
    pub fn set_pps_enable_sink_mode(&mut self, value: bool) {
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
    /// `66:65` - Set the `pps_request_interval` field.
    ///
    /// Sink PPS request interval
    #[doc(alias = "PpsRequestInterval")]
    pub fn set_pps_request_interval(&mut self, value: PpsRequestInterval) {
        let start = 65;
        let end = 66;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 67` - Set the `pps_source_operating_mode` field.
    ///
    /// Selection for CV or CC mode
    #[doc(alias = "PpsSourceOperatingMode")]
    pub fn set_pps_source_operating_mode(&mut self, value: bool) {
        let start = 67;
        let end = 67;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 68` - Set the `pps_required_full_voltage_range` field.
    ///
    /// Select only a source with full voltage range
    #[doc(alias = "PpsRequiredFullVoltageRange")]
    pub fn set_pps_required_full_voltage_range(&mut self, value: bool) {
        let start = 68;
        let end = 68;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 69` - Set the `pps_disable_sink_upon_non_apdo_contract` field.
    ///
    /// Sink path handling during supply type transition
    #[doc(alias = "PpsDisableSinkUponNonApdoContract")]
    pub fn set_pps_disable_sink_upon_non_apdo_contract(&mut self, value: bool) {
        let start = 69;
        let end = 69;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 83` - Set the `enable_sink_path_above_pps_sink_voltage_threshold` field.
    ///
    /// Enable Sink Path above PPS Sink Voltage threshold
    #[doc(alias = "EnableSinkPathAbovePpsSinkVoltageThreshold")]
    pub fn set_enable_sink_path_above_pps_sink_voltage_threshold(
        &mut self,
        value: bool,
    ) {
        let start = 83;
        let end = 83;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 84` - Set the `disable_sink_if_not_in_pps_contract` field.
    ///
    /// Disable Sink Path if not in PPS contract
    #[doc(alias = "DisableSinkIfNotInPpsContract")]
    pub fn set_disable_sink_if_not_in_pps_contract(&mut self, value: bool) {
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
    /// `95:85` - Set the `pps_sink_voltage_threshold` field.
    ///
    /// Operation current in Sink PPS mode
    #[doc(alias = "PpsSinkVoltageThreshold")]
    pub fn set_pps_sink_voltage_threshold(&mut self, value: u16) {
        let start = 85;
        let end = 95;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `102:96` - Set the `pps_operating_current` field.
    ///
    /// Operation current in Sink PPS mode (50 mA per LSB)
    #[doc(alias = "PpsOperatingCurrent")]
    pub fn set_pps_operating_current(&mut self, value: u8) {
        let start = 96;
        let end = 102;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `115:105` - Set the `pps_output_voltage` field.
    ///
    /// Desired output voltage (20 mV per LSB)
    #[doc(alias = "PpsOutputVoltage")]
    pub fn set_pps_output_voltage(&mut self, value: u16) {
        let start = 105;
        let end = 115;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 128` - Set the `epr_avs_enable_sink_mode` field.
    ///
    /// Enable Sink EPR AVS mode
    #[doc(alias = "EprAvsEnableSinkMode")]
    pub fn set_epr_avs_enable_sink_mode(&mut self, value: bool) {
        let start = 128;
        let end = 128;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `166:160` - Set the `epr_avs_operating_current` field.
    ///
    /// AVS operating current
    #[doc(alias = "EprAvsOperatingCurrent")]
    pub fn set_epr_avs_operating_current(&mut self, value: u8) {
        let start = 160;
        let end = 166;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `180:169` - Set the `epr_avs_output_voltage` field.
    ///
    /// AVS operating voltage
    #[doc(alias = "EprAvsOutputVoltage")]
    pub fn set_epr_avs_output_voltage(&mut self, value: u16) {
        let start = 169;
        let end = 180;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for AutonegotiateSink {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 24]> for AutonegotiateSink {
    fn from(bits: [u8; 24]) -> Self {
        Self { bits }
    }
}
impl From<AutonegotiateSink> for [u8; 24] {
    fn from(val: AutonegotiateSink) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AutonegotiateSink {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AutonegotiateSink");
        d.field("auto_neg_rdo_priority", &self.auto_neg_rdo_priority());
        d.field("no_usb_suspend", &self.no_usb_suspend());
        d.field("auto_compute_sink_min_power", &self.auto_compute_sink_min_power());
        d.field("no_capability_mismatch", &self.no_capability_mismatch());
        d.field("auto_compute_sink_min_voltage", &self.auto_compute_sink_min_voltage());
        d.field("auto_compute_sink_max_voltage", &self.auto_compute_sink_max_voltage());
        d.field(
            "auto_disable_sink_upon_capability_mismatch",
            &self.auto_disable_sink_upon_capability_mismatch(),
        );
        d.field(
            "auto_disable_input_for_sink_standby",
            &self.auto_disable_input_for_sink_standby(),
        );
        d.field(
            "auto_disable_input_for_sink_standby_in_dbm",
            &self.auto_disable_input_for_sink_standby_in_dbm(),
        );
        d.field("auto_enable_standby_srdy", &self.auto_enable_standby_srdy());
        d.field(
            "auto_enable_input_after_snk_ready_in_dbm",
            &self.auto_enable_input_after_snk_ready_in_dbm(),
        );
        d.field("auto_neg_max_current", &self.auto_neg_max_current());
        d.field(
            "auto_neg_sink_min_required_power",
            &self.auto_neg_sink_min_required_power(),
        );
        d.field("auto_neg_max_voltage", &self.auto_neg_max_voltage());
        d.field("auto_neg_min_voltage", &self.auto_neg_min_voltage());
        d.field(
            "auto_neg_capabilities_mismatch_power",
            &self.auto_neg_capabilities_mismatch_power(),
        );
        d.field("pps_enable_sink_mode", &self.pps_enable_sink_mode());
        d.field("pps_request_interval", &self.pps_request_interval());
        d.field("pps_source_operating_mode", &self.pps_source_operating_mode());
        d.field(
            "pps_required_full_voltage_range",
            &self.pps_required_full_voltage_range(),
        );
        d.field(
            "pps_disable_sink_upon_non_apdo_contract",
            &self.pps_disable_sink_upon_non_apdo_contract(),
        );
        d.field(
            "enable_sink_path_above_pps_sink_voltage_threshold",
            &self.enable_sink_path_above_pps_sink_voltage_threshold(),
        );
        d.field(
            "disable_sink_if_not_in_pps_contract",
            &self.disable_sink_if_not_in_pps_contract(),
        );
        d.field("pps_sink_voltage_threshold", &self.pps_sink_voltage_threshold());
        d.field("pps_operating_current", &self.pps_operating_current());
        d.field("pps_output_voltage", &self.pps_output_voltage());
        d.field("epr_avs_enable_sink_mode", &self.epr_avs_enable_sink_mode());
        d.field("epr_avs_operating_current", &self.epr_avs_operating_current());
        d.field("epr_avs_output_voltage", &self.epr_avs_output_voltage());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AutonegotiateSink {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AutonegotiateSink {{ ");
        defmt::write!(f, "auto_neg_rdo_priority: {}, ", & self.auto_neg_rdo_priority());
        defmt::write!(f, "no_usb_suspend: {=bool}, ", & self.no_usb_suspend());
        defmt::write!(
            f, "auto_compute_sink_min_power: {}, ", & self.auto_compute_sink_min_power()
        );
        defmt::write!(
            f, "no_capability_mismatch: {}, ", & self.no_capability_mismatch()
        );
        defmt::write!(
            f, "auto_compute_sink_min_voltage: {}, ", & self
            .auto_compute_sink_min_voltage()
        );
        defmt::write!(
            f, "auto_compute_sink_max_voltage: {}, ", & self
            .auto_compute_sink_max_voltage()
        );
        defmt::write!(
            f, "auto_disable_sink_upon_capability_mismatch: {=bool}, ", & self
            .auto_disable_sink_upon_capability_mismatch()
        );
        defmt::write!(
            f, "auto_disable_input_for_sink_standby: {=bool}, ", & self
            .auto_disable_input_for_sink_standby()
        );
        defmt::write!(
            f, "auto_disable_input_for_sink_standby_in_dbm: {=bool}, ", & self
            .auto_disable_input_for_sink_standby_in_dbm()
        );
        defmt::write!(
            f, "auto_enable_standby_srdy: {=bool}, ", & self.auto_enable_standby_srdy()
        );
        defmt::write!(
            f, "auto_enable_input_after_snk_ready_in_dbm: {=bool}, ", & self
            .auto_enable_input_after_snk_ready_in_dbm()
        );
        defmt::write!(
            f, "auto_neg_max_current: {=u16}, ", & self.auto_neg_max_current()
        );
        defmt::write!(
            f, "auto_neg_sink_min_required_power: {=u16}, ", & self
            .auto_neg_sink_min_required_power()
        );
        defmt::write!(
            f, "auto_neg_max_voltage: {=u16}, ", & self.auto_neg_max_voltage()
        );
        defmt::write!(
            f, "auto_neg_min_voltage: {=u16}, ", & self.auto_neg_min_voltage()
        );
        defmt::write!(
            f, "auto_neg_capabilities_mismatch_power: {=u16}, ", & self
            .auto_neg_capabilities_mismatch_power()
        );
        defmt::write!(
            f, "pps_enable_sink_mode: {=bool}, ", & self.pps_enable_sink_mode()
        );
        defmt::write!(f, "pps_request_interval: {}, ", & self.pps_request_interval());
        defmt::write!(
            f, "pps_source_operating_mode: {=bool}, ", & self.pps_source_operating_mode()
        );
        defmt::write!(
            f, "pps_required_full_voltage_range: {=bool}, ", & self
            .pps_required_full_voltage_range()
        );
        defmt::write!(
            f, "pps_disable_sink_upon_non_apdo_contract: {=bool}, ", & self
            .pps_disable_sink_upon_non_apdo_contract()
        );
        defmt::write!(
            f, "enable_sink_path_above_pps_sink_voltage_threshold: {=bool}, ", & self
            .enable_sink_path_above_pps_sink_voltage_threshold()
        );
        defmt::write!(
            f, "disable_sink_if_not_in_pps_contract: {=bool}, ", & self
            .disable_sink_if_not_in_pps_contract()
        );
        defmt::write!(
            f, "pps_sink_voltage_threshold: {=u16}, ", & self
            .pps_sink_voltage_threshold()
        );
        defmt::write!(
            f, "pps_operating_current: {=u8}, ", & self.pps_operating_current()
        );
        defmt::write!(f, "pps_output_voltage: {=u16}, ", & self.pps_output_voltage());
        defmt::write!(
            f, "epr_avs_enable_sink_mode: {=bool}, ", & self.epr_avs_enable_sink_mode()
        );
        defmt::write!(
            f, "epr_avs_operating_current: {=u8}, ", & self.epr_avs_operating_current()
        );
        defmt::write!(
            f, "epr_avs_output_voltage: {=u16}, ", & self.epr_avs_output_voltage()
        );
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AutonegotiateSink {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AutonegotiateSink {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AutonegotiateSink {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AutonegotiateSink {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AutonegotiateSink {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AutonegotiateSink {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AutonegotiateSink {
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
pub struct RxSrcCaps {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 45],
}
unsafe impl ::device_driver::Fieldset for RxSrcCaps {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 45] };
}
impl RxSrcCaps {
    /// `2:0` - Read the `num_valid_pdos` field.
    ///
    /// Number of valid standard PDOs
    #[doc(alias = "NumValidPdos")]
    #[must_use]
    pub fn num_valid_pdos(&self) -> u8 {
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
    /// `5:3` - Read the `num_valid_epr_pdos` field.
    ///
    /// Number of valid EPR PDOs
    #[doc(alias = "NumValidEprPdos")]
    #[must_use]
    pub fn num_valid_epr_pdos(&self) -> u8 {
        let start = 3;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 6` - Read the `last_src_cap_is_epr` field.
    ///
    /// Last source capabilities received is EPR
    #[doc(alias = "LastSrcCapIsEpr")]
    #[must_use]
    pub fn last_src_cap_is_epr(&self) -> bool {
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
    /// `39:8` - Read the `pdo_0` field.
    ///
    /// Standard PDO 0
    #[doc(alias = "Pdo0")]
    #[must_use]
    pub fn pdo_0(&self) -> u32 {
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
    /// `71:40` - Read the `pdo_1` field.
    ///
    /// Standard PDO 1
    #[doc(alias = "Pdo1")]
    #[must_use]
    pub fn pdo_1(&self) -> u32 {
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
    /// `103:72` - Read the `pdo_2` field.
    ///
    /// Standard PDO 2
    #[doc(alias = "Pdo2")]
    #[must_use]
    pub fn pdo_2(&self) -> u32 {
        let start = 72;
        let end = 103;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `135:104` - Read the `pdo_3` field.
    ///
    /// Standard PDO 3
    #[doc(alias = "Pdo3")]
    #[must_use]
    pub fn pdo_3(&self) -> u32 {
        let start = 104;
        let end = 135;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `167:136` - Read the `pdo_4` field.
    ///
    /// Standard PDO 4
    #[doc(alias = "Pdo4")]
    #[must_use]
    pub fn pdo_4(&self) -> u32 {
        let start = 136;
        let end = 167;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `199:168` - Read the `pdo_5` field.
    ///
    /// Standard PDO 5
    #[doc(alias = "Pdo5")]
    #[must_use]
    pub fn pdo_5(&self) -> u32 {
        let start = 168;
        let end = 199;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `231:200` - Read the `pdo_6` field.
    ///
    /// Standard PDO 6
    #[doc(alias = "Pdo6")]
    #[must_use]
    pub fn pdo_6(&self) -> u32 {
        let start = 200;
        let end = 231;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `263:232` - Read the `epr_pdo_0` field.
    ///
    /// EPR PDO 0
    #[doc(alias = "EprPdo0")]
    #[must_use]
    pub fn epr_pdo_0(&self) -> u32 {
        let start = 232;
        let end = 263;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `295:264` - Read the `epr_pdo_1` field.
    ///
    /// EPR PDO 1
    #[doc(alias = "EprPdo1")]
    #[must_use]
    pub fn epr_pdo_1(&self) -> u32 {
        let start = 264;
        let end = 295;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `327:296` - Read the `epr_pdo_2` field.
    ///
    /// EPR PDO 2
    #[doc(alias = "EprPdo2")]
    #[must_use]
    pub fn epr_pdo_2(&self) -> u32 {
        let start = 296;
        let end = 327;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `359:328` - Read the `epr_pdo_3` field.
    ///
    /// EPR PDO 3
    #[doc(alias = "EprPdo3")]
    #[must_use]
    pub fn epr_pdo_3(&self) -> u32 {
        let start = 328;
        let end = 359;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `2:0` - Set the `num_valid_pdos` field.
    ///
    /// Number of valid standard PDOs
    #[doc(alias = "NumValidPdos")]
    pub fn set_num_valid_pdos(&mut self, value: u8) {
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
    /// `5:3` - Set the `num_valid_epr_pdos` field.
    ///
    /// Number of valid EPR PDOs
    #[doc(alias = "NumValidEprPdos")]
    pub fn set_num_valid_epr_pdos(&mut self, value: u8) {
        let start = 3;
        let end = 5;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `last_src_cap_is_epr` field.
    ///
    /// Last source capabilities received is EPR
    #[doc(alias = "LastSrcCapIsEpr")]
    pub fn set_last_src_cap_is_epr(&mut self, value: bool) {
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
    /// `39:8` - Set the `pdo_0` field.
    ///
    /// Standard PDO 0
    #[doc(alias = "Pdo0")]
    pub fn set_pdo_0(&mut self, value: u32) {
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
    /// `71:40` - Set the `pdo_1` field.
    ///
    /// Standard PDO 1
    #[doc(alias = "Pdo1")]
    pub fn set_pdo_1(&mut self, value: u32) {
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
    /// `103:72` - Set the `pdo_2` field.
    ///
    /// Standard PDO 2
    #[doc(alias = "Pdo2")]
    pub fn set_pdo_2(&mut self, value: u32) {
        let start = 72;
        let end = 103;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `135:104` - Set the `pdo_3` field.
    ///
    /// Standard PDO 3
    #[doc(alias = "Pdo3")]
    pub fn set_pdo_3(&mut self, value: u32) {
        let start = 104;
        let end = 135;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `167:136` - Set the `pdo_4` field.
    ///
    /// Standard PDO 4
    #[doc(alias = "Pdo4")]
    pub fn set_pdo_4(&mut self, value: u32) {
        let start = 136;
        let end = 167;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `199:168` - Set the `pdo_5` field.
    ///
    /// Standard PDO 5
    #[doc(alias = "Pdo5")]
    pub fn set_pdo_5(&mut self, value: u32) {
        let start = 168;
        let end = 199;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `231:200` - Set the `pdo_6` field.
    ///
    /// Standard PDO 6
    #[doc(alias = "Pdo6")]
    pub fn set_pdo_6(&mut self, value: u32) {
        let start = 200;
        let end = 231;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `263:232` - Set the `epr_pdo_0` field.
    ///
    /// EPR PDO 0
    #[doc(alias = "EprPdo0")]
    pub fn set_epr_pdo_0(&mut self, value: u32) {
        let start = 232;
        let end = 263;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `295:264` - Set the `epr_pdo_1` field.
    ///
    /// EPR PDO 1
    #[doc(alias = "EprPdo1")]
    pub fn set_epr_pdo_1(&mut self, value: u32) {
        let start = 264;
        let end = 295;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `327:296` - Set the `epr_pdo_2` field.
    ///
    /// EPR PDO 2
    #[doc(alias = "EprPdo2")]
    pub fn set_epr_pdo_2(&mut self, value: u32) {
        let start = 296;
        let end = 327;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `359:328` - Set the `epr_pdo_3` field.
    ///
    /// EPR PDO 3
    #[doc(alias = "EprPdo3")]
    pub fn set_epr_pdo_3(&mut self, value: u32) {
        let start = 328;
        let end = 359;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for RxSrcCaps {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 45]> for RxSrcCaps {
    fn from(bits: [u8; 45]) -> Self {
        Self { bits }
    }
}
impl From<RxSrcCaps> for [u8; 45] {
    fn from(val: RxSrcCaps) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RxSrcCaps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RxSrcCaps");
        d.field("num_valid_pdos", &self.num_valid_pdos());
        d.field("num_valid_epr_pdos", &self.num_valid_epr_pdos());
        d.field("last_src_cap_is_epr", &self.last_src_cap_is_epr());
        d.field("pdo_0", &self.pdo_0());
        d.field("pdo_1", &self.pdo_1());
        d.field("pdo_2", &self.pdo_2());
        d.field("pdo_3", &self.pdo_3());
        d.field("pdo_4", &self.pdo_4());
        d.field("pdo_5", &self.pdo_5());
        d.field("pdo_6", &self.pdo_6());
        d.field("epr_pdo_0", &self.epr_pdo_0());
        d.field("epr_pdo_1", &self.epr_pdo_1());
        d.field("epr_pdo_2", &self.epr_pdo_2());
        d.field("epr_pdo_3", &self.epr_pdo_3());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxSrcCaps {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxSrcCaps {{ ");
        defmt::write!(f, "num_valid_pdos: {=u8}, ", & self.num_valid_pdos());
        defmt::write!(f, "num_valid_epr_pdos: {=u8}, ", & self.num_valid_epr_pdos());
        defmt::write!(f, "last_src_cap_is_epr: {=bool}, ", & self.last_src_cap_is_epr());
        defmt::write!(f, "pdo_0: {=u32}, ", & self.pdo_0());
        defmt::write!(f, "pdo_1: {=u32}, ", & self.pdo_1());
        defmt::write!(f, "pdo_2: {=u32}, ", & self.pdo_2());
        defmt::write!(f, "pdo_3: {=u32}, ", & self.pdo_3());
        defmt::write!(f, "pdo_4: {=u32}, ", & self.pdo_4());
        defmt::write!(f, "pdo_5: {=u32}, ", & self.pdo_5());
        defmt::write!(f, "pdo_6: {=u32}, ", & self.pdo_6());
        defmt::write!(f, "epr_pdo_0: {=u32}, ", & self.epr_pdo_0());
        defmt::write!(f, "epr_pdo_1: {=u32}, ", & self.epr_pdo_1());
        defmt::write!(f, "epr_pdo_2: {=u32}, ", & self.epr_pdo_2());
        defmt::write!(f, "epr_pdo_3: {=u32}, ", & self.epr_pdo_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RxSrcCaps {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RxSrcCaps {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RxSrcCaps {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RxSrcCaps {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RxSrcCaps {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RxSrcCaps {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RxSrcCaps {
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
pub struct BootFlags {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 44],
}
unsafe impl ::device_driver::Fieldset for BootFlags {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 44] };
}
impl BootFlags {
    /// `3:0` - Read the `boot_stage` field.
    ///
    /// Current boot stage
    #[doc(alias = "BootStage")]
    #[must_use]
    pub fn boot_stage(&self) -> u8 {
        let start = 0;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `65:64` - Read the `total_num_pps` field.
    ///
    /// Number of ports
    #[doc(alias = "TotalNumPps")]
    #[must_use]
    pub fn total_num_pps(&self) -> u8 {
        let start = 64;
        let end = 65;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 66` - Read the `is_ext_pp_present` field.
    ///
    /// 1 if external power path is present
    #[doc(alias = "IsExtPpPresent")]
    #[must_use]
    pub fn is_ext_pp_present(&self) -> bool {
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
    /// `bit 128` - Read the `dead_battery_flag` field.
    ///
    /// Dead battery flag
    #[doc(alias = "DeadBatteryFlag")]
    #[must_use]
    pub fn dead_battery_flag(&self) -> bool {
        let start = 128;
        let end = 128;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 129` - Read the `db_port_b_power_provider` field.
    ///
    /// Dead battery, port B is power provider
    #[doc(alias = "DbPortBPowerProvider")]
    #[must_use]
    pub fn db_port_b_power_provider(&self) -> bool {
        let start = 129;
        let end = 129;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 130` - Read the `db_port_a_power_provider` field.
    ///
    /// Dead battery, port A is power provider
    #[doc(alias = "DbPortAPowerProvider")]
    #[must_use]
    pub fn db_port_a_power_provider(&self) -> bool {
        let start = 130;
        let end = 130;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 131` - Read the `port_a_sink_switch` field.
    ///
    /// Port A sink switch is enabled
    #[doc(alias = "PortASinkSwitch")]
    #[must_use]
    pub fn port_a_sink_switch(&self) -> bool {
        let start = 131;
        let end = 131;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 132` - Read the `port_b_sink_switch` field.
    ///
    /// Port B sink switch is enabled
    #[doc(alias = "PortBSinkSwitch")]
    #[must_use]
    pub fn port_b_sink_switch(&self) -> bool {
        let start = 132;
        let end = 132;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `167:160` - Read the `port_a_i_2_c_1_trgt_addr` field.
    ///
    /// Port A I2C1 Target Address
    #[doc(alias = "PortA_I2c1TrgtAddr")]
    #[must_use]
    pub fn port_a_i_2_c_1_trgt_addr(&self) -> u8 {
        let start = 160;
        let end = 167;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `175:168` - Read the `port_b_i_2_c_1_trgt_addr` field.
    ///
    /// Port B I2C1 Target Address
    #[doc(alias = "PortB_I2c1TrgtAddr")]
    #[must_use]
    pub fn port_b_i_2_c_1_trgt_addr(&self) -> u8 {
        let start = 168;
        let end = 175;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `183:176` - Read the `port_a_i_2_c_2_trgt_addr` field.
    ///
    /// Port A I2C2 Target Address
    #[doc(alias = "PortA_I2c2TrgtAddr")]
    #[must_use]
    pub fn port_a_i_2_c_2_trgt_addr(&self) -> u8 {
        let start = 176;
        let end = 183;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `191:184` - Read the `port_b_i_2_c_2_trgt_addr` field.
    ///
    /// Port B I2C2 Target Address
    #[doc(alias = "PortB_I2c2TrgtAddr")]
    #[must_use]
    pub fn port_b_i_2_c_2_trgt_addr(&self) -> u8 {
        let start = 184;
        let end = 191;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `199:192` - Read the `port_a_i_2_c_4_trgt_addr` field.
    ///
    /// Port A I2C4 Target Address
    #[doc(alias = "PortA_I2c4TrgtAddr")]
    #[must_use]
    pub fn port_a_i_2_c_4_trgt_addr(&self) -> u8 {
        let start = 192;
        let end = 199;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `207:200` - Read the `port_b_i_2_c_4_trgt_addr` field.
    ///
    /// Port B I2C4 Target Address
    #[doc(alias = "PortB_I2c4TrgtAddr")]
    #[must_use]
    pub fn port_b_i_2_c_4_trgt_addr(&self) -> u8 {
        let start = 200;
        let end = 207;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `225:224` - Read the `active_bank` field.
    ///
    /// Active Bank the device booted from
    #[doc(alias = "ActiveBank")]
    #[must_use]
    pub fn active_bank(&self) -> u8 {
        let start = 224;
        let end = 225;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 226` - Read the `bank_0_valid` field.
    ///
    /// Asserted 1 if Bank 0 has valid Application Code
    #[doc(alias = "Bank0Valid")]
    #[must_use]
    pub fn bank_0_valid(&self) -> bool {
        let start = 226;
        let end = 226;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 227` - Read the `bank_1_valid` field.
    ///
    /// Asserted 1 if Bank 1 has valid Application Code
    #[doc(alias = "Bank1Valid")]
    #[must_use]
    pub fn bank_1_valid(&self) -> bool {
        let start = 227;
        let end = 227;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `287:256` - Read the `bank_0_fw_version` field.
    ///
    /// Application Firmware Version in Bank 0
    #[doc(alias = "Bank0FwVersion")]
    #[must_use]
    pub fn bank_0_fw_version(&self) -> u32 {
        let start = 256;
        let end = 287;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `319:288` - Read the `bank_1_fw_version` field.
    ///
    /// Application Firmware Version in Bank 1
    #[doc(alias = "Bank1FwVersion")]
    #[must_use]
    pub fn bank_1_fw_version(&self) -> u32 {
        let start = 288;
        let end = 319;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u32,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `335:320` - Read the `adc_in_value` field.
    ///
    /// Raw ADCIN Value
    #[doc(alias = "AdcInValue")]
    #[must_use]
    pub fn adc_in_value(&self) -> u16 {
        let start = 320;
        let end = 335;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `351:336` - Read the `adc_in_index` field.
    ///
    /// ADCIN Index
    #[doc(alias = "AdcInIndex")]
    #[must_use]
    pub fn adc_in_index(&self) -> u16 {
        let start = 336;
        let end = 351;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `3:0` - Set the `boot_stage` field.
    ///
    /// Current boot stage
    #[doc(alias = "BootStage")]
    pub fn set_boot_stage(&mut self, value: u8) {
        let start = 0;
        let end = 3;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `65:64` - Set the `total_num_pps` field.
    ///
    /// Number of ports
    #[doc(alias = "TotalNumPps")]
    pub fn set_total_num_pps(&mut self, value: u8) {
        let start = 64;
        let end = 65;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 66` - Set the `is_ext_pp_present` field.
    ///
    /// 1 if external power path is present
    #[doc(alias = "IsExtPpPresent")]
    pub fn set_is_ext_pp_present(&mut self, value: bool) {
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
    /// `bit 128` - Set the `dead_battery_flag` field.
    ///
    /// Dead battery flag
    #[doc(alias = "DeadBatteryFlag")]
    pub fn set_dead_battery_flag(&mut self, value: bool) {
        let start = 128;
        let end = 128;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 129` - Set the `db_port_b_power_provider` field.
    ///
    /// Dead battery, port B is power provider
    #[doc(alias = "DbPortBPowerProvider")]
    pub fn set_db_port_b_power_provider(&mut self, value: bool) {
        let start = 129;
        let end = 129;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 130` - Set the `db_port_a_power_provider` field.
    ///
    /// Dead battery, port A is power provider
    #[doc(alias = "DbPortAPowerProvider")]
    pub fn set_db_port_a_power_provider(&mut self, value: bool) {
        let start = 130;
        let end = 130;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 131` - Set the `port_a_sink_switch` field.
    ///
    /// Port A sink switch is enabled
    #[doc(alias = "PortASinkSwitch")]
    pub fn set_port_a_sink_switch(&mut self, value: bool) {
        let start = 131;
        let end = 131;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 132` - Set the `port_b_sink_switch` field.
    ///
    /// Port B sink switch is enabled
    #[doc(alias = "PortBSinkSwitch")]
    pub fn set_port_b_sink_switch(&mut self, value: bool) {
        let start = 132;
        let end = 132;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `167:160` - Set the `port_a_i_2_c_1_trgt_addr` field.
    ///
    /// Port A I2C1 Target Address
    #[doc(alias = "PortA_I2c1TrgtAddr")]
    pub fn set_port_a_i_2_c_1_trgt_addr(&mut self, value: u8) {
        let start = 160;
        let end = 167;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `175:168` - Set the `port_b_i_2_c_1_trgt_addr` field.
    ///
    /// Port B I2C1 Target Address
    #[doc(alias = "PortB_I2c1TrgtAddr")]
    pub fn set_port_b_i_2_c_1_trgt_addr(&mut self, value: u8) {
        let start = 168;
        let end = 175;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `183:176` - Set the `port_a_i_2_c_2_trgt_addr` field.
    ///
    /// Port A I2C2 Target Address
    #[doc(alias = "PortA_I2c2TrgtAddr")]
    pub fn set_port_a_i_2_c_2_trgt_addr(&mut self, value: u8) {
        let start = 176;
        let end = 183;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `191:184` - Set the `port_b_i_2_c_2_trgt_addr` field.
    ///
    /// Port B I2C2 Target Address
    #[doc(alias = "PortB_I2c2TrgtAddr")]
    pub fn set_port_b_i_2_c_2_trgt_addr(&mut self, value: u8) {
        let start = 184;
        let end = 191;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `199:192` - Set the `port_a_i_2_c_4_trgt_addr` field.
    ///
    /// Port A I2C4 Target Address
    #[doc(alias = "PortA_I2c4TrgtAddr")]
    pub fn set_port_a_i_2_c_4_trgt_addr(&mut self, value: u8) {
        let start = 192;
        let end = 199;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `207:200` - Set the `port_b_i_2_c_4_trgt_addr` field.
    ///
    /// Port B I2C4 Target Address
    #[doc(alias = "PortB_I2c4TrgtAddr")]
    pub fn set_port_b_i_2_c_4_trgt_addr(&mut self, value: u8) {
        let start = 200;
        let end = 207;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `225:224` - Set the `active_bank` field.
    ///
    /// Active Bank the device booted from
    #[doc(alias = "ActiveBank")]
    pub fn set_active_bank(&mut self, value: u8) {
        let start = 224;
        let end = 225;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 226` - Set the `bank_0_valid` field.
    ///
    /// Asserted 1 if Bank 0 has valid Application Code
    #[doc(alias = "Bank0Valid")]
    pub fn set_bank_0_valid(&mut self, value: bool) {
        let start = 226;
        let end = 226;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 227` - Set the `bank_1_valid` field.
    ///
    /// Asserted 1 if Bank 1 has valid Application Code
    #[doc(alias = "Bank1Valid")]
    pub fn set_bank_1_valid(&mut self, value: bool) {
        let start = 227;
        let end = 227;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `287:256` - Set the `bank_0_fw_version` field.
    ///
    /// Application Firmware Version in Bank 0
    #[doc(alias = "Bank0FwVersion")]
    pub fn set_bank_0_fw_version(&mut self, value: u32) {
        let start = 256;
        let end = 287;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `319:288` - Set the `bank_1_fw_version` field.
    ///
    /// Application Firmware Version in Bank 1
    #[doc(alias = "Bank1FwVersion")]
    pub fn set_bank_1_fw_version(&mut self, value: u32) {
        let start = 288;
        let end = 319;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u32,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `335:320` - Set the `adc_in_value` field.
    ///
    /// Raw ADCIN Value
    #[doc(alias = "AdcInValue")]
    pub fn set_adc_in_value(&mut self, value: u16) {
        let start = 320;
        let end = 335;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `351:336` - Set the `adc_in_index` field.
    ///
    /// ADCIN Index
    #[doc(alias = "AdcInIndex")]
    pub fn set_adc_in_index(&mut self, value: u16) {
        let start = 336;
        let end = 351;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for BootFlags {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 44]> for BootFlags {
    fn from(bits: [u8; 44]) -> Self {
        Self { bits }
    }
}
impl From<BootFlags> for [u8; 44] {
    fn from(val: BootFlags) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BootFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BootFlags");
        d.field("boot_stage", &self.boot_stage());
        d.field("total_num_pps", &self.total_num_pps());
        d.field("is_ext_pp_present", &self.is_ext_pp_present());
        d.field("dead_battery_flag", &self.dead_battery_flag());
        d.field("db_port_b_power_provider", &self.db_port_b_power_provider());
        d.field("db_port_a_power_provider", &self.db_port_a_power_provider());
        d.field("port_a_sink_switch", &self.port_a_sink_switch());
        d.field("port_b_sink_switch", &self.port_b_sink_switch());
        d.field("port_a_i_2_c_1_trgt_addr", &self.port_a_i_2_c_1_trgt_addr());
        d.field("port_b_i_2_c_1_trgt_addr", &self.port_b_i_2_c_1_trgt_addr());
        d.field("port_a_i_2_c_2_trgt_addr", &self.port_a_i_2_c_2_trgt_addr());
        d.field("port_b_i_2_c_2_trgt_addr", &self.port_b_i_2_c_2_trgt_addr());
        d.field("port_a_i_2_c_4_trgt_addr", &self.port_a_i_2_c_4_trgt_addr());
        d.field("port_b_i_2_c_4_trgt_addr", &self.port_b_i_2_c_4_trgt_addr());
        d.field("active_bank", &self.active_bank());
        d.field("bank_0_valid", &self.bank_0_valid());
        d.field("bank_1_valid", &self.bank_1_valid());
        d.field("bank_0_fw_version", &self.bank_0_fw_version());
        d.field("bank_1_fw_version", &self.bank_1_fw_version());
        d.field("adc_in_value", &self.adc_in_value());
        d.field("adc_in_index", &self.adc_in_index());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BootFlags {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BootFlags {{ ");
        defmt::write!(f, "boot_stage: {=u8}, ", & self.boot_stage());
        defmt::write!(f, "total_num_pps: {=u8}, ", & self.total_num_pps());
        defmt::write!(f, "is_ext_pp_present: {=bool}, ", & self.is_ext_pp_present());
        defmt::write!(f, "dead_battery_flag: {=bool}, ", & self.dead_battery_flag());
        defmt::write!(
            f, "db_port_b_power_provider: {=bool}, ", & self.db_port_b_power_provider()
        );
        defmt::write!(
            f, "db_port_a_power_provider: {=bool}, ", & self.db_port_a_power_provider()
        );
        defmt::write!(f, "port_a_sink_switch: {=bool}, ", & self.port_a_sink_switch());
        defmt::write!(f, "port_b_sink_switch: {=bool}, ", & self.port_b_sink_switch());
        defmt::write!(
            f, "port_a_i_2_c_1_trgt_addr: {=u8}, ", & self.port_a_i_2_c_1_trgt_addr()
        );
        defmt::write!(
            f, "port_b_i_2_c_1_trgt_addr: {=u8}, ", & self.port_b_i_2_c_1_trgt_addr()
        );
        defmt::write!(
            f, "port_a_i_2_c_2_trgt_addr: {=u8}, ", & self.port_a_i_2_c_2_trgt_addr()
        );
        defmt::write!(
            f, "port_b_i_2_c_2_trgt_addr: {=u8}, ", & self.port_b_i_2_c_2_trgt_addr()
        );
        defmt::write!(
            f, "port_a_i_2_c_4_trgt_addr: {=u8}, ", & self.port_a_i_2_c_4_trgt_addr()
        );
        defmt::write!(
            f, "port_b_i_2_c_4_trgt_addr: {=u8}, ", & self.port_b_i_2_c_4_trgt_addr()
        );
        defmt::write!(f, "active_bank: {=u8}, ", & self.active_bank());
        defmt::write!(f, "bank_0_valid: {=bool}, ", & self.bank_0_valid());
        defmt::write!(f, "bank_1_valid: {=bool}, ", & self.bank_1_valid());
        defmt::write!(f, "bank_0_fw_version: {=u32}, ", & self.bank_0_fw_version());
        defmt::write!(f, "bank_1_fw_version: {=u32}, ", & self.bank_1_fw_version());
        defmt::write!(f, "adc_in_value: {=u16}, ", & self.adc_in_value());
        defmt::write!(f, "adc_in_index: {=u16}, ", & self.adc_in_index());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BootFlags {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BootFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BootFlags {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BootFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BootFlags {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BootFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BootFlags {
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
pub struct PortConfig {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 18],
}
unsafe impl ::device_driver::Fieldset for PortConfig {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 18] };
}
impl PortConfig {
    /// `1:0` - Read the `typec_state_machine` field.
    ///
    /// Type-C State machine
    #[doc(alias = "TypecStateMachine")]
    #[must_use]
    pub fn typec_state_machine(&self) -> TypeCStateMachine {
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
    /// `bit 2` - Read the `crossbar_type` field.
    ///
    /// Crossbar Type
    #[doc(alias = "CrossbarType")]
    #[must_use]
    pub fn crossbar_type(&self) -> CrossbarType {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 7` - Read the `pp_ext_active_low` field.
    ///
    /// PP_EXT ActiveLow
    #[doc(alias = "PpExtActiveLow")]
    #[must_use]
    pub fn pp_ext_active_low(&self) -> bool {
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
    /// `9:8` - Read the `typec_support_options` field.
    ///
    /// TypeC Support Options
    #[doc(alias = "TypecSupportOptions")]
    #[must_use]
    pub fn typec_support_options(&self) -> TypeCSupportOptions {
        let start = 8;
        let end = 9;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 10` - Read the `disable_pd` field.
    ///
    /// Disable PD
    #[doc(alias = "DisablePd")]
    #[must_use]
    pub fn disable_pd(&self) -> bool {
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
    /// `bit 11` - Read the `usb_communication_capable` field.
    ///
    /// USB Communication Capable
    #[doc(alias = "UsbCommunicationCapable")]
    #[must_use]
    pub fn usb_communication_capable(&self) -> bool {
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
    /// `bit 12` - Read the `debug_accessory_support` field.
    ///
    /// DebugAccessory Support
    #[doc(alias = "DebugAccessorySupport")]
    #[must_use]
    pub fn debug_accessory_support(&self) -> bool {
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
    /// `14:13` - Read the `usb_3_rate` field.
    ///
    /// USB3 Rate
    #[doc(alias = "Usb3Rate")]
    #[must_use]
    pub fn usb_3_rate(&self) -> Usb3Rate {
        let start = 13;
        let end = 14;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 15` - Read the `crossbar_i_2_c_controller_enable` field.
    ///
    /// Crossbar I2C Controller enable
    #[doc(alias = "CrossbarI2cControllerEnable")]
    #[must_use]
    pub fn crossbar_i_2_c_controller_enable(&self) -> bool {
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
    /// `17:16` - Read the `vbus_ovp_usage` field.
    ///
    /// VBUS OVP Usage
    #[doc(alias = "VbusOvpUsage")]
    #[must_use]
    pub fn vbus_ovp_usage(&self) -> VbusOvpUsage {
        let start = 16;
        let end = 17;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `21:20` - Read the `ovp_for_pp_5_v` field.
    ///
    /// OVP for PP5V
    #[doc(alias = "OvpForPp5v")]
    #[must_use]
    pub fn ovp_for_pp_5_v(&self) -> OvpForPp5V {
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
    /// `bit 22` - Read the `crossbar_config_type_1_extended` field.
    ///
    /// Crossbar Config Type 1 Extended
    #[doc(alias = "CrossbarConfigType1Extended")]
    #[must_use]
    pub fn crossbar_config_type_1_extended(&self) -> bool {
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
    /// `bit 23` - Read the `remove_safe_state_between_usb_3_to_dp_transition` field.
    ///
    /// Remove Safe-State Between USB3-to-DP Transition
    #[doc(alias = "RemoveSafeStateBetweenUsb3ToDpTransition")]
    #[must_use]
    pub fn remove_safe_state_between_usb_3_to_dp_transition(&self) -> bool {
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
    /// `26:24` - Read the `vbus_sink_uvp_trip_hv` field.
    ///
    /// VBUS Sink UVP Trip HV
    #[doc(alias = "VbusSinkUvpTripHv")]
    #[must_use]
    pub fn vbus_sink_uvp_trip_hv(&self) -> VbusSinkUvpTripHv {
        let start = 24;
        let end = 26;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `28:27` - Read the `apdo_vbus_uvp_threshold` field.
    ///
    /// APDO VBUS UVP Threshold
    #[doc(alias = "ApdoVbusUvpThreshold")]
    #[must_use]
    pub fn apdo_vbus_uvp_threshold(&self) -> u8 {
        let start = 27;
        let end = 28;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `30:29` - Read the `apdo_ilim_over_shoot` field.
    ///
    /// APDO ILIM Over Shoot
    #[doc(alias = "ApdoIlimOverShoot")]
    #[must_use]
    pub fn apdo_ilim_over_shoot(&self) -> u8 {
        let start = 29;
        let end = 30;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `47:32` - Read the `apdo_vbus_uvp_trip_point_offset` field.
    ///
    /// APDO VBUS Uvp TripPoint Offset
    #[doc(alias = "ApdoVbusUvpTripPointOffset")]
    #[must_use]
    pub fn apdo_vbus_uvp_trip_point_offset(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `63:48` - Read the `vbus_for_valid_pps_status` field.
    ///
    /// VBUS For Valid PPS Status
    #[doc(alias = "VbusForValidPpsStatus")]
    #[must_use]
    pub fn vbus_for_valid_pps_status(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `71:64` - Read the `external_dcdc_type` field.
    ///
    /// External DCDC Type
    #[doc(alias = "ExternalDcdcType")]
    #[must_use]
    pub fn external_dcdc_type(&self) -> ExternalDcdcType {
        let start = 64;
        let end = 71;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 72` - Read the `sink_mode_i_2_c_irq_config` field.
    ///
    /// Enables support for GPIO10 to I2C IRQ
    #[doc(alias = "SinkModeI2cIrqConfig")]
    #[must_use]
    pub fn sink_mode_i_2_c_irq_config(&self) -> bool {
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
    /// `95:80` - Read the `gt_threshold_voltage` field.
    ///
    /// Threshold voltage to trigger the GREATER_THAN_THRESHOLD_VOLTAGE GPIO Event (50mV per LSB)
    #[doc(alias = "GtThresholdVoltage")]
    #[must_use]
    pub fn gt_threshold_voltage(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 124` - Read the `enable_internal_auxbiasing` field.
    ///
    /// Enable Internal Auxbiasing
    #[doc(alias = "EnableInternalAuxbiasing")]
    #[must_use]
    pub fn enable_internal_auxbiasing(&self) -> bool {
        let start = 124;
        let end = 124;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 125` - Read the `enable_internal_level_shifter` field.
    ///
    /// EnableInternalLevelShifter
    #[doc(alias = "EnableInternalLevelShifter")]
    #[must_use]
    pub fn enable_internal_level_shifter(&self) -> bool {
        let start = 125;
        let end = 125;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `127:126` - Read the `level_shifter_direction_cfg` field.
    ///
    /// Level shifter direction configuration
    #[doc(alias = "LevelShifterDirectionCfg")]
    #[must_use]
    pub fn level_shifter_direction_cfg(&self) -> LevelShifterDirectionCfg {
        let start = 126;
        let end = 127;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `130:128` - Read the `sbu_mux_debug_setting` field.
    ///
    /// SBU Mux Debug Setting
    #[doc(alias = "SbuMuxDebugSetting")]
    #[must_use]
    pub fn sbu_mux_debug_setting(&self) -> SbuMuxDebugSetting {
        let start = 128;
        let end = 130;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `133:131` - Read the `sbu_mux_default_setting` field.
    ///
    /// SBU Mux Default Setting
    #[doc(alias = "SbuMuxDefaultSetting")]
    #[must_use]
    pub fn sbu_mux_default_setting(&self) -> SbuMuxDefaultSetting {
        let start = 131;
        let end = 133;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `135:134` - Read the `sbu_mux_usage` field.
    ///
    /// Sbu Mux Usage
    #[doc(alias = "SbuMuxUsage")]
    #[must_use]
    pub fn sbu_mux_usage(&self) -> SbuMuxUsage {
        let start = 134;
        let end = 135;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw.into()
    }
    /// `bit 136` - Read the `epr_supported_as_source` field.
    ///
    /// EPR Supported as Source
    #[doc(alias = "EprSupportedAsSource")]
    #[must_use]
    pub fn epr_supported_as_source(&self) -> bool {
        let start = 136;
        let end = 136;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 137` - Read the `epr_supported_as_sink` field.
    ///
    /// EPR Supported as Sink
    #[doc(alias = "EprSupportedAsSink")]
    #[must_use]
    pub fn epr_supported_as_sink(&self) -> bool {
        let start = 137;
        let end = 137;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 138` - Read the `disable_sourcing_in_dbm` field.
    ///
    /// Disable sourcing in dead battery mode
    #[doc(alias = "DisableSourcingInDbm")]
    #[must_use]
    pub fn disable_sourcing_in_dbm(&self) -> bool {
        let start = 138;
        let end = 138;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 139` - Read the `flip_crossbar_dbg_setting` field.
    ///
    /// Flip Px_DBG
    #[doc(alias = "FlipCrossbarDbgSetting")]
    #[must_use]
    pub fn flip_crossbar_dbg_setting(&self) -> bool {
        let start = 139;
        let end = 139;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 140` - Read the `flip_crossbar_aux_setting` field.
    ///
    /// Flip Px_AUX
    #[doc(alias = "FlipCrossbarAuxSetting")]
    #[must_use]
    pub fn flip_crossbar_aux_setting(&self) -> bool {
        let start = 140;
        let end = 140;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 141` - Read the `flip_crossbar_sbtx_setting` field.
    ///
    /// Flip PX_SBTX/RX
    #[doc(alias = "FlipCrossbarSbtxSetting")]
    #[must_use]
    pub fn flip_crossbar_sbtx_setting(&self) -> bool {
        let start = 141;
        let end = 141;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `1:0` - Set the `typec_state_machine` field.
    ///
    /// Type-C State machine
    #[doc(alias = "TypecStateMachine")]
    pub fn set_typec_state_machine(&mut self, value: TypeCStateMachine) {
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
    /// `bit 2` - Set the `crossbar_type` field.
    ///
    /// Crossbar Type
    #[doc(alias = "CrossbarType")]
    pub fn set_crossbar_type(&mut self, value: CrossbarType) {
        let start = 2;
        let end = 2;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `pp_ext_active_low` field.
    ///
    /// PP_EXT ActiveLow
    #[doc(alias = "PpExtActiveLow")]
    pub fn set_pp_ext_active_low(&mut self, value: bool) {
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
    /// `9:8` - Set the `typec_support_options` field.
    ///
    /// TypeC Support Options
    #[doc(alias = "TypecSupportOptions")]
    pub fn set_typec_support_options(&mut self, value: TypeCSupportOptions) {
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
    /// `bit 10` - Set the `disable_pd` field.
    ///
    /// Disable PD
    #[doc(alias = "DisablePd")]
    pub fn set_disable_pd(&mut self, value: bool) {
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
    /// `bit 11` - Set the `usb_communication_capable` field.
    ///
    /// USB Communication Capable
    #[doc(alias = "UsbCommunicationCapable")]
    pub fn set_usb_communication_capable(&mut self, value: bool) {
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
    /// `bit 12` - Set the `debug_accessory_support` field.
    ///
    /// DebugAccessory Support
    #[doc(alias = "DebugAccessorySupport")]
    pub fn set_debug_accessory_support(&mut self, value: bool) {
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
    /// `14:13` - Set the `usb_3_rate` field.
    ///
    /// USB3 Rate
    #[doc(alias = "Usb3Rate")]
    pub fn set_usb_3_rate(&mut self, value: Usb3Rate) {
        let start = 13;
        let end = 14;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 15` - Set the `crossbar_i_2_c_controller_enable` field.
    ///
    /// Crossbar I2C Controller enable
    #[doc(alias = "CrossbarI2cControllerEnable")]
    pub fn set_crossbar_i_2_c_controller_enable(&mut self, value: bool) {
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
    /// `17:16` - Set the `vbus_ovp_usage` field.
    ///
    /// VBUS OVP Usage
    #[doc(alias = "VbusOvpUsage")]
    pub fn set_vbus_ovp_usage(&mut self, value: VbusOvpUsage) {
        let start = 16;
        let end = 17;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `21:20` - Set the `ovp_for_pp_5_v` field.
    ///
    /// OVP for PP5V
    #[doc(alias = "OvpForPp5v")]
    pub fn set_ovp_for_pp_5_v(&mut self, value: OvpForPp5V) {
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
    /// `bit 22` - Set the `crossbar_config_type_1_extended` field.
    ///
    /// Crossbar Config Type 1 Extended
    #[doc(alias = "CrossbarConfigType1Extended")]
    pub fn set_crossbar_config_type_1_extended(&mut self, value: bool) {
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
    /// `bit 23` - Set the `remove_safe_state_between_usb_3_to_dp_transition` field.
    ///
    /// Remove Safe-State Between USB3-to-DP Transition
    #[doc(alias = "RemoveSafeStateBetweenUsb3ToDpTransition")]
    pub fn set_remove_safe_state_between_usb_3_to_dp_transition(&mut self, value: bool) {
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
    /// `26:24` - Set the `vbus_sink_uvp_trip_hv` field.
    ///
    /// VBUS Sink UVP Trip HV
    #[doc(alias = "VbusSinkUvpTripHv")]
    pub fn set_vbus_sink_uvp_trip_hv(&mut self, value: VbusSinkUvpTripHv) {
        let start = 24;
        let end = 26;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `28:27` - Set the `apdo_vbus_uvp_threshold` field.
    ///
    /// APDO VBUS UVP Threshold
    #[doc(alias = "ApdoVbusUvpThreshold")]
    pub fn set_apdo_vbus_uvp_threshold(&mut self, value: u8) {
        let start = 27;
        let end = 28;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `30:29` - Set the `apdo_ilim_over_shoot` field.
    ///
    /// APDO ILIM Over Shoot
    #[doc(alias = "ApdoIlimOverShoot")]
    pub fn set_apdo_ilim_over_shoot(&mut self, value: u8) {
        let start = 29;
        let end = 30;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `47:32` - Set the `apdo_vbus_uvp_trip_point_offset` field.
    ///
    /// APDO VBUS Uvp TripPoint Offset
    #[doc(alias = "ApdoVbusUvpTripPointOffset")]
    pub fn set_apdo_vbus_uvp_trip_point_offset(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `63:48` - Set the `vbus_for_valid_pps_status` field.
    ///
    /// VBUS For Valid PPS Status
    #[doc(alias = "VbusForValidPpsStatus")]
    pub fn set_vbus_for_valid_pps_status(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `71:64` - Set the `external_dcdc_type` field.
    ///
    /// External DCDC Type
    #[doc(alias = "ExternalDcdcType")]
    pub fn set_external_dcdc_type(&mut self, value: ExternalDcdcType) {
        let start = 64;
        let end = 71;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 72` - Set the `sink_mode_i_2_c_irq_config` field.
    ///
    /// Enables support for GPIO10 to I2C IRQ
    #[doc(alias = "SinkModeI2cIrqConfig")]
    pub fn set_sink_mode_i_2_c_irq_config(&mut self, value: bool) {
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
    /// `95:80` - Set the `gt_threshold_voltage` field.
    ///
    /// Threshold voltage to trigger the GREATER_THAN_THRESHOLD_VOLTAGE GPIO Event (50mV per LSB)
    #[doc(alias = "GtThresholdVoltage")]
    pub fn set_gt_threshold_voltage(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 124` - Set the `enable_internal_auxbiasing` field.
    ///
    /// Enable Internal Auxbiasing
    #[doc(alias = "EnableInternalAuxbiasing")]
    pub fn set_enable_internal_auxbiasing(&mut self, value: bool) {
        let start = 124;
        let end = 124;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 125` - Set the `enable_internal_level_shifter` field.
    ///
    /// EnableInternalLevelShifter
    #[doc(alias = "EnableInternalLevelShifter")]
    pub fn set_enable_internal_level_shifter(&mut self, value: bool) {
        let start = 125;
        let end = 125;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `127:126` - Set the `level_shifter_direction_cfg` field.
    ///
    /// Level shifter direction configuration
    #[doc(alias = "LevelShifterDirectionCfg")]
    pub fn set_level_shifter_direction_cfg(&mut self, value: LevelShifterDirectionCfg) {
        let start = 126;
        let end = 127;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `130:128` - Set the `sbu_mux_debug_setting` field.
    ///
    /// SBU Mux Debug Setting
    #[doc(alias = "SbuMuxDebugSetting")]
    pub fn set_sbu_mux_debug_setting(&mut self, value: SbuMuxDebugSetting) {
        let start = 128;
        let end = 130;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `133:131` - Set the `sbu_mux_default_setting` field.
    ///
    /// SBU Mux Default Setting
    #[doc(alias = "SbuMuxDefaultSetting")]
    pub fn set_sbu_mux_default_setting(&mut self, value: SbuMuxDefaultSetting) {
        let start = 131;
        let end = 133;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `135:134` - Set the `sbu_mux_usage` field.
    ///
    /// Sbu Mux Usage
    #[doc(alias = "SbuMuxUsage")]
    pub fn set_sbu_mux_usage(&mut self, value: SbuMuxUsage) {
        let start = 134;
        let end = 135;
        let raw = value.into();
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 136` - Set the `epr_supported_as_source` field.
    ///
    /// EPR Supported as Source
    #[doc(alias = "EprSupportedAsSource")]
    pub fn set_epr_supported_as_source(&mut self, value: bool) {
        let start = 136;
        let end = 136;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 137` - Set the `epr_supported_as_sink` field.
    ///
    /// EPR Supported as Sink
    #[doc(alias = "EprSupportedAsSink")]
    pub fn set_epr_supported_as_sink(&mut self, value: bool) {
        let start = 137;
        let end = 137;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 138` - Set the `disable_sourcing_in_dbm` field.
    ///
    /// Disable sourcing in dead battery mode
    #[doc(alias = "DisableSourcingInDbm")]
    pub fn set_disable_sourcing_in_dbm(&mut self, value: bool) {
        let start = 138;
        let end = 138;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 139` - Set the `flip_crossbar_dbg_setting` field.
    ///
    /// Flip Px_DBG
    #[doc(alias = "FlipCrossbarDbgSetting")]
    pub fn set_flip_crossbar_dbg_setting(&mut self, value: bool) {
        let start = 139;
        let end = 139;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 140` - Set the `flip_crossbar_aux_setting` field.
    ///
    /// Flip Px_AUX
    #[doc(alias = "FlipCrossbarAuxSetting")]
    pub fn set_flip_crossbar_aux_setting(&mut self, value: bool) {
        let start = 140;
        let end = 140;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 141` - Set the `flip_crossbar_sbtx_setting` field.
    ///
    /// Flip PX_SBTX/RX
    #[doc(alias = "FlipCrossbarSbtxSetting")]
    pub fn set_flip_crossbar_sbtx_setting(&mut self, value: bool) {
        let start = 141;
        let end = 141;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for PortConfig {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 18]> for PortConfig {
    fn from(bits: [u8; 18]) -> Self {
        Self { bits }
    }
}
impl From<PortConfig> for [u8; 18] {
    fn from(val: PortConfig) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PortConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PortConfig");
        d.field("typec_state_machine", &self.typec_state_machine());
        d.field("crossbar_type", &self.crossbar_type());
        d.field("pp_ext_active_low", &self.pp_ext_active_low());
        d.field("typec_support_options", &self.typec_support_options());
        d.field("disable_pd", &self.disable_pd());
        d.field("usb_communication_capable", &self.usb_communication_capable());
        d.field("debug_accessory_support", &self.debug_accessory_support());
        d.field("usb_3_rate", &self.usb_3_rate());
        d.field(
            "crossbar_i_2_c_controller_enable",
            &self.crossbar_i_2_c_controller_enable(),
        );
        d.field("vbus_ovp_usage", &self.vbus_ovp_usage());
        d.field("ovp_for_pp_5_v", &self.ovp_for_pp_5_v());
        d.field(
            "crossbar_config_type_1_extended",
            &self.crossbar_config_type_1_extended(),
        );
        d.field(
            "remove_safe_state_between_usb_3_to_dp_transition",
            &self.remove_safe_state_between_usb_3_to_dp_transition(),
        );
        d.field("vbus_sink_uvp_trip_hv", &self.vbus_sink_uvp_trip_hv());
        d.field("apdo_vbus_uvp_threshold", &self.apdo_vbus_uvp_threshold());
        d.field("apdo_ilim_over_shoot", &self.apdo_ilim_over_shoot());
        d.field(
            "apdo_vbus_uvp_trip_point_offset",
            &self.apdo_vbus_uvp_trip_point_offset(),
        );
        d.field("vbus_for_valid_pps_status", &self.vbus_for_valid_pps_status());
        d.field("external_dcdc_type", &self.external_dcdc_type());
        d.field("sink_mode_i_2_c_irq_config", &self.sink_mode_i_2_c_irq_config());
        d.field("gt_threshold_voltage", &self.gt_threshold_voltage());
        d.field("enable_internal_auxbiasing", &self.enable_internal_auxbiasing());
        d.field("enable_internal_level_shifter", &self.enable_internal_level_shifter());
        d.field("level_shifter_direction_cfg", &self.level_shifter_direction_cfg());
        d.field("sbu_mux_debug_setting", &self.sbu_mux_debug_setting());
        d.field("sbu_mux_default_setting", &self.sbu_mux_default_setting());
        d.field("sbu_mux_usage", &self.sbu_mux_usage());
        d.field("epr_supported_as_source", &self.epr_supported_as_source());
        d.field("epr_supported_as_sink", &self.epr_supported_as_sink());
        d.field("disable_sourcing_in_dbm", &self.disable_sourcing_in_dbm());
        d.field("flip_crossbar_dbg_setting", &self.flip_crossbar_dbg_setting());
        d.field("flip_crossbar_aux_setting", &self.flip_crossbar_aux_setting());
        d.field("flip_crossbar_sbtx_setting", &self.flip_crossbar_sbtx_setting());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PortConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PortConfig {{ ");
        defmt::write!(f, "typec_state_machine: {}, ", & self.typec_state_machine());
        defmt::write!(f, "crossbar_type: {}, ", & self.crossbar_type());
        defmt::write!(f, "pp_ext_active_low: {=bool}, ", & self.pp_ext_active_low());
        defmt::write!(f, "typec_support_options: {}, ", & self.typec_support_options());
        defmt::write!(f, "disable_pd: {=bool}, ", & self.disable_pd());
        defmt::write!(
            f, "usb_communication_capable: {=bool}, ", & self.usb_communication_capable()
        );
        defmt::write!(
            f, "debug_accessory_support: {=bool}, ", & self.debug_accessory_support()
        );
        defmt::write!(f, "usb_3_rate: {}, ", & self.usb_3_rate());
        defmt::write!(
            f, "crossbar_i_2_c_controller_enable: {=bool}, ", & self
            .crossbar_i_2_c_controller_enable()
        );
        defmt::write!(f, "vbus_ovp_usage: {}, ", & self.vbus_ovp_usage());
        defmt::write!(f, "ovp_for_pp_5_v: {}, ", & self.ovp_for_pp_5_v());
        defmt::write!(
            f, "crossbar_config_type_1_extended: {=bool}, ", & self
            .crossbar_config_type_1_extended()
        );
        defmt::write!(
            f, "remove_safe_state_between_usb_3_to_dp_transition: {=bool}, ", & self
            .remove_safe_state_between_usb_3_to_dp_transition()
        );
        defmt::write!(f, "vbus_sink_uvp_trip_hv: {}, ", & self.vbus_sink_uvp_trip_hv());
        defmt::write!(
            f, "apdo_vbus_uvp_threshold: {=u8}, ", & self.apdo_vbus_uvp_threshold()
        );
        defmt::write!(f, "apdo_ilim_over_shoot: {=u8}, ", & self.apdo_ilim_over_shoot());
        defmt::write!(
            f, "apdo_vbus_uvp_trip_point_offset: {=u16}, ", & self
            .apdo_vbus_uvp_trip_point_offset()
        );
        defmt::write!(
            f, "vbus_for_valid_pps_status: {=u16}, ", & self.vbus_for_valid_pps_status()
        );
        defmt::write!(f, "external_dcdc_type: {}, ", & self.external_dcdc_type());
        defmt::write!(
            f, "sink_mode_i_2_c_irq_config: {=bool}, ", & self
            .sink_mode_i_2_c_irq_config()
        );
        defmt::write!(
            f, "gt_threshold_voltage: {=u16}, ", & self.gt_threshold_voltage()
        );
        defmt::write!(
            f, "enable_internal_auxbiasing: {=bool}, ", & self
            .enable_internal_auxbiasing()
        );
        defmt::write!(
            f, "enable_internal_level_shifter: {=bool}, ", & self
            .enable_internal_level_shifter()
        );
        defmt::write!(
            f, "level_shifter_direction_cfg: {}, ", & self.level_shifter_direction_cfg()
        );
        defmt::write!(f, "sbu_mux_debug_setting: {}, ", & self.sbu_mux_debug_setting());
        defmt::write!(
            f, "sbu_mux_default_setting: {}, ", & self.sbu_mux_default_setting()
        );
        defmt::write!(f, "sbu_mux_usage: {}, ", & self.sbu_mux_usage());
        defmt::write!(
            f, "epr_supported_as_source: {=bool}, ", & self.epr_supported_as_source()
        );
        defmt::write!(
            f, "epr_supported_as_sink: {=bool}, ", & self.epr_supported_as_sink()
        );
        defmt::write!(
            f, "disable_sourcing_in_dbm: {=bool}, ", & self.disable_sourcing_in_dbm()
        );
        defmt::write!(
            f, "flip_crossbar_dbg_setting: {=bool}, ", & self.flip_crossbar_dbg_setting()
        );
        defmt::write!(
            f, "flip_crossbar_aux_setting: {=bool}, ", & self.flip_crossbar_aux_setting()
        );
        defmt::write!(
            f, "flip_crossbar_sbtx_setting: {=bool}, ", & self
            .flip_crossbar_sbtx_setting()
        );
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PortConfig {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PortConfig {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PortConfig {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PortConfig {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PortConfig {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PortConfig {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PortConfig {
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
pub struct DiscoveredSvids {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 33],
}
unsafe impl ::device_driver::Fieldset for DiscoveredSvids {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 33] };
}
impl DiscoveredSvids {
    /// `3:0` - Read the `number_sop_svids` field.
    ///
    /// Number of SVIDs discovered on SOP
    #[doc(alias = "NumberSopSvids")]
    #[must_use]
    pub fn number_sop_svids(&self) -> u8 {
        let start = 0;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `7:4` - Read the `number_sop_prime_svids` field.
    ///
    /// Number of SVIDs discovered on SOP'
    #[doc(alias = "NumberSopPrimeSvids")]
    #[must_use]
    pub fn number_sop_prime_svids(&self) -> u8 {
        let start = 4;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `23:8` - Read the `svid_sop_0` field.
    ///
    /// First SVID supported by SOP port partner
    #[doc(alias = "SvidSop0")]
    #[must_use]
    pub fn svid_sop_0(&self) -> u16 {
        let start = 8;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `39:24` - Read the `svid_sop_1` field.
    ///
    /// Second SVID supported by SOP port partner
    #[doc(alias = "SvidSop1")]
    #[must_use]
    pub fn svid_sop_1(&self) -> u16 {
        let start = 24;
        let end = 39;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `55:40` - Read the `svid_sop_2` field.
    ///
    /// Third SVID supported by SOP port partner
    #[doc(alias = "SvidSop2")]
    #[must_use]
    pub fn svid_sop_2(&self) -> u16 {
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
    /// `71:56` - Read the `svid_sop_3` field.
    ///
    /// Fourth SVID supported by SOP port partner
    #[doc(alias = "SvidSop3")]
    #[must_use]
    pub fn svid_sop_3(&self) -> u16 {
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
    /// `87:72` - Read the `svid_sop_4` field.
    ///
    /// Fifth SVID supported by SOP port partner
    #[doc(alias = "SvidSop4")]
    #[must_use]
    pub fn svid_sop_4(&self) -> u16 {
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
    /// `103:88` - Read the `svid_sop_5` field.
    ///
    /// Sixth SVID supported by SOP port partner
    #[doc(alias = "SvidSop5")]
    #[must_use]
    pub fn svid_sop_5(&self) -> u16 {
        let start = 88;
        let end = 103;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `119:104` - Read the `svid_sop_6` field.
    ///
    /// Seventh SVID supported by SOP port partner
    #[doc(alias = "SvidSop6")]
    #[must_use]
    pub fn svid_sop_6(&self) -> u16 {
        let start = 104;
        let end = 119;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `135:120` - Read the `svid_sop_7` field.
    ///
    /// Eighth SVID supported by SOP port partner
    #[doc(alias = "SvidSop7")]
    #[must_use]
    pub fn svid_sop_7(&self) -> u16 {
        let start = 120;
        let end = 135;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `151:136` - Read the `svid_sop_prime_0` field.
    ///
    /// First SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime0")]
    #[must_use]
    pub fn svid_sop_prime_0(&self) -> u16 {
        let start = 136;
        let end = 151;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `167:152` - Read the `svid_sop_prime_1` field.
    ///
    /// Second SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime1")]
    #[must_use]
    pub fn svid_sop_prime_1(&self) -> u16 {
        let start = 152;
        let end = 167;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `183:168` - Read the `svid_sop_prime_2` field.
    ///
    /// Third SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime2")]
    #[must_use]
    pub fn svid_sop_prime_2(&self) -> u16 {
        let start = 168;
        let end = 183;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `199:184` - Read the `svid_sop_prime_3` field.
    ///
    /// Fourth SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime3")]
    #[must_use]
    pub fn svid_sop_prime_3(&self) -> u16 {
        let start = 184;
        let end = 199;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `215:200` - Read the `svid_sop_prime_4` field.
    ///
    /// Fifth SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime4")]
    #[must_use]
    pub fn svid_sop_prime_4(&self) -> u16 {
        let start = 200;
        let end = 215;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `231:216` - Read the `svid_sop_prime_5` field.
    ///
    /// Sixth SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime5")]
    #[must_use]
    pub fn svid_sop_prime_5(&self) -> u16 {
        let start = 216;
        let end = 231;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `247:232` - Read the `svid_sop_prime_6` field.
    ///
    /// Seventh SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime6")]
    #[must_use]
    pub fn svid_sop_prime_6(&self) -> u16 {
        let start = 232;
        let end = 247;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `263:248` - Read the `svid_sop_prime_7` field.
    ///
    /// Eighth SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime7")]
    #[must_use]
    pub fn svid_sop_prime_7(&self) -> u16 {
        let start = 248;
        let end = 263;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u16,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `3:0` - Set the `number_sop_svids` field.
    ///
    /// Number of SVIDs discovered on SOP
    #[doc(alias = "NumberSopSvids")]
    pub fn set_number_sop_svids(&mut self, value: u8) {
        let start = 0;
        let end = 3;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `7:4` - Set the `number_sop_prime_svids` field.
    ///
    /// Number of SVIDs discovered on SOP'
    #[doc(alias = "NumberSopPrimeSvids")]
    pub fn set_number_sop_prime_svids(&mut self, value: u8) {
        let start = 4;
        let end = 7;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `23:8` - Set the `svid_sop_0` field.
    ///
    /// First SVID supported by SOP port partner
    #[doc(alias = "SvidSop0")]
    pub fn set_svid_sop_0(&mut self, value: u16) {
        let start = 8;
        let end = 23;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `39:24` - Set the `svid_sop_1` field.
    ///
    /// Second SVID supported by SOP port partner
    #[doc(alias = "SvidSop1")]
    pub fn set_svid_sop_1(&mut self, value: u16) {
        let start = 24;
        let end = 39;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `55:40` - Set the `svid_sop_2` field.
    ///
    /// Third SVID supported by SOP port partner
    #[doc(alias = "SvidSop2")]
    pub fn set_svid_sop_2(&mut self, value: u16) {
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
    /// `71:56` - Set the `svid_sop_3` field.
    ///
    /// Fourth SVID supported by SOP port partner
    #[doc(alias = "SvidSop3")]
    pub fn set_svid_sop_3(&mut self, value: u16) {
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
    /// `87:72` - Set the `svid_sop_4` field.
    ///
    /// Fifth SVID supported by SOP port partner
    #[doc(alias = "SvidSop4")]
    pub fn set_svid_sop_4(&mut self, value: u16) {
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
    /// `103:88` - Set the `svid_sop_5` field.
    ///
    /// Sixth SVID supported by SOP port partner
    #[doc(alias = "SvidSop5")]
    pub fn set_svid_sop_5(&mut self, value: u16) {
        let start = 88;
        let end = 103;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `119:104` - Set the `svid_sop_6` field.
    ///
    /// Seventh SVID supported by SOP port partner
    #[doc(alias = "SvidSop6")]
    pub fn set_svid_sop_6(&mut self, value: u16) {
        let start = 104;
        let end = 119;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `135:120` - Set the `svid_sop_7` field.
    ///
    /// Eighth SVID supported by SOP port partner
    #[doc(alias = "SvidSop7")]
    pub fn set_svid_sop_7(&mut self, value: u16) {
        let start = 120;
        let end = 135;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `151:136` - Set the `svid_sop_prime_0` field.
    ///
    /// First SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime0")]
    pub fn set_svid_sop_prime_0(&mut self, value: u16) {
        let start = 136;
        let end = 151;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `167:152` - Set the `svid_sop_prime_1` field.
    ///
    /// Second SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime1")]
    pub fn set_svid_sop_prime_1(&mut self, value: u16) {
        let start = 152;
        let end = 167;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `183:168` - Set the `svid_sop_prime_2` field.
    ///
    /// Third SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime2")]
    pub fn set_svid_sop_prime_2(&mut self, value: u16) {
        let start = 168;
        let end = 183;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `199:184` - Set the `svid_sop_prime_3` field.
    ///
    /// Fourth SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime3")]
    pub fn set_svid_sop_prime_3(&mut self, value: u16) {
        let start = 184;
        let end = 199;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `215:200` - Set the `svid_sop_prime_4` field.
    ///
    /// Fifth SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime4")]
    pub fn set_svid_sop_prime_4(&mut self, value: u16) {
        let start = 200;
        let end = 215;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `231:216` - Set the `svid_sop_prime_5` field.
    ///
    /// Sixth SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime5")]
    pub fn set_svid_sop_prime_5(&mut self, value: u16) {
        let start = 216;
        let end = 231;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `247:232` - Set the `svid_sop_prime_6` field.
    ///
    /// Seventh SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime6")]
    pub fn set_svid_sop_prime_6(&mut self, value: u16) {
        let start = 232;
        let end = 247;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `263:248` - Set the `svid_sop_prime_7` field.
    ///
    /// Eighth SVID supported by SOP' cable plug
    #[doc(alias = "SvidSopPrime7")]
    pub fn set_svid_sop_prime_7(&mut self, value: u16) {
        let start = 248;
        let end = 263;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u16,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for DiscoveredSvids {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 33]> for DiscoveredSvids {
    fn from(bits: [u8; 33]) -> Self {
        Self { bits }
    }
}
impl From<DiscoveredSvids> for [u8; 33] {
    fn from(val: DiscoveredSvids) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for DiscoveredSvids {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("DiscoveredSvids");
        d.field("number_sop_svids", &self.number_sop_svids());
        d.field("number_sop_prime_svids", &self.number_sop_prime_svids());
        d.field("svid_sop_0", &self.svid_sop_0());
        d.field("svid_sop_1", &self.svid_sop_1());
        d.field("svid_sop_2", &self.svid_sop_2());
        d.field("svid_sop_3", &self.svid_sop_3());
        d.field("svid_sop_4", &self.svid_sop_4());
        d.field("svid_sop_5", &self.svid_sop_5());
        d.field("svid_sop_6", &self.svid_sop_6());
        d.field("svid_sop_7", &self.svid_sop_7());
        d.field("svid_sop_prime_0", &self.svid_sop_prime_0());
        d.field("svid_sop_prime_1", &self.svid_sop_prime_1());
        d.field("svid_sop_prime_2", &self.svid_sop_prime_2());
        d.field("svid_sop_prime_3", &self.svid_sop_prime_3());
        d.field("svid_sop_prime_4", &self.svid_sop_prime_4());
        d.field("svid_sop_prime_5", &self.svid_sop_prime_5());
        d.field("svid_sop_prime_6", &self.svid_sop_prime_6());
        d.field("svid_sop_prime_7", &self.svid_sop_prime_7());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DiscoveredSvids {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DiscoveredSvids {{ ");
        defmt::write!(f, "number_sop_svids: {=u8}, ", & self.number_sop_svids());
        defmt::write!(
            f, "number_sop_prime_svids: {=u8}, ", & self.number_sop_prime_svids()
        );
        defmt::write!(f, "svid_sop_0: {=u16}, ", & self.svid_sop_0());
        defmt::write!(f, "svid_sop_1: {=u16}, ", & self.svid_sop_1());
        defmt::write!(f, "svid_sop_2: {=u16}, ", & self.svid_sop_2());
        defmt::write!(f, "svid_sop_3: {=u16}, ", & self.svid_sop_3());
        defmt::write!(f, "svid_sop_4: {=u16}, ", & self.svid_sop_4());
        defmt::write!(f, "svid_sop_5: {=u16}, ", & self.svid_sop_5());
        defmt::write!(f, "svid_sop_6: {=u16}, ", & self.svid_sop_6());
        defmt::write!(f, "svid_sop_7: {=u16}, ", & self.svid_sop_7());
        defmt::write!(f, "svid_sop_prime_0: {=u16}, ", & self.svid_sop_prime_0());
        defmt::write!(f, "svid_sop_prime_1: {=u16}, ", & self.svid_sop_prime_1());
        defmt::write!(f, "svid_sop_prime_2: {=u16}, ", & self.svid_sop_prime_2());
        defmt::write!(f, "svid_sop_prime_3: {=u16}, ", & self.svid_sop_prime_3());
        defmt::write!(f, "svid_sop_prime_4: {=u16}, ", & self.svid_sop_prime_4());
        defmt::write!(f, "svid_sop_prime_5: {=u16}, ", & self.svid_sop_prime_5());
        defmt::write!(f, "svid_sop_prime_6: {=u16}, ", & self.svid_sop_prime_6());
        defmt::write!(f, "svid_sop_prime_7: {=u16}, ", & self.svid_sop_prime_7());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for DiscoveredSvids {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for DiscoveredSvids {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for DiscoveredSvids {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for DiscoveredSvids {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for DiscoveredSvids {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for DiscoveredSvids {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for DiscoveredSvids {
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
pub struct Data1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 64],
}
unsafe impl ::device_driver::Fieldset for Data1 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 64] };
}
impl Data1 {
    /// `7:0` - Read the `bytes` field.
    ///
    /// Command payload byte
    #[doc(alias = "Bytes")]
    #[must_use]
    pub fn bytes(&self, index: usize) -> u8 {
        assert!(index < 64);
        let start = 0 + index * 8;
        let end = start + 8 - 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `7:0` - Set the `bytes` field.
    ///
    /// Command payload byte
    #[doc(alias = "Bytes")]
    pub fn set_bytes(&mut self, index: usize, value: u8) {
        assert!(index < 64);
        let start = 0 + index * 8;
        let end = start + 8 - 1;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for Data1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 64]> for Data1 {
    fn from(bits: [u8; 64]) -> Self {
        Self { bits }
    }
}
impl From<Data1> for [u8; 64] {
    fn from(val: Data1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Data1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Data1");
        d.field("bytes[0]", &self.bytes(0));
        d.field("bytes[1]", &self.bytes(1));
        d.field("bytes[2]", &self.bytes(2));
        d.field("bytes[3]", &self.bytes(3));
        d.field("bytes[4]", &self.bytes(4));
        d.field("bytes[5]", &self.bytes(5));
        d.field("bytes[6]", &self.bytes(6));
        d.field("bytes[7]", &self.bytes(7));
        d.field("bytes[8]", &self.bytes(8));
        d.field("bytes[9]", &self.bytes(9));
        d.field("bytes[10]", &self.bytes(10));
        d.field("bytes[11]", &self.bytes(11));
        d.field("bytes[12]", &self.bytes(12));
        d.field("bytes[13]", &self.bytes(13));
        d.field("bytes[14]", &self.bytes(14));
        d.field("bytes[15]", &self.bytes(15));
        d.field("bytes[16]", &self.bytes(16));
        d.field("bytes[17]", &self.bytes(17));
        d.field("bytes[18]", &self.bytes(18));
        d.field("bytes[19]", &self.bytes(19));
        d.field("bytes[20]", &self.bytes(20));
        d.field("bytes[21]", &self.bytes(21));
        d.field("bytes[22]", &self.bytes(22));
        d.field("bytes[23]", &self.bytes(23));
        d.field("bytes[24]", &self.bytes(24));
        d.field("bytes[25]", &self.bytes(25));
        d.field("bytes[26]", &self.bytes(26));
        d.field("bytes[27]", &self.bytes(27));
        d.field("bytes[28]", &self.bytes(28));
        d.field("bytes[29]", &self.bytes(29));
        d.field("bytes[30]", &self.bytes(30));
        d.field("bytes[31]", &self.bytes(31));
        d.field("bytes[32]", &self.bytes(32));
        d.field("bytes[33]", &self.bytes(33));
        d.field("bytes[34]", &self.bytes(34));
        d.field("bytes[35]", &self.bytes(35));
        d.field("bytes[36]", &self.bytes(36));
        d.field("bytes[37]", &self.bytes(37));
        d.field("bytes[38]", &self.bytes(38));
        d.field("bytes[39]", &self.bytes(39));
        d.field("bytes[40]", &self.bytes(40));
        d.field("bytes[41]", &self.bytes(41));
        d.field("bytes[42]", &self.bytes(42));
        d.field("bytes[43]", &self.bytes(43));
        d.field("bytes[44]", &self.bytes(44));
        d.field("bytes[45]", &self.bytes(45));
        d.field("bytes[46]", &self.bytes(46));
        d.field("bytes[47]", &self.bytes(47));
        d.field("bytes[48]", &self.bytes(48));
        d.field("bytes[49]", &self.bytes(49));
        d.field("bytes[50]", &self.bytes(50));
        d.field("bytes[51]", &self.bytes(51));
        d.field("bytes[52]", &self.bytes(52));
        d.field("bytes[53]", &self.bytes(53));
        d.field("bytes[54]", &self.bytes(54));
        d.field("bytes[55]", &self.bytes(55));
        d.field("bytes[56]", &self.bytes(56));
        d.field("bytes[57]", &self.bytes(57));
        d.field("bytes[58]", &self.bytes(58));
        d.field("bytes[59]", &self.bytes(59));
        d.field("bytes[60]", &self.bytes(60));
        d.field("bytes[61]", &self.bytes(61));
        d.field("bytes[62]", &self.bytes(62));
        d.field("bytes[63]", &self.bytes(63));
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Data1 {{ ");
        defmt::write!(f, "bytes: {=u8}, [0]", & self.bytes(0));
        defmt::write!(f, "bytes: {=u8}, [1]", & self.bytes(1));
        defmt::write!(f, "bytes: {=u8}, [2]", & self.bytes(2));
        defmt::write!(f, "bytes: {=u8}, [3]", & self.bytes(3));
        defmt::write!(f, "bytes: {=u8}, [4]", & self.bytes(4));
        defmt::write!(f, "bytes: {=u8}, [5]", & self.bytes(5));
        defmt::write!(f, "bytes: {=u8}, [6]", & self.bytes(6));
        defmt::write!(f, "bytes: {=u8}, [7]", & self.bytes(7));
        defmt::write!(f, "bytes: {=u8}, [8]", & self.bytes(8));
        defmt::write!(f, "bytes: {=u8}, [9]", & self.bytes(9));
        defmt::write!(f, "bytes: {=u8}, [10]", & self.bytes(10));
        defmt::write!(f, "bytes: {=u8}, [11]", & self.bytes(11));
        defmt::write!(f, "bytes: {=u8}, [12]", & self.bytes(12));
        defmt::write!(f, "bytes: {=u8}, [13]", & self.bytes(13));
        defmt::write!(f, "bytes: {=u8}, [14]", & self.bytes(14));
        defmt::write!(f, "bytes: {=u8}, [15]", & self.bytes(15));
        defmt::write!(f, "bytes: {=u8}, [16]", & self.bytes(16));
        defmt::write!(f, "bytes: {=u8}, [17]", & self.bytes(17));
        defmt::write!(f, "bytes: {=u8}, [18]", & self.bytes(18));
        defmt::write!(f, "bytes: {=u8}, [19]", & self.bytes(19));
        defmt::write!(f, "bytes: {=u8}, [20]", & self.bytes(20));
        defmt::write!(f, "bytes: {=u8}, [21]", & self.bytes(21));
        defmt::write!(f, "bytes: {=u8}, [22]", & self.bytes(22));
        defmt::write!(f, "bytes: {=u8}, [23]", & self.bytes(23));
        defmt::write!(f, "bytes: {=u8}, [24]", & self.bytes(24));
        defmt::write!(f, "bytes: {=u8}, [25]", & self.bytes(25));
        defmt::write!(f, "bytes: {=u8}, [26]", & self.bytes(26));
        defmt::write!(f, "bytes: {=u8}, [27]", & self.bytes(27));
        defmt::write!(f, "bytes: {=u8}, [28]", & self.bytes(28));
        defmt::write!(f, "bytes: {=u8}, [29]", & self.bytes(29));
        defmt::write!(f, "bytes: {=u8}, [30]", & self.bytes(30));
        defmt::write!(f, "bytes: {=u8}, [31]", & self.bytes(31));
        defmt::write!(f, "bytes: {=u8}, [32]", & self.bytes(32));
        defmt::write!(f, "bytes: {=u8}, [33]", & self.bytes(33));
        defmt::write!(f, "bytes: {=u8}, [34]", & self.bytes(34));
        defmt::write!(f, "bytes: {=u8}, [35]", & self.bytes(35));
        defmt::write!(f, "bytes: {=u8}, [36]", & self.bytes(36));
        defmt::write!(f, "bytes: {=u8}, [37]", & self.bytes(37));
        defmt::write!(f, "bytes: {=u8}, [38]", & self.bytes(38));
        defmt::write!(f, "bytes: {=u8}, [39]", & self.bytes(39));
        defmt::write!(f, "bytes: {=u8}, [40]", & self.bytes(40));
        defmt::write!(f, "bytes: {=u8}, [41]", & self.bytes(41));
        defmt::write!(f, "bytes: {=u8}, [42]", & self.bytes(42));
        defmt::write!(f, "bytes: {=u8}, [43]", & self.bytes(43));
        defmt::write!(f, "bytes: {=u8}, [44]", & self.bytes(44));
        defmt::write!(f, "bytes: {=u8}, [45]", & self.bytes(45));
        defmt::write!(f, "bytes: {=u8}, [46]", & self.bytes(46));
        defmt::write!(f, "bytes: {=u8}, [47]", & self.bytes(47));
        defmt::write!(f, "bytes: {=u8}, [48]", & self.bytes(48));
        defmt::write!(f, "bytes: {=u8}, [49]", & self.bytes(49));
        defmt::write!(f, "bytes: {=u8}, [50]", & self.bytes(50));
        defmt::write!(f, "bytes: {=u8}, [51]", & self.bytes(51));
        defmt::write!(f, "bytes: {=u8}, [52]", & self.bytes(52));
        defmt::write!(f, "bytes: {=u8}, [53]", & self.bytes(53));
        defmt::write!(f, "bytes: {=u8}, [54]", & self.bytes(54));
        defmt::write!(f, "bytes: {=u8}, [55]", & self.bytes(55));
        defmt::write!(f, "bytes: {=u8}, [56]", & self.bytes(56));
        defmt::write!(f, "bytes: {=u8}, [57]", & self.bytes(57));
        defmt::write!(f, "bytes: {=u8}, [58]", & self.bytes(58));
        defmt::write!(f, "bytes: {=u8}, [59]", & self.bytes(59));
        defmt::write!(f, "bytes: {=u8}, [60]", & self.bytes(60));
        defmt::write!(f, "bytes: {=u8}, [61]", & self.bytes(61));
        defmt::write!(f, "bytes: {=u8}, [62]", & self.bytes(62));
        defmt::write!(f, "bytes: {=u8}, [63]", & self.bytes(63));
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Data1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Data1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Data1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Data1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Data1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Data1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Data1 {
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
pub enum ProductTypeUfp {
    UndefinedUfp = 0,
    PdUsbHub = 1,
    PdUsbPeripheral = 2,
    Psd = 3,
    Reserved(u8) = 4,
}
impl From<u8> for ProductTypeUfp {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UndefinedUfp,
            1 => Self::PdUsbHub,
            2 => Self::PdUsbPeripheral,
            3 => Self::Psd,
            val => Self::Reserved(val),
        }
    }
}
impl From<ProductTypeUfp> for u8 {
    fn from(val: ProductTypeUfp) -> Self {
        match val {
            ProductTypeUfp::UndefinedUfp => 0,
            ProductTypeUfp::PdUsbHub => 1,
            ProductTypeUfp::PdUsbPeripheral => 2,
            ProductTypeUfp::Psd => 3,
            ProductTypeUfp::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ProductTypeUfp {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProductTypeDfp {
    UndefinedDfp = 0,
    PdUsbHub = 1,
    PdUsbHost = 2,
    PowerBrick = 3,
    Amc = 4,
    Reserved(u8) = 5,
}
impl From<u8> for ProductTypeDfp {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UndefinedDfp,
            1 => Self::PdUsbHub,
            2 => Self::PdUsbHost,
            3 => Self::PowerBrick,
            4 => Self::Amc,
            val => Self::Reserved(val),
        }
    }
}
impl From<ProductTypeDfp> for u8 {
    fn from(val: ProductTypeDfp) -> Self {
        match val {
            ProductTypeDfp::UndefinedDfp => 0,
            ProductTypeDfp::PdUsbHub => 1,
            ProductTypeDfp::PdUsbHost => 2,
            ProductTypeDfp::PowerBrick => 3,
            ProductTypeDfp::Amc => 4,
            ProductTypeDfp::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ProductTypeDfp {
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
pub enum PpsRequestInterval {
    EightSeconds = 0,
    FourSeconds = 1,
    TwoSeconds = 2,
    OneSecond = 3,
}
impl core::convert::TryFrom<u8> for PpsRequestInterval {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightSeconds),
            1 => Ok(Self::FourSeconds),
            2 => Ok(Self::TwoSeconds),
            3 => Ok(Self::OneSecond),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PpsRequestInterval",
                })
            }
        }
    }
}
impl From<PpsRequestInterval> for u8 {
    fn from(val: PpsRequestInterval) -> Self {
        match val {
            PpsRequestInterval::EightSeconds => 0,
            PpsRequestInterval::FourSeconds => 1,
            PpsRequestInterval::TwoSeconds => 2,
            PpsRequestInterval::OneSecond => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PpsRequestInterval {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoComputeSinkMaxVoltage {
    ProvidedByHost = 0,
    ComputedByPdController = 1,
}
impl core::convert::TryFrom<u8> for AutoComputeSinkMaxVoltage {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ProvidedByHost),
            1 => Ok(Self::ComputedByPdController),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "AutoComputeSinkMaxVoltage",
                })
            }
        }
    }
}
impl From<AutoComputeSinkMaxVoltage> for u8 {
    fn from(val: AutoComputeSinkMaxVoltage) -> Self {
        match val {
            AutoComputeSinkMaxVoltage::ProvidedByHost => 0,
            AutoComputeSinkMaxVoltage::ComputedByPdController => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AutoComputeSinkMaxVoltage {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoComputeSinkMinVoltage {
    ProvidedByHost = 0,
    ComputedByPdController = 1,
}
impl core::convert::TryFrom<u8> for AutoComputeSinkMinVoltage {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ProvidedByHost),
            1 => Ok(Self::ComputedByPdController),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "AutoComputeSinkMinVoltage",
                })
            }
        }
    }
}
impl From<AutoComputeSinkMinVoltage> for u8 {
    fn from(val: AutoComputeSinkMinVoltage) -> Self {
        match val {
            AutoComputeSinkMinVoltage::ProvidedByHost => 0,
            AutoComputeSinkMinVoltage::ComputedByPdController => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AutoComputeSinkMinVoltage {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoCapabilityMismatch {
    Enabled = 0,
    Disabled = 1,
}
impl core::convert::TryFrom<u8> for NoCapabilityMismatch {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Enabled),
            1 => Ok(Self::Disabled),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "NoCapabilityMismatch",
                })
            }
        }
    }
}
impl From<NoCapabilityMismatch> for u8 {
    fn from(val: NoCapabilityMismatch) -> Self {
        match val {
            NoCapabilityMismatch::Enabled => 0,
            NoCapabilityMismatch::Disabled => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for NoCapabilityMismatch {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoComputeSinkMinPower {
    ProvidedByHost = 0,
    ComputedByPdController = 1,
}
impl core::convert::TryFrom<u8> for AutoComputeSinkMinPower {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ProvidedByHost),
            1 => Ok(Self::ComputedByPdController),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "AutoComputeSinkMinPower",
                })
            }
        }
    }
}
impl From<AutoComputeSinkMinPower> for u8 {
    fn from(val: AutoComputeSinkMinPower) -> Self {
        match val {
            AutoComputeSinkMinPower::ProvidedByHost => 0,
            AutoComputeSinkMinPower::ComputedByPdController => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AutoComputeSinkMinPower {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoNegRdoPriority {
    HigherVoltage = 0,
    LowerVoltage = 1,
}
impl core::convert::TryFrom<u8> for AutoNegRdoPriority {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::HigherVoltage),
            1 => Ok(Self::LowerVoltage),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "AutoNegRdoPriority",
                })
            }
        }
    }
}
impl From<AutoNegRdoPriority> for u8 {
    fn from(val: AutoNegRdoPriority) -> Self {
        match val {
            AutoNegRdoPriority::HigherVoltage => 0,
            AutoNegRdoPriority::LowerVoltage => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AutoNegRdoPriority {
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
pub enum SbuMuxUsage {
    DisplayPortAux = 0,
    AllAux = 1,
    Reserved(u8) = 2,
}
impl From<u8> for SbuMuxUsage {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::DisplayPortAux,
            1 => Self::AllAux,
            val => Self::Reserved(val),
        }
    }
}
impl From<SbuMuxUsage> for u8 {
    fn from(val: SbuMuxUsage) -> Self {
        match val {
            SbuMuxUsage::DisplayPortAux => 0,
            SbuMuxUsage::AllAux => 1,
            SbuMuxUsage::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for SbuMuxUsage {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SbuMuxDefaultSetting {
    DontChange = 0,
    DisableSbuMux = 1,
    EnableDbg = 2,
    Reserved(u8) = 3,
}
impl From<u8> for SbuMuxDefaultSetting {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::DontChange,
            1 => Self::DisableSbuMux,
            2 => Self::EnableDbg,
            val => Self::Reserved(val),
        }
    }
}
impl From<SbuMuxDefaultSetting> for u8 {
    fn from(val: SbuMuxDefaultSetting) -> Self {
        match val {
            SbuMuxDefaultSetting::DontChange => 0,
            SbuMuxDefaultSetting::DisableSbuMux => 1,
            SbuMuxDefaultSetting::EnableDbg => 2,
            SbuMuxDefaultSetting::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for SbuMuxDefaultSetting {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SbuMuxDebugSetting {
    DebugSnk = 0,
    UnorientedDebugSrc = 1,
    OrientedDebugSrc = 2,
    AnyDebug = 3,
    NeverEnable = 4,
    Reserved(u8) = 5,
}
impl From<u8> for SbuMuxDebugSetting {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::DebugSnk,
            1 => Self::UnorientedDebugSrc,
            2 => Self::OrientedDebugSrc,
            3 => Self::AnyDebug,
            4 => Self::NeverEnable,
            val => Self::Reserved(val),
        }
    }
}
impl From<SbuMuxDebugSetting> for u8 {
    fn from(val: SbuMuxDebugSetting) -> Self {
        match val {
            SbuMuxDebugSetting::DebugSnk => 0,
            SbuMuxDebugSetting::UnorientedDebugSrc => 1,
            SbuMuxDebugSetting::OrientedDebugSrc => 2,
            SbuMuxDebugSetting::AnyDebug => 3,
            SbuMuxDebugSetting::NeverEnable => 4,
            SbuMuxDebugSetting::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for SbuMuxDebugSetting {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LevelShifterDirectionCfg {
    Automatic = 0,
    GpioControlled = 1,
    #[doc(alias = "I2cControlled")]
    I2CControlled = 2,
    Reserved(u8) = 3,
}
impl From<u8> for LevelShifterDirectionCfg {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Automatic,
            1 => Self::GpioControlled,
            2 => Self::I2CControlled,
            val => Self::Reserved(val),
        }
    }
}
impl From<LevelShifterDirectionCfg> for u8 {
    fn from(val: LevelShifterDirectionCfg) -> Self {
        match val {
            LevelShifterDirectionCfg::Automatic => 0,
            LevelShifterDirectionCfg::GpioControlled => 1,
            LevelShifterDirectionCfg::I2CControlled => 2,
            LevelShifterDirectionCfg::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for LevelShifterDirectionCfg {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExternalDcdcType {
    MarinaBay = 0,
    Bq25758 = 1,
    Reserved(u8) = 2,
}
impl From<u8> for ExternalDcdcType {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::MarinaBay,
            1 => Self::Bq25758,
            val => Self::Reserved(val),
        }
    }
}
impl From<ExternalDcdcType> for u8 {
    fn from(val: ExternalDcdcType) -> Self {
        match val {
            ExternalDcdcType::MarinaBay => 0,
            ExternalDcdcType::Bq25758 => 1,
            ExternalDcdcType::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ExternalDcdcType {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusSinkUvpTripHv {
    Pct5 = 0,
    Pct10 = 1,
    Pct15 = 2,
    Pct20 = 3,
    Pct25 = 4,
    Pct30 = 5,
    Pct40 = 6,
    Reserved(u8) = 7,
}
impl From<u8> for VbusSinkUvpTripHv {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Pct5,
            1 => Self::Pct10,
            2 => Self::Pct15,
            3 => Self::Pct20,
            4 => Self::Pct25,
            5 => Self::Pct30,
            6 => Self::Pct40,
            val => Self::Reserved(val),
        }
    }
}
impl From<VbusSinkUvpTripHv> for u8 {
    fn from(val: VbusSinkUvpTripHv) -> Self {
        match val {
            VbusSinkUvpTripHv::Pct5 => 0,
            VbusSinkUvpTripHv::Pct10 => 1,
            VbusSinkUvpTripHv::Pct15 => 2,
            VbusSinkUvpTripHv::Pct20 => 3,
            VbusSinkUvpTripHv::Pct25 => 4,
            VbusSinkUvpTripHv::Pct30 => 5,
            VbusSinkUvpTripHv::Pct40 => 6,
            VbusSinkUvpTripHv::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for VbusSinkUvpTripHv {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "OvpForPp5v")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvpForPp5V {
    #[doc(alias = "V5p25")]
    V5P25 = 0,
    #[doc(alias = "V5p5")]
    V5P5 = 1,
    #[doc(alias = "V5p8")]
    V5P8 = 2,
    #[doc(alias = "V6p1")]
    V6P1 = 3,
}
impl core::convert::TryFrom<u8> for OvpForPp5V {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::V5P25),
            1 => Ok(Self::V5P5),
            2 => Ok(Self::V5P8),
            3 => Ok(Self::V6P1),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "OvpForPp5V",
                })
            }
        }
    }
}
impl From<OvpForPp5V> for u8 {
    fn from(val: OvpForPp5V) -> Self {
        match val {
            OvpForPp5V::V5P25 => 0,
            OvpForPp5V::V5P5 => 1,
            OvpForPp5V::V5P8 => 2,
            OvpForPp5V::V6P1 => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for OvpForPp5V {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusOvpUsage {
    Pct100 = 0,
    Pct105 = 1,
    Pct111 = 2,
    Pct114 = 3,
}
impl core::convert::TryFrom<u8> for VbusOvpUsage {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Pct100),
            1 => Ok(Self::Pct105),
            2 => Ok(Self::Pct111),
            3 => Ok(Self::Pct114),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "VbusOvpUsage",
                })
            }
        }
    }
}
impl From<VbusOvpUsage> for u8 {
    fn from(val: VbusOvpUsage) -> Self {
        match val {
            VbusOvpUsage::Pct100 => 0,
            VbusOvpUsage::Pct105 => 1,
            VbusOvpUsage::Pct111 => 2,
            VbusOvpUsage::Pct114 => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for VbusOvpUsage {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb3Rate {
    NotSupported = 0,
    Gen1 = 1,
    Gen2 = 2,
    Reserved(u8) = 3,
}
impl From<u8> for Usb3Rate {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NotSupported,
            1 => Self::Gen1,
            2 => Self::Gen2,
            val => Self::Reserved(val),
        }
    }
}
impl From<Usb3Rate> for u8 {
    fn from(val: Usb3Rate) -> Self {
        match val {
            Usb3Rate::NotSupported => 0,
            Usb3Rate::Gen1 => 1,
            Usb3Rate::Gen2 => 2,
            Usb3Rate::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Usb3Rate {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TypeCSupportOptions {
    None = 0,
    TrySrcDrp = 1,
    Reserved(u8) = 2,
}
impl From<u8> for TypeCSupportOptions {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::TrySrcDrp,
            val => Self::Reserved(val),
        }
    }
}
impl From<TypeCSupportOptions> for u8 {
    fn from(val: TypeCSupportOptions) -> Self {
        match val {
            TypeCSupportOptions::None => 0,
            TypeCSupportOptions::TrySrcDrp => 1,
            TypeCSupportOptions::Reserved(num) => num,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for TypeCSupportOptions {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrossbarType {
    Type1 = 0,
    Type2 = 1,
}
impl core::convert::TryFrom<u8> for CrossbarType {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Type1),
            1 => Ok(Self::Type2),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "CrossbarType",
                })
            }
        }
    }
}
impl From<CrossbarType> for u8 {
    fn from(val: CrossbarType) -> Self {
        match val {
            CrossbarType::Type1 => 0,
            CrossbarType::Type2 => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for CrossbarType {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TypeCStateMachine {
    Sink = 0,
    Source = 1,
    Drp = 2,
    Disabled = 3,
}
impl core::convert::TryFrom<u8> for TypeCStateMachine {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Sink),
            1 => Ok(Self::Source),
            2 => Ok(Self::Drp),
            3 => Ok(Self::Disabled),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "TypeCStateMachine",
                })
            }
        }
    }
}
impl From<TypeCStateMachine> for u8 {
    fn from(val: TypeCStateMachine) -> Self {
        match val {
            TypeCStateMachine::Sink => 0,
            TypeCStateMachine::Source => 1,
            TypeCStateMachine::Drp => 2,
            TypeCStateMachine::Disabled => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for TypeCStateMachine {
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
