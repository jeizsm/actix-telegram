use super::*;

/// This object contains information about an incoming pre-checkout query.
#[derive(Serialize, Deserialize, Debug)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: Integer,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}