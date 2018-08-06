use super::*;

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultDocument {
    ty: String,
    id: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    document_url: String,
    mime_type: String,
    description: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<Integer>,
    thumb_height: Option<Integer>,
}