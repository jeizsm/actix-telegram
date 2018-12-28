use crate::types::*;

/// This object represents a shipping address.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    country_code: String,
    /// State, if applicable
    state: String,
    /// City
    city: String,
    /// First line for the address
    street_line1: String,
    /// Second line for the address
    street_line2: String,
    /// Address post code
    post_code: String,
}
