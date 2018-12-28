use crate::types::*;

/// This object contains information about an incoming pre-checkout query.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    id: String,
    /// User who sent the query
    from: User,
    /// Three-letter ISO 4217 currency code
    currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    total_amount: Integer,
    /// Bot specified invoice payload
    invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_option_id: Option<String>,
    /// Order info provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    order_info: Option<OrderInfo>,
}
