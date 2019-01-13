use crate::types::*;

/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultCachedDocument {
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub(crate) id: String,
    /// Title for the result
    pub(crate) title: String,
    /// A valid file identifier for the file
    pub(crate) document_file_id: String,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    /// Caption of the document to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_message_content: Option<InputMessageContent>,
}
