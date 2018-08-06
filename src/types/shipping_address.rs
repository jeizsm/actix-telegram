use super::*;

/// This object represents a shipping address.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}