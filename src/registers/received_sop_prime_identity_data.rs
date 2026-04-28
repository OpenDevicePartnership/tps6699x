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

bitfield! {
    /// Received source/sink capabilities register
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
        let raw = self.vdos().next()?;
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
        self.vdos().nth(1).map(CertStatVdo)
    }

    /// Contains identity information relating to the product.
    pub fn product_vdo(&self) -> Option<ProductVdo> {
        self.vdos().nth(2).map(ProductVdo::from)
    }

    /// Return an iterator over the Product Type VDOs, if present.
    ///
    /// The interpretation of these VDOs is context-specific based on the contents
    /// of the [`Self::id_header`]. Some or all may be padding with the value of `0x00000000`.
    pub fn product_type_vdos(&self) -> impl Iterator<Item = ProductTypeVdo> {
        self.vdos().skip(3).map(ProductTypeVdo)
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
    use super::*;

    #[test]
    fn number_valid_vdos_is_capped_at_6() {
        let mut reg = ReceivedSopPrimeIdentityData::default();
        reg.0.set_number_valid_vdos(7);
        assert_eq!(reg.number_valid_vdos(), 6);
    }
}
