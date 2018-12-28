use crate::types::*;

/// This object contains information about an incoming shipping query.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct ShippingQuery {
    /// Unique query identifier
    id: String,
    /// User who sent the query
    from: User,
    /// Bot specified invoice payload
    invoice_payload: String,
    /// User specified shipping address
    shipping_address: ShippingAddress,
}
