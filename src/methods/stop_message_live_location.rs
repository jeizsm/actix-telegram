use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StopMessageLiveLocation {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}