use crate::types::*;

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub(crate) result_id: String,
    /// The user that chose the result
    pub(crate) from: User,
    /// Sender location, only for bots that require user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) location: Option<Location>,
    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message. Will be also received in callback queries and can be used to edit the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub(crate) query: String,
}
