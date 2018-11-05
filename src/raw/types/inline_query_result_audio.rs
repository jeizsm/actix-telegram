use types::*;

/// Represents a link to an mp3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct InlineQueryResultAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    type_: String,
    /// Unique identifier for this result, 1-64 bytes
    id: String,
    /// A valid URL for the audio file
    audio_url: String,
    /// Title
    title: String,
    /// Caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    /// Audio duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}
