use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultLocation {
    ty: String,
    id: String,
    latitude: Float,
    longitude: Float,
    title: String,
    live_period: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<Integer>,
    thumb_height: Option<Integer>,
}