use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    ty: String,
    id: String,
    mpeg4_file_id: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}