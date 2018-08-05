use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultArticle {
    ty: String,
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    reply_markup: Option<InlineKeyboardMarkup>,
    url: Option<String>,
    hide_url: Option<bool>,
    description: Option<String>,
    thumb_url: Option<String>,
    thumb_width: Option<Integer>,
    thumb_height: Option<Integer>,
}