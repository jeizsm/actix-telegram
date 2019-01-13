use crate::types::*;

/// This object represents information about an order.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct OrderInfo {
    /// User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    /// User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) phone_number: Option<String>,
    /// User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) email: Option<String>,
    /// User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) shipping_address: Option<ShippingAddress>,
}
