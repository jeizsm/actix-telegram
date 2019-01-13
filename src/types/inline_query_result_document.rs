use crate::types::*;

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultDocument {
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub(crate) id: String,
    /// Title for the result
    pub(crate) title: String,
    /// Caption of the document to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
    /// A valid URL for the file
    pub(crate) document_url: String,
    /// Mime type of the content of the file, either “application/pdf” or “application/zip”
    pub(crate) mime_type: String,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail (jpeg only) for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_height: Option<Integer>,
}
