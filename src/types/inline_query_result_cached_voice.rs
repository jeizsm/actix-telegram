use crate::types::*;

/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the voice message.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultCachedVoice {
    /// Type of the result, must be voice
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub(crate) id: String,
    /// A valid file identifier for the voice message
    pub(crate) voice_file_id: String,
    /// Voice message title
    pub(crate) title: String,
    /// Caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_message_content: Option<InputMessageContent>,
}
