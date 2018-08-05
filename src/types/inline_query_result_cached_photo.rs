use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedPhoto {
    ty: String,
    id: String,
    photo_file_id: String,
    title: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}