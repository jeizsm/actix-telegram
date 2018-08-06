use super::*;

/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultContact {
    ty: String,
    id: String,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<Integer>,
    thumb_height: Option<Integer>,
}