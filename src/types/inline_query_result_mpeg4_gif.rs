use crate::types::*;

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub(crate) id: String,
    /// A valid URL for the MP4 file. File size must not exceed 1MB
    pub(crate) mpeg4_url: String,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mpeg4_width: Option<Integer>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mpeg4_height: Option<Integer>,
    /// Video duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mpeg4_duration: Option<Integer>,
    /// URL of the static thumbnail (jpeg or gif) for the result
    pub(crate) thumb_url: String,
    /// Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_message_content: Option<InputMessageContent>,
}
