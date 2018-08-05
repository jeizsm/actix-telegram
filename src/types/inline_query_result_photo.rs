use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultPhoto {
    ty: String,
    id: String,
    photo_url: String,
    thumb_url: String,
    photo_width: Option<Integer>,
    photo_height: Option<Integer>,
    title: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}