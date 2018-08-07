use super::*;

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    /// Type of the entity. Can be mention (@username), hashtag, cashtag, bot_command, url, email, phone_number, bold (bold text), italic (italic text), code (monowidth string), pre (monowidth block), text_link (for clickable text URLs), text_mention (for users without usernames)
    #[serde(rename = "type")]
    pub type_: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units
    pub length: Integer,
    /// For “text_link” only, url that will be opened after user taps on the text
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user
    pub user: Option<User>,
}
