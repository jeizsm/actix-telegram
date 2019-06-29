use crate::types::*;

/// Represents a link to an article or web page.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultArticle {
    /// Type of the result, must be article
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub(crate) id: String,
    /// Title of the result
    pub(crate) title: String,
    /// Content of the message to be sent
    pub(crate) input_message_content: InputMessageContent,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,
    /// Pass True, if you don't want the URL to be shown in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hide_url: Option<bool>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_height: Option<Integer>,
}