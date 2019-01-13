use crate::types::*;

/// Represents a link to a voice recording in an .ogg container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultVoice {
    /// Type of the result, must be voice
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub(crate) id: String,
    /// A valid URL for the voice recording
    pub(crate) voice_url: String,
    /// Recording title
    pub(crate) title: String,
    /// Caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<String>,
    /// Recording duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) voice_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice recording
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_message_content: Option<InputMessageContent>,
}
