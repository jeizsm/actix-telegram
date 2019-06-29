use crate::types::*;

/// This object contains information about an incoming shipping query.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct ShippingQuery {
    /// Unique query identifier
    pub(crate) id: String,
    /// User who sent the query
    pub(crate) from: User,
    /// Bot specified invoice payload
    pub(crate) invoice_payload: String,
    /// User specified shipping address
    pub(crate) shipping_address: ShippingAddress,
}