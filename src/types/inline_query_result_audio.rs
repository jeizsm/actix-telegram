use types::*;

/// Represents a link to an mp3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    pub type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    /// Caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Audio duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}