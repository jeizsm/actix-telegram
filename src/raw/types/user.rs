use types::*;

/// This object represents a Telegram user or bot.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct User {
    /// Unique identifier for this user or bot
    id: Integer,
    /// True, if this user is a bot
    is_bot: bool,
    /// User‘s or bot’s first name
    first_name: String,
    /// User‘s or bot’s last name
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    /// User‘s or bot’s username
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    /// IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}
