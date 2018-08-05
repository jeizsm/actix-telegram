use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedDocument {
    ty: String,
    id: String,
    title: String,
    document_file_id: String,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}