use crate::types::*;

/// This object contains information about an incoming pre-checkout query.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub(crate) id: String,
    /// User who sent the query
    pub(crate) from: User,
    /// Three-letter ISO 4217 currency code
    pub(crate) currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub(crate) total_amount: Integer,
    /// Bot specified invoice payload
    pub(crate) invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) shipping_option_id: Option<String>,
    /// Order info provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) order_info: Option<OrderInfo>,
}