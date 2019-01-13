use crate::types::*;

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub(crate) message_text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<String>,
    /// Disables link previews for links in the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_web_page_preview: Option<bool>,
}
