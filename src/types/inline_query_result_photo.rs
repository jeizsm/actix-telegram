use crate::types::{InlineKeyboardMarkup, InputMessageContent, Integer, ParseMode};

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub(crate) id: String,
    /// A valid URL of the photo. Photo must be in jpeg format. Photo size must not exceed 5MB
    pub(crate) photo_url: String,
    /// URL of the thumbnail for the photo
    pub(crate) thumb_url: String,
    /// Width of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) photo_width: Option<Integer>,
    /// Height of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) photo_height: Option<Integer>,
    /// Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_message_content: Option<InputMessageContent>,
}
