use crate::types::*;

/// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub(crate) shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)
    pub(crate) ok: bool,
    /// Required if ok is True. A JSON-serialized array of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) shipping_options: Option<Vec<ShippingOption>>,
    /// Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error_message: Option<String>,
}
