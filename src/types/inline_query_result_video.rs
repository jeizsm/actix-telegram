use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVideo {
    ty: String,
    id: String,
    video_url: String,
    mime_type: String,
    thumb_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    video_width: Option<Integer>,
    video_height: Option<Integer>,
    video_duration: Option<Integer>,
    description: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}