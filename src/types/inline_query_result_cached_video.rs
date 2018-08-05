use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedVideo {
    ty: String,
    id: String,
    video_file_id: String,
    title: String,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}