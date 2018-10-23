use types::*;

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
///
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// Mime type of the content of video url, “text/html” or “video/mp4”
    pub mime_type: String,
    /// URL of the thumbnail (jpeg only) for the video
    pub thumb_url: String,
    /// Title for the result
    pub title: String,
    /// Caption of the video to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<Integer>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<Integer>,
    /// Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<Integer>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video. This field is required if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
