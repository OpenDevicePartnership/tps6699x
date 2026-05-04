//! Received SOP Identity Data Object register (`0x48`).
//!
//! This register's size exceeds the maximum supported length by the [`device_driver`] crate.
//!
//! This register contains the response to Discover Identity command sent to the SOP port partner.

use bitfield::bitfield;
use embedded_usb_pd::vdm::structured::command::discover_identity::sop::{
    id_header_vdo, DfpProductTypeVdos, IdHeaderVdo, UfpProductTypeVdos,
};
use embedded_usb_pd::vdm::structured::command::discover_identity::ufp_vdo::ParseUfpVdoError;
use embedded_usb_pd::vdm::structured::command::discover_identity::{CertStatVdo, ProductTypeVdo, ProductVdo};
use embedded_usb_pd::vdm::structured::header::CommandType;

use crate::debug;

/// The address of the `Received SOP Identity Data Object` register.
pub const ADDR: u8 = 0x48;

/// The length of the `Received SOP Identity Data Object` register, in bytes.
///
/// This exceeds the maximum supported length by the [`device_driver`] crate.
pub const LEN: usize = 200 / 8;

/// Index of the DFP VDO in the Received SOP Identity Data Object's Product Type VDOs
/// when the port partner supports dual-role (UFP and DFP) functionality.
///
/// - See [`SINGLE_ROLE_DFP_PRODUCT_TYPE_VDOS_INDEX`].
/// - See PD spec 6.4.4.3.1 Discover Identity.
const DUAL_ROLE_DFP_PRODUCT_TYPE_VDOS_INDEX: usize = 2;

/// Index of the DFP VDO in the Received SOP Identity Data Object's Product Type VDOs
/// when the port partner only supports single-role (DFP only) functionality.
///
/// - See [`DUAL_ROLE_DFP_PRODUCT_TYPE_VDOS_INDEX`].
/// - See PD spec 6.4.4.3.1 Discover Identity.
const SINGLE_ROLE_DFP_PRODUCT_TYPE_VDOS_INDEX: usize = 0;

/// Index of the ID Header VDO in the Received SOP Identity Data Object's VDO list.
///
/// See [`ReceivedSopIdentityData::id_header`].
const ID_HEADER_VDO_INDEX: usize = 0;

/// Index of the Cert Stat VDO in the Received SOP Identity Data Object's VDO list.
///
/// See [`ReceivedSopIdentityData::cert_stat`].
const CERT_STAT_VDO_INDEX: usize = 1;

/// Index of the Product VDO in the Received SOP Identity Data Object's VDO list.
///
/// See [`ReceivedSopIdentityData::product_vdo`].
const PRODUCT_VDO_INDEX: usize = 2;

/// Index of the first Product Type VDO in the Received SOP Identity Data Object's VDO list.
///
/// See [`ReceivedSopIdentityData::product_type_vdos`].
const PRODUCT_TYPE_VDOS_STARTING_INDEX: usize = 3;

