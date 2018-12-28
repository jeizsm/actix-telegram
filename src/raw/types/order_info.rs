use crate::types::*;

/// This object represents information about an order.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct OrderInfo {
    /// User name
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,
    /// User email
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address: Option<ShippingAddress>,
}
