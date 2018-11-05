use types::*;

/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct InlineQueryResultContact {
    /// Type of the result, must be contact
    #[serde(rename = "type")]
    type_: String,
    /// Unique identifier for this result, 1-64 Bytes
    id: String,
    /// Contact's phone number
    phone_number: String,
    /// Contact's first name
    first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_height: Option<Integer>,
}
