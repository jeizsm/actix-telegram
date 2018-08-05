use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EditMessageText {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    text: String,
    parse_mode: Option<String>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}