bitfield! {
    /// Received SOP Identity Data Object register
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

/// Received SOP Identity Data Object register, containing the identity information returned from `Discover Identity REQ` messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReceivedSopIdentityData(Raw<[u8; LEN]>);

impl ReceivedSopIdentityData {
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
    /// the SOP port partner.
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
    ///
    /// See PD spec 6.4.4.3.1.3 Product VDO, table 6.38 Product VDO.
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

impl Default for ReceivedSopIdentityData {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<[u8; LEN]> for ReceivedSopIdentityData {
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
    MissingProductTypeUfpVdo {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,
    },
    InvalidProductTypeUfpVdo {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,

        /// The inner error encountered when parsing the Product Type (UFP) VDO.
        inner: ParseUfpVdoError,
    },
    MissingProductTypeDfpVdo {
        /// The ID Header VDO, included for context in debugging.
        id: IdHeaderVdo,

        /// The Cert Stat VDO, included for context in debugging.
        cert_stat: CertStatVdo,

        /// The Product VDO, included for context in debugging.
        product: ProductVdo,

        /// The UFP Product Type VDO, included for context in debugging.
        ufp_product_type_vdos: UfpProductTypeVdos,

        /// The number of Product Type VDOs needed based on the ID Header.
        needed: usize,

        /// The number of Product Type VDOs actually available.
        available: usize,
    },
}

impl ConvertToResponseVdosError {
    /// Get the ID Header VDO if it was parsed successfully.
    pub const fn id(&self) -> Option<IdHeaderVdo> {
        match self {
            Self::MissingIdHeader | Self::InvalidIdHeader(_) => None,
            Self::MissingCertStat { id }
            | Self::MissingProductVdo { id, .. }
            | Self::MissingProductTypeUfpVdo { id, .. }
            | Self::InvalidProductTypeUfpVdo { id, .. }
            | Self::MissingProductTypeDfpVdo { id, .. } => Some(*id),
        }
    }

    /// Get the Cert Stat VDO if it was parsed successfully.
    pub const fn cert_stat(&self) -> Option<CertStatVdo> {
        match self {
            Self::MissingIdHeader | Self::InvalidIdHeader(_) | Self::MissingCertStat { .. } => None,
            Self::MissingProductVdo { cert_stat, .. }
            | Self::MissingProductTypeUfpVdo { cert_stat, .. }
            | Self::InvalidProductTypeUfpVdo { cert_stat, .. }
            | Self::MissingProductTypeDfpVdo { cert_stat, .. } => Some(*cert_stat),
        }
    }

    /// Get the Product VDO if it was parsed successfully.
    pub const fn product(&self) -> Option<ProductVdo> {
        match self {
            Self::MissingIdHeader
            | Self::InvalidIdHeader(_)
            | Self::MissingCertStat { .. }
            | Self::MissingProductVdo { .. } => None,
            Self::MissingProductTypeUfpVdo { product, .. }
            | Self::InvalidProductTypeUfpVdo { product, .. }
            | Self::MissingProductTypeDfpVdo { product, .. } => Some(*product),
        }
    }

    /// Get the UFP Product Type VDOs if they were parsed successfully.
    ///
    /// If the DFP Product Type VDO was parsed successfully, it, and the UFP VDO,
    /// are available in the [`Ok`] return value of the [`TryFrom`] implementation.
    pub const fn ufp_product_type_vdos(&self) -> Option<UfpProductTypeVdos> {
        match self {
            Self::MissingIdHeader
            | Self::InvalidIdHeader(_)
            | Self::MissingCertStat { .. }
            | Self::MissingProductVdo { .. }
            | Self::MissingProductTypeUfpVdo { .. }
            | Self::InvalidProductTypeUfpVdo { .. } => None,
            Self::MissingProductTypeDfpVdo {
                ufp_product_type_vdos, ..
            } => Some(*ufp_product_type_vdos),
        }
    }
}

impl TryFrom<ReceivedSopIdentityData>
    for embedded_usb_pd::vdm::structured::command::discover_identity::sop::ResponseVdos
{
    type Error = ConvertToResponseVdosError;

    fn try_from(value: ReceivedSopIdentityData) -> Result<Self, Self::Error> {
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

        // parse UFP first since it always comes first in the VDO list for DRDs (see DFP parsing below)
        // this provides the UFP VDO to callers in the case that DFP parsing fails, whereas parsing DFP first would not
        let ufp_product_type_vdos = match id.product_type_ufp {
            id_header_vdo::ProductTypeUfp::NotAUfp => UfpProductTypeVdos::NotAUfp,
            id_header_vdo::ProductTypeUfp::Psd => UfpProductTypeVdos::Psd,

            // these all parse the same way, so combine to reduce code duplication
            product_type_ufp @ (id_header_vdo::ProductTypeUfp::Hub | id_header_vdo::ProductTypeUfp::Peripheral) => {
                let ufp_vdo = value
                    .product_type_vdos()
                    .next()
                    .ok_or(ConvertToResponseVdosError::MissingProductTypeUfpVdo { id, cert_stat, product })?
                    .try_into()
                    .map_err(|inner| ConvertToResponseVdosError::InvalidProductTypeUfpVdo {
                        id,
                        cert_stat,
                        product,
                        inner,
                    })?;

                match product_type_ufp {
                    id_header_vdo::ProductTypeUfp::Hub => UfpProductTypeVdos::Hub(ufp_vdo),
                    id_header_vdo::ProductTypeUfp::Peripheral => UfpProductTypeVdos::Peripheral(ufp_vdo),

                    // technically unreachable since the case was handled above, but we include it for exhaustiveness
                    id_header_vdo::ProductTypeUfp::NotAUfp => UfpProductTypeVdos::NotAUfp,
                    id_header_vdo::ProductTypeUfp::Psd => UfpProductTypeVdos::Psd,
                }
            }
        };

        let dfp_product_type_vdos = match id.product_type_dfp {
            id_header_vdo::ProductTypeDfp::NotADfp => DfpProductTypeVdos::NotADfp,

            // these all parse the same way, so combine to reduce code duplication
            product_type_dfp @ (id_header_vdo::ProductTypeDfp::Hub
            | id_header_vdo::ProductTypeDfp::Host
            | id_header_vdo::ProductTypeDfp::Charger) => {
                /* PD 6.4.4.3.1 Discover Identity

                If the product is a DRD both a Product Type (UFP) and a Product Type (DFP) are declared in the ID Header. These
                products Shall return Product Type VDOs for both UFP and DFP beginning with the UFP VDO, then by a 32-bit Pad
                Object (defined as all '0's), followed by the DFP VDO as shown in Figure 6.17, "Discover Identity Command response
                for a DRD".
                */

                // we're already a DFP at this scope, so we're DRD if we're also a UFP
                let is_dual_role = !matches!(id.product_type_ufp, id_header_vdo::ProductTypeUfp::NotAUfp);
                let index = if is_dual_role {
                    DUAL_ROLE_DFP_PRODUCT_TYPE_VDOS_INDEX
                } else {
                    SINGLE_ROLE_DFP_PRODUCT_TYPE_VDOS_INDEX
                };
                let dfp_vdo = value
                    .product_type_vdos()
                    .nth(index)
                    .ok_or(ConvertToResponseVdosError::MissingProductTypeDfpVdo {
                        id,
                        cert_stat,
                        product,
                        ufp_product_type_vdos,
                        needed: index + 1,
                        available: value.product_type_vdos().count(),
                    })?
                    .into();

                match product_type_dfp {
                    id_header_vdo::ProductTypeDfp::Hub => DfpProductTypeVdos::Hub(dfp_vdo),
                    id_header_vdo::ProductTypeDfp::Host => DfpProductTypeVdos::Host(dfp_vdo),
                    id_header_vdo::ProductTypeDfp::Charger => DfpProductTypeVdos::Charger(dfp_vdo),

                    // techincally unreachable since the case was handled above, but we include it for exhaustiveness
                    id_header_vdo::ProductTypeDfp::NotADfp => DfpProductTypeVdos::NotADfp,
                }
            }
        };

        Ok(Self {
            id: id.into(),
            cert_stat,
            product,
            dfp_product_type_vdos,
            ufp_product_type_vdos,
        })
    }
}

#[cfg(test)]
mod tests {
    use embedded_usb_pd::vdm::structured::command::discover_identity::sop::{
        DfpProductTypeVdos, ResponseVdos, UfpProductTypeVdos,
    };
    use embedded_usb_pd::vdm::structured::header::CommandType;

    use super::*;

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
    fn number_valid_vdos_is_capped_at_6() {
        let mut reg = ReceivedSopIdentityData::default();
        reg.0.set_number_valid_vdos(7);
        assert_eq!(reg.number_valid_vdos(), 6);
    }

    #[test]
    fn vdos_returns_correct_count() {
        for n in 0..=6u8 {
            let raw = make_raw(n, 0, &[0; 6]);
            let reg = ReceivedSopIdentityData::from(raw);
            assert_eq!(reg.vdos().len(), n as usize, "n={n}");
        }
    }

    #[test]
    fn vdos_returns_correct_values() {
        let expected = [0x11111111, 0x22222222, 0x33333333, 0x44444444, 0x55555555, 0x66666666];
        let raw = make_raw(6, 0, &expected);
        let reg = ReceivedSopIdentityData::from(raw);
        let mut iter = reg.vdos();
        for &e in &expected {
            assert_eq!(iter.next(), Some(e));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn response_type_maps_all_variants() {
        let cases = [
            (0b00u8, CommandType::Request),
            (0b01, CommandType::Ack),
            (0b10, CommandType::Nak),
            (0b11, CommandType::Busy),
        ];
        for (raw_bits, expected) in cases {
            let raw = make_raw(0, raw_bits, &[]);
            let reg = ReceivedSopIdentityData::from(raw);
            assert_eq!(reg.response_type(), expected, "raw_bits={raw_bits:#04b}");
        }
    }

    #[test]
    fn id_header_returns_none_when_no_vdos() {
        let reg = ReceivedSopIdentityData::default();
        assert!(reg.id_header().is_none());
    }

    #[test]
    fn cert_stat_returns_none_when_fewer_than_2_vdos() {
        let raw = make_raw(1, 0, &[0]);
        let reg = ReceivedSopIdentityData::from(raw);
        assert!(reg.cert_stat().is_none());
    }

    #[test]
    fn product_vdo_returns_none_when_fewer_than_3_vdos() {
        let raw = make_raw(2, 0, &[0, 0]);
        let reg = ReceivedSopIdentityData::from(raw);
        assert!(reg.product_vdo().is_none());
    }

    #[test]
    fn product_type_vdos_skips_first_three() {
        let raw = make_raw(
            6,
            0,
            &[0x11111111, 0x22222222, 0x33333333, 0xAAAAAAAA, 0xBBBBBBBB, 0xCCCCCCCC],
        );
        let reg = ReceivedSopIdentityData::from(raw);
        let mut iter = reg.product_type_vdos();
        assert_eq!(iter.next().map(|v| v.0), Some(0xAAAAAAAA));
        assert_eq!(iter.next().map(|v| v.0), Some(0xBBBBBBBB));
        assert_eq!(iter.next().map(|v| v.0), Some(0xCCCCCCCC));
        assert_eq!(iter.next(), None);
    }

    mod try_from {
        use super::*;

        // connector_type=Receptacle (0b10) at bits 22:21, product_type_dfp=NotADfp (0b000) at bits 25:23,
        // product_type_ufp=NotAUfp (0b000) at bits 29:27.
        const SIMPLE_RECEPTACLE_ID_HEADER: u32 = 0b10 << 21; // 0x00400000

        #[test]
        fn missing_id_header() {
            let reg = ReceivedSopIdentityData::default();
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingIdHeader)
            );
        }

        #[test]
        fn invalid_id_header() {
            // connector_type bits 22:21 = 0b00 is invalid (valid: 0b10=Receptacle, 0b11=Plug)
            let raw = make_raw(1, 0b01, &[0x00000000]);
            let reg = ReceivedSopIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::InvalidIdHeader(id_header_vdo::Raw(
                    0x00000000
                )))
            );
        }

        #[test]
        fn missing_cert_stat() {
            let raw = make_raw(1, 0b01, &[SIMPLE_RECEPTACLE_ID_HEADER]);
            let reg = ReceivedSopIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingCertStat {
                    id: SIMPLE_RECEPTACLE_ID_HEADER.try_into().unwrap(),
                })
            );
        }

        #[test]
        fn missing_product_vdo() {
            let raw = make_raw(2, 0b01, &[SIMPLE_RECEPTACLE_ID_HEADER, 0]);
            let reg = ReceivedSopIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingProductVdo {
                    id: SIMPLE_RECEPTACLE_ID_HEADER.try_into().unwrap(),
                    cert_stat: CertStatVdo(0),
                })
            );
        }

        #[test]
        fn success_not_ufp_not_dfp() {
            // A device that is neither UFP nor DFP requires only the base 3 VDOs.
            let raw = make_raw(3, 0b01, &[SIMPLE_RECEPTACLE_ID_HEADER, 0, 0]);
            let reg = ReceivedSopIdentityData::from(raw);
            let vdos = ResponseVdos::try_from(reg).unwrap();
            assert_eq!(vdos.ufp_product_type_vdos, UfpProductTypeVdos::NotAUfp);
            assert_eq!(vdos.dfp_product_type_vdos, DfpProductTypeVdos::NotADfp);
        }

        // connector_type=Receptacle (0b10), product_type_ufp=Hub (0b001) at bits 29:27,
        // product_type_dfp=NotADfp (0b000) at bits 25:23.
        const UFP_HUB_ID_HEADER: u32 = (0b10 << 21) | (0b001 << 27); // 0x08400000

        #[test]
        fn missing_ufp_product_type_vdo() {
            // Hub UFP requires a product type VDO, but we only have the base 3.
            let raw = make_raw(3, 0b01, &[UFP_HUB_ID_HEADER, 0, 0]);
            let reg = ReceivedSopIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingProductTypeUfpVdo {
                    id: UFP_HUB_ID_HEADER.try_into().unwrap(),
                    cert_stat: CertStatVdo(0),
                    product: 0.into(),
                })
            );
        }

        #[test]
        fn success_ufp_hub() {
            // 0x00000000 is a valid UfpVdo (usb_highest_speed=Usb2p0, vconn_power=OneW, etc.)
            let raw = make_raw(4, 0b01, &[UFP_HUB_ID_HEADER, 0, 0, 0x00000000]);
            let reg = ReceivedSopIdentityData::from(raw);
            let vdos = ResponseVdos::try_from(reg).unwrap();
            assert_eq!(
                vdos.ufp_product_type_vdos,
                UfpProductTypeVdos::Hub(0.try_into().unwrap())
            );
            assert_eq!(vdos.dfp_product_type_vdos, DfpProductTypeVdos::NotADfp);
        }

        // connector_type=Receptacle (0b10), product_type_dfp=Host (0b010) at bits 25:23,
        // product_type_ufp=NotAUfp (0b000) at bits 29:27.
        const DFP_HOST_ID_HEADER: u32 = (0b10 << 21) | (0b010 << 23); // 0x01400000

        #[test]
        fn missing_dfp_product_type_vdo() {
            // Host DFP requires a product type VDO, but we only have the base 3.
            let raw = make_raw(3, 0b01, &[DFP_HOST_ID_HEADER, 0, 0]);
            let reg = ReceivedSopIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingProductTypeDfpVdo {
                    id: DFP_HOST_ID_HEADER.try_into().unwrap(),
                    cert_stat: CertStatVdo(0),
                    product: 0.into(),
                    ufp_product_type_vdos: UfpProductTypeVdos::NotAUfp,
                    needed: 1,
                    available: 0,
                })
            );
        }

        #[test]
        fn success_dfp_host() {
            // DfpVdo uses From<u32> (infallible), so 0x00000000 is valid.
            let raw = make_raw(4, 0b01, &[DFP_HOST_ID_HEADER, 0, 0, 0x00000000]);
            let reg = ReceivedSopIdentityData::from(raw);
            let vdos = ResponseVdos::try_from(reg).unwrap();
            assert_eq!(vdos.dfp_product_type_vdos, DfpProductTypeVdos::Host(0.into()));
            assert_eq!(vdos.ufp_product_type_vdos, UfpProductTypeVdos::NotAUfp);
        }

        // DRD: connector_type=Receptacle (0b10), product_type_ufp=Hub (0b001) at bits 29:27,
        // product_type_dfp=Host (0b010) at bits 25:23.
        // PD spec: DRD response has [ufp_vdo, pad(0), dfp_vdo] in product type VDOs.
        const DRD_HUB_HOST_ID_HEADER: u32 = (0b10 << 21) | (0b001 << 27) | (0b010 << 23); // 0x09400000

        #[test]
        fn drd_missing_dfp_product_type_vdo() {
            // DRD DFP VDO is at product_type_vdos index 2, but with only 5 total VDOs
            // product_type_vdos() yields 2 items (indices 0 and 1), so nth(2) returns None.
            let raw = make_raw(5, 0b01, &[DRD_HUB_HOST_ID_HEADER, 0, 0, 0, 0]);
            let reg = ReceivedSopIdentityData::from(raw);
            assert_eq!(
                ResponseVdos::try_from(reg),
                Err(ConvertToResponseVdosError::MissingProductTypeDfpVdo {
                    id: DRD_HUB_HOST_ID_HEADER.try_into().unwrap(),
                    cert_stat: CertStatVdo(0),
                    product: 0.into(),
                    ufp_product_type_vdos: UfpProductTypeVdos::Hub(0.try_into().unwrap()),
                    needed: 3,
                    available: 2,
                })
            );
        }

        #[test]
        fn success_drd() {
            // 6 VDOs: id, cert_stat, product_vdo, ufp_vdo, pad(0), dfp_vdo.
            let raw = make_raw(6, 0b01, &[DRD_HUB_HOST_ID_HEADER, 0, 0, 0, 0, 0]);
            let reg = ReceivedSopIdentityData::from(raw);
            let vdos = ResponseVdos::try_from(reg).unwrap();
            assert_eq!(
                vdos.ufp_product_type_vdos,
                UfpProductTypeVdos::Hub(0.try_into().unwrap())
            );
            assert_eq!(vdos.dfp_product_type_vdos, DfpProductTypeVdos::Host(0.into()));
        }
    }
}
