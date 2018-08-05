use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultMpeg4Gif {
    ty: String,
    id: String,
    mpeg4_url: String,
    mpeg4_width: Option<Integer>,
    mpeg4_height: Option<Integer>,
    mpeg4_duration: Option<Integer>,
    thumb_url: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}