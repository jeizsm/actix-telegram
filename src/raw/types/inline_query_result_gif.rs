use crate::types::*;

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultGif {
    /// Type of the result, must be gif
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub(crate) id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub(crate) gif_url: String,
    /// Width of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) gif_width: Option<Integer>,
    /// Height of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) gif_height: Option<Integer>,
    /// Duration of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) gif_duration: Option<Integer>,
    /// URL of the static thumbnail for the result (jpeg or gif)
    pub(crate) thumb_url: String,
    /// Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    /// Caption of the GIF file to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_message_content: Option<InputMessageContent>,
}
