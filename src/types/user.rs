use super::*;

/// This object represents a Telegram user or bot.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: Integer,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User‘s or bot’s first name
    pub first_name: String,
    /// User‘s or bot’s last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// User‘s or bot’s username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
