use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedGif {
    ty: String,
    id: String,
    gif_file_id: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}