use crate::types::*;

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub(crate) id: String,
    /// Sender
    pub(crate) from: User,
    /// Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) location: Option<Location>,
    /// Text of the query (up to 512 characters)
    pub(crate) query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub(crate) offset: String,
}
