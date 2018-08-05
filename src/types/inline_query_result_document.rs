use super::*;

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