//! Received SOP Prime Identity Data Object register (`0x49`).
//!
//! This register's size exceeds the maximum supported length by the [`device_driver`] crate.
//!
//! This register contains the response to Discover Identity command sent to the SOP' or SOP'' cable plug.

use bitfield::bitfield;
use embedded_usb_pd::vdm::structured::command::discover_identity::active_cable_vdo::{
    ParseActiveCableVdo1Error, ParseActiveCableVdo2Error,
};
use embedded_usb_pd::vdm::structured::command::discover_identity::passive_cable_vdo::ParsePassiveCableVdoError;
use embedded_usb_pd::vdm::structured::command::discover_identity::sop_prime::{
    id_header_vdo, IdHeaderVdo, ProductTypeVdos,
};
use embedded_usb_pd::vdm::structured::command::discover_identity::vpd_vdo::ParseVpdVdoError;
use embedded_usb_pd::vdm::structured::command::discover_identity::{
    ActiveCableVdo1, CertStatVdo, ProductTypeVdo, ProductVdo,
};
use embedded_usb_pd::vdm::structured::header::CommandType;

use crate::debug;

/// The address of the `Received SOP Prime Identity Data Object` register.
pub const ADDR: u8 = 0x49;

/// The length of the `Received SOP Prime Identity Data Object` register, in bytes.
///
/// This exceeds the maximum supported length by the [`device_driver`] crate.
pub const LEN: usize = 200 / 8;

/// Index of the ID Header VDO in the Received SOP Prime Identity Data Object's VDO list.
///
/// See [`ReceivedSopPrimeIdentityData::id_header`].
const ID_HEADER_VDO_INDEX: usize = 0;

/// Index of the Cert Stat VDO in the Received SOP Prime Identity Data Object's VDO list.
///
/// See [`ReceivedSopPrimeIdentityData::cert_stat`].
const CERT_STAT_VDO_INDEX: usize = 1;

/// Index of the Product VDO in the Received SOP Prime Identity Data Object's VDO list.
///
/// See [`ReceivedSopPrimeIdentityData::product_vdo`].
const PRODUCT_VDO_INDEX: usize = 2;

/// Index of the first Product Type VDO in the Received SOP Prime Identity Data Object's VDO list.
///
/// See [`ReceivedSopPrimeIdentityData::product_type_vdos`].
const PRODUCT_TYPE_VDOS_STARTING_INDEX: usize = 3;

bitfield! {
    /// Received SOP Prime Identity Data Object register
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct Raw([u8]);
    impl Debug;

    /// Number of valid VDOs in this register (max of 6).
    pub u8, number_valid_vdos, set_number_valid_vdos: 2, 0;

    /// Type of response received.
    ///
    /// See [`CommandType`] for more details.
    pub u8, response_type, set_response_type: 7, 6;

    pub u32, vdo1, set_vdo1: 39, 8;
    pub u32, vdo2, set_vdo2: 71, 40;
    pub u32, vdo3, set_vdo3: 103, 72;
    pub u32, vdo4, set_vdo4: 135, 104;
    pub u32, vdo5, set_vdo5: 167, 136;
    pub u32, vdo6, set_vdo6: 199, 168;
}

/// Received SOP Prime Identity Data Object register, containing the identity information returned from `Discover Identity REQ` messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReceivedSopPrimeIdentityData(Raw<[u8; LEN]>);

impl ReceivedSopPrimeIdentityData {
    pub const DEFAULT: Self = Self(Raw([0; LEN]));

    /// Returns the number of valid VDOs in this register (max of 6).
    pub fn number_valid_vdos(&self) -> usize {
        self.0.number_valid_vdos().min(6) as usize
    }

