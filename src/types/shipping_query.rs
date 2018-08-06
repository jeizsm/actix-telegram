use super::*;

/// This object contains information about an incoming shipping query.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}