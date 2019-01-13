use crate::types::*;

/// This object represents a shipping address.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    pub(crate) country_code: String,
    /// State, if applicable
    pub(crate) state: String,
    /// City
    pub(crate) city: String,
    /// First line for the address
    pub(crate) street_line1: String,
    /// Second line for the address
    pub(crate) street_line2: String,
    /// Address post code
    pub(crate) post_code: String,
}
