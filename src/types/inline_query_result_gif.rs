use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGif {
    ty: String,
    id: String,
    gif_url: String,
    gif_width: Option<Integer>,
    gif_height: Option<Integer>,
    gif_duration: Option<Integer>,
    thumb_url: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}