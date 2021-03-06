use crate::types::*;

/// This object represents a Telegram user or bot.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct User {
    /// Unique identifier for this user or bot
    pub(crate) id: Integer,
    /// True, if this user is a bot
    pub(crate) is_bot: bool,
    /// User‘s or bot’s first name
    pub(crate) first_name: String,
    /// User‘s or bot’s last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_name: Option<String>,
    /// User‘s or bot’s username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) username: Option<String>,
    /// IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) language_code: Option<String>,
}