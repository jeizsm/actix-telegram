use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EditMessageMedia {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}