    /// Returns an iterator over the VDOs.
    ///
    /// Each response usually contains an ID Header VDO, a Cert Stat VDO, a Product VDO,
    /// and up to 3 Product Type VDOs whose types are context-specific. Specific
    /// methods are available to parse the first 3 VDOs and to retrieve the
    /// Product Type VDOs.
    ///
    /// - ID Header VDO: [`Self::id_header`]
    /// - Cert Stat VDO: [`Self::cert_stat`]
    /// - Product VDO: [`Self::product_vdo`]
    /// - Product Type VDOs: [`Self::product_type_vdos`]
    pub fn vdos(&self) -> impl ExactSizeIterator<Item = u32> {
        [
            self.0.vdo1(),
            self.0.vdo2(),
            self.0.vdo3(),
            self.0.vdo4(),
            self.0.vdo5(),
            self.0.vdo6(),
        ]
        .into_iter()
        .take(self.number_valid_vdos())
    }

    /// The type of response received for the Discover Identity command sent to
    /// the SOP' or SOP'' cable plug.
    ///
    /// See [`CommandType`] for more details.
    pub fn response_type(&self) -> CommandType {
        self.0.response_type().into()
    }

    /// Contains information corresponding to the Power Delivery Product.
    ///
    /// Returns [`None`] if there isn't enough valid VDOs to contain an ID Header VDO.
    /// If there are, attempts to parse it as an [`IdHeaderVdo`] and returns the result.
    /// If that fails, returns the raw VDO for further analysis.
    pub fn id_header(&self) -> Option<Result<IdHeaderVdo, id_header_vdo::Raw>> {
        let raw = self.vdos().nth(ID_HEADER_VDO_INDEX)?;
        let raw = id_header_vdo::Raw(raw);
        match IdHeaderVdo::try_from(raw) {
            Ok(id_header) => Some(Ok(id_header)),
            Err(e) => {
                debug!("Failed to parse ID Header VDO: {:?}", e);
                Some(Err(raw))
            }
        }
    }

    /// Contains the XID assigned by USB-IF to the product before certification,
    /// in binary format.
    pub fn cert_stat(&self) -> Option<CertStatVdo> {
        self.vdos().nth(CERT_STAT_VDO_INDEX).map(CertStatVdo)
    }

    /// Contains identity information relating to the product.
    pub fn product_vdo(&self) -> Option<ProductVdo> {
        self.vdos().nth(PRODUCT_VDO_INDEX).map(ProductVdo::from)
    }

    /// Return an iterator over the Product Type VDOs, if present.
    ///
    /// The interpretation of these VDOs is context-specific based on the contents
    /// of the [`Self::id_header`]. Some or all may be padding with the value of `0x00000000`.
    pub fn product_type_vdos(&self) -> impl Iterator<Item = ProductTypeVdo> {
        self.vdos().skip(PRODUCT_TYPE_VDOS_STARTING_INDEX).map(ProductTypeVdo)
    }
}

impl Default for ReceivedSopPrimeIdentityData {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<[u8; LEN]> for ReceivedSopPrimeIdentityData {
    fn from(raw: [u8; LEN]) -> Self {
        Self(Raw(raw))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConvertToResponseVdosError {
    MissingIdHeader,
    InvalidIdHeader(id_header_vdo::Raw),
    MissingCertStat {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,
    },
    MissingProductVdo {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,
    },
    MissingProductTypeVdo {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,
    },
    MissingProductTypeActiveCableVdo2 {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,

        /// The first Product Type (Active Cable) VDO, included for context in debugging.
        active_cable_vdo1: ActiveCableVdo1,
    },
    InvalidProductTypePassiveCableVdo {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,

        /// The inner error encountered when parsing the Product Type (Passive Cable) VDO.
        inner: ParsePassiveCableVdoError,
    },
    InvalidProductTypeActiveCableVdo1 {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,

        /// The inner error encountered when parsing the first Product Type (Active Cable) VDO.
        inner: ParseActiveCableVdo1Error,
    },
    InvalidProductTypeActiveCableVdo2 {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,

        /// The first Product Type (Active Cable) VDO, included for context in debugging.
        active_cable_vdo1: ActiveCableVdo1,

        /// The inner error encountered when parsing the second Product Type (Active Cable) VDO.
        inner: ParseActiveCableVdo2Error,
    },
    InvalidProductTypeVpdVdo {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,

        /// The inner error encountered when parsing the Product Type (VPD) VDO.
        inner: ParseVpdVdoError,
    },
}

impl ConvertToResponseVdosError {
    /// Get the ID Header VDO if it was parsed successfully.
    pub const fn id(&self) -> Option<IdHeaderVdo> {
        match self {
            Self::MissingIdHeader | Self::InvalidIdHeader(_) => None,
            Self::MissingCertStat { id }
            | Self::MissingProductVdo { id, .. }
            | Self::MissingProductTypeVdo { id, .. }
            | Self::MissingProductTypeActiveCableVdo2 { id, .. }
            | Self::InvalidProductTypePassiveCableVdo { id, .. }
            | Self::InvalidProductTypeActiveCableVdo1 { id, .. }
            | Self::InvalidProductTypeActiveCableVdo2 { id, .. }
            | Self::InvalidProductTypeVpdVdo { id, .. } => Some(*id),
        }
    }

    /// Get the Cert Stat VDO if it was parsed successfully.
    pub const fn cert_stat(&self) -> Option<CertStatVdo> {
        match self {
            Self::MissingIdHeader | Self::InvalidIdHeader(_) | Self::MissingCertStat { .. } => None,
            Self::MissingProductVdo { cert_stat, .. }
            | Self::MissingProductTypeVdo { cert_stat, .. }
            | Self::MissingProductTypeActiveCableVdo2 { cert_stat, .. }
            | Self::InvalidProductTypePassiveCableVdo { cert_stat, .. }
            | Self::InvalidProductTypeActiveCableVdo1 { cert_stat, .. }
            | Self::InvalidProductTypeActiveCableVdo2 { cert_stat, .. }
            | Self::InvalidProductTypeVpdVdo { cert_stat, .. } => Some(*cert_stat),
        }
    }

    /// Get the Product VDO if it was parsed successfully.
    pub const fn product(&self) -> Option<ProductVdo> {
        match self {
            Self::MissingIdHeader
            | Self::InvalidIdHeader(_)
            | Self::MissingCertStat { .. }
            | Self::MissingProductVdo { .. } => None,
            Self::MissingProductTypeVdo { product, .. }
            | Self::MissingProductTypeActiveCableVdo2 { product, .. }
            | Self::InvalidProductTypePassiveCableVdo { product, .. }
            | Self::InvalidProductTypeActiveCableVdo1 { product, .. }
            | Self::InvalidProductTypeActiveCableVdo2 { product, .. }
            | Self::InvalidProductTypeVpdVdo { product, .. } => Some(*product),
        }
    }

    /// Get the Active Cable VDO1 if it was parsed successfully.
    ///
    /// If the Active Cable VDO2 was parsed successfully, it, and the VDO1, are
    /// available in the [`Ok`] return value of the [`TryFrom`] implementation.
    pub const fn active_cable_vdo1(&self) -> Option<ActiveCableVdo1> {
        match self {
            Self::MissingIdHeader
            | Self::InvalidIdHeader(_)
            | Self::MissingCertStat { .. }
            | Self::MissingProductVdo { .. }
            | Self::MissingProductTypeVdo { .. }
            | Self::InvalidProductTypePassiveCableVdo { .. }
            | Self::InvalidProductTypeActiveCableVdo1 { .. }
            | Self::InvalidProductTypeVpdVdo { .. } => None,
            Self::MissingProductTypeActiveCableVdo2 { active_cable_vdo1, .. }
            | Self::InvalidProductTypeActiveCableVdo2 { active_cable_vdo1, .. } => Some(*active_cable_vdo1),
        }
    }
}

impl TryFrom<ReceivedSopPrimeIdentityData>
    for embedded_usb_pd::vdm::structured::command::discover_identity::sop_prime::ResponseVdos
{
    type Error = ConvertToResponseVdosError;

    fn try_from(value: ReceivedSopPrimeIdentityData) -> Result<Self, Self::Error> {
        let id = value
            .id_header()
            .ok_or(ConvertToResponseVdosError::MissingIdHeader)?
            .map_err(ConvertToResponseVdosError::InvalidIdHeader)?;

        let cert_stat = value
            .cert_stat()
            .ok_or(ConvertToResponseVdosError::MissingCertStat { id })?;

        let product = value
            .product_vdo()
            .ok_or(ConvertToResponseVdosError::MissingProductVdo { id, cert_stat })?;

        let product_type_vdos = match id.product_type {
            id_header_vdo::ProductType::NotACablePlugVpd => ProductTypeVdos::NotACablePlugVpd,
            id_header_vdo::ProductType::PassiveCable => {
                let vdo = value
                    .product_type_vdos()
                    .next()
                    .ok_or(ConvertToResponseVdosError::MissingProductTypeVdo { id, cert_stat, product })?
                    .try_into()
                    .map_err(|inner| ConvertToResponseVdosError::InvalidProductTypePassiveCableVdo {
                        id,
                        cert_stat,
                        product,
                        inner,
                    })?;

                ProductTypeVdos::PassiveCable(vdo)
            }
            id_header_vdo::ProductType::ActiveCable => {
                let vdo1 = value
                    .product_type_vdos()
                    .next()
                    .ok_or(ConvertToResponseVdosError::MissingProductTypeVdo { id, cert_stat, product })?
                    .try_into()
                    .map_err(|inner| ConvertToResponseVdosError::InvalidProductTypeActiveCableVdo1 {
                        id,
                        cert_stat,
                        product,
                        inner,
                    })?;

                let vdo2 = value
                    .product_type_vdos()
                    .nth(1)
                    .ok_or(ConvertToResponseVdosError::MissingProductTypeActiveCableVdo2 {
                        id,
                        cert_stat,
                        product,
                        active_cable_vdo1: vdo1,
                    })?
                    .try_into()
                    .map_err(|inner| ConvertToResponseVdosError::InvalidProductTypeActiveCableVdo2 {
                        id,
                        cert_stat,
                        product,
                        active_cable_vdo1: vdo1,
                        inner,
                    })?;

                ProductTypeVdos::ActiveCable(vdo1, vdo2)
            }
            id_header_vdo::ProductType::Vpd => {
                let vdo = value
                    .product_type_vdos()
                    .next()
                    .ok_or(ConvertToResponseVdosError::MissingProductTypeVdo { id, cert_stat, product })?
                    .try_into()
                    .map_err(|inner| ConvertToResponseVdosError::InvalidProductTypeVpdVdo {
                        id,
                        cert_stat,
                        product,
                        inner,
                    })?;

                ProductTypeVdos::Vpd(vdo)
            }
        };

        Ok(Self {
            id: id.into(),
            cert_stat,
            product,
            product_type_vdos,
        })
    }
}

#[cfg(test)]
mod tests {
    use embedded_usb_pd::vdm::structured::command::discover_identity::sop_prime::{ProductTypeVdos, ResponseVdos};

    use super::*;

    #[test]
    fn default_has_no_vdos() {
        let reg = ReceivedSopPrimeIdentityData::default();
        assert_eq!(reg.number_valid_vdos(), 0);
        assert_eq!(reg.vdos().count(), 0);
        assert_eq!(reg.id_header(), None);
        assert_eq!(reg.cert_stat(), None);
        assert_eq!(reg.product_vdo(), None);
        assert_eq!(reg.product_type_vdos().count(), 0);
    }

    #[test]
    fn number_valid_vdos_is_capped_at_6() {
        let mut reg = ReceivedSopPrimeIdentityData::default();
        reg.0.set_number_valid_vdos(7);
        assert_eq!(reg.number_valid_vdos(), 6);
    }

    /// Build a raw register byte array for testing.
    ///
    /// Byte 0 encodes `num_vdos` in bits 2:0 and `response_type` in bits 7:6.
    /// VDO values are stored little-endian starting at byte 1, 4 bytes each.
    fn make_raw(num_vdos: u8, response_type: u8, vdos: &[u32]) -> [u8; LEN] {
        let mut raw = [0u8; LEN];
        raw[0] = (num_vdos & 0b111) | ((response_type & 0b11) << 6);
        for (i, &vdo) in vdos.iter().enumerate().take(6) {
            let offset = 1 + i * 4;
            raw[offset..offset + 4].copy_from_slice(&vdo.to_le_bytes());
        }
        raw
    }

    #[test]
    fn vdos_returns_correct_count() {
        for n in 0..=6u8 {
            let raw = make_raw(n, 0, &[0; 6]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            assert_eq!(reg.vdos().len(), n as usize);
        }
    }

    #[test]
    fn vdos_returns_correct_values() {
        let expected = [0x11111111, 0x22222222, 0x33333333, 0x44444444, 0x55555555, 0x66666666];
        let raw = make_raw(6, 0, &expected);
        let reg = ReceivedSopPrimeIdentityData::from(raw);
        let mut iter = reg.vdos();
        for &e in &expected {
            assert_eq!(iter.next(), Some(e));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn product_type_vdos_skips_first_three() {
        let raw = make_raw(
            6,
            0,
            &[0x11111111, 0x22222222, 0x33333333, 0xAAAAAAAA, 0xBBBBBBBB, 0xCCCCCCCC],
        );
        let reg = ReceivedSopPrimeIdentityData::from(raw);
        let mut iter = reg.product_type_vdos();
        assert_eq!(iter.next().map(|v| v.0), Some(0xAAAAAAAA));
        assert_eq!(iter.next().map(|v| v.0), Some(0xBBBBBBBB));
        assert_eq!(iter.next().map(|v| v.0), Some(0xCCCCCCCC));
        assert_eq!(iter.next(), None);
    }

    mod try_from {
        use super::*;

        // connector_type=Plug (0b11) at bits 22:21, product_type=NotACablePlugVpd (0b000) at bits 29:27.
        const SIMPLE_PLUG_ID_HEADER: u32 = 0b11 << 21; // 0x00600000

        #[test]
        fn missing_id_header() {
            let reg = ReceivedSopPrimeIdentityData::default();
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingIdHeader)
            );
        }

        #[test]
        fn invalid_id_header() {
            // connector_type bits 22:21 = 0b00 is invalid (valid: 0b10=Receptacle, 0b11=Plug)
            let raw = make_raw(1, 0b01, &[0x00000000]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::InvalidIdHeader(id_header_vdo::Raw(
                    0x00000000
                )))
            );
        }

        #[test]
        fn missing_cert_stat() {
            let raw = make_raw(1, 0b01, &[SIMPLE_PLUG_ID_HEADER]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingCertStat {
                    id: SIMPLE_PLUG_ID_HEADER.try_into().unwrap(),
                })
            );
        }

        #[test]
        fn missing_product_vdo() {
            let raw = make_raw(2, 0b01, &[SIMPLE_PLUG_ID_HEADER, 0]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingProductVdo {
                    id: SIMPLE_PLUG_ID_HEADER.try_into().unwrap(),
                    cert_stat: CertStatVdo(0),
                })
            );
        }

        #[test]
        fn success_not_a_cable_plug_vpd() {
            // NotACablePlugVpd requires no product type VDOs, only the base 3.
            let raw = make_raw(3, 0b01, &[SIMPLE_PLUG_ID_HEADER, 0, 0]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            let vdos = ResponseVdos::try_from(reg).unwrap();
            assert_eq!(vdos.product_type_vdos, ProductTypeVdos::NotACablePlugVpd);
        }

        // connector_type=Plug (0b11), product_type=PassiveCable (0b011) at bits 29:27.
        const PASSIVE_CABLE_ID_HEADER: u32 = (0b11 << 21) | (0b011 << 27); // 0x18600000

        // A valid PassiveCableVdo raw value:
        //   vbus_current_handling_capability=ThreeAmps (0b01) at bits 6:5
        //   cable_latency=LessThan10ns (0b0001) at bits 16:13
        //   usb_type_c_or_captive=UsbTypeC (0b10) at bits 19:18
        //   all other fields at their zero-value (usb_highest_speed=Usb2p0, maximum_vbus_voltage=TwentyVolt, etc.)
        const VALID_PASSIVE_CABLE_VDO: u32 = (0b01 << 5) | (0b0001 << 13) | (0b10 << 18); // 0x82020

        #[test]
        fn missing_product_type_vdo_for_passive_cable() {
            // PassiveCable requires 1 product type VDO, but we only have the base 3.
            let raw = make_raw(3, 0b01, &[PASSIVE_CABLE_ID_HEADER, 0, 0]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingProductTypeVdo {
                    id: PASSIVE_CABLE_ID_HEADER.try_into().unwrap(),
                    cert_stat: CertStatVdo(0),
                    product: 0.into(),
                })
            );
        }

        #[test]
        fn invalid_product_type_passive_cable_vdo() {
            // 0x00000000 has vbus_current_handling_capability=0b00, which is invalid.
            let raw = make_raw(4, 0b01, &[PASSIVE_CABLE_ID_HEADER, 0, 0, 0x00000000]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::InvalidProductTypePassiveCableVdo {
                    id: PASSIVE_CABLE_ID_HEADER.try_into().unwrap(),
                    cert_stat: CertStatVdo(0),
                    product: 0.into(),
                    inner: ParsePassiveCableVdoError::InvalidVbusCurrentHandlingCapability,
                })
            );
        }

        #[test]
        fn success_passive_cable() {
            let raw = make_raw(4, 0b01, &[PASSIVE_CABLE_ID_HEADER, 0, 0, VALID_PASSIVE_CABLE_VDO]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            let vdos = ResponseVdos::try_from(reg).unwrap();
            assert_eq!(
                vdos.product_type_vdos,
                ProductTypeVdos::PassiveCable(VALID_PASSIVE_CABLE_VDO.try_into().unwrap())
            );
        }

        // connector_type=Plug (0b11), product_type=ActiveCable (0b100) at bits 29:27.
        const ACTIVE_CABLE_ID_HEADER: u32 = (0b11 << 21) | (0b100 << 27); // 0x20600000

        // A valid ActiveCableVdo1 raw value:
        //   vbus_current_handling_capability=ThreeAmps (0b01) at bits 6:5
        //   cable_termination_type=OneEndActive (0b10) at bits 12:11
        //     (unlike PassiveCableVdo, active cable's CableTerminationType only accepts 0b10/0b11)
        //   cable_latency=LessThan10ns (0b0001) at bits 16:13
        //   usb_type_c_or_captive=UsbTypeC (0b10) at bits 19:18
        //   all other fields at their zero-value
        const VALID_ACTIVE_CABLE_VDO1: u32 = (0b01 << 5) | (0b10 << 11) | (0b0001 << 13) | (0b10 << 18); // 0x83020

        #[test]
        fn missing_active_cable_vdo2() {
            // ActiveCable needs 2 product type VDOs; we provide a valid VDO1 but no VDO2.
            let raw = make_raw(4, 0b01, &[ACTIVE_CABLE_ID_HEADER, 0, 0, VALID_ACTIVE_CABLE_VDO1]);
            let reg = ReceivedSopPrimeIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingProductTypeActiveCableVdo2 {
                    id: ACTIVE_CABLE_ID_HEADER.try_into().unwrap(),
                    cert_stat: CertStatVdo(0),
                    product: 0.into(),
                    active_cable_vdo1: ActiveCableVdo1::try_from(VALID_ACTIVE_CABLE_VDO1).unwrap(),
                })
            );
        }
    }
}
