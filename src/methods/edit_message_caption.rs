use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EditMessageCaption {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}