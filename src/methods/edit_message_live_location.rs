use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EditMessageLiveLocation {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    latitude: Float,
    longitude: Float,
    reply_markup: Option<InlineKeyboardMarkup>,
}