use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendLocation {
    chat_id: ChatId,
    latitude: Float,
    longitude: Float,
    live_period: Option<Integer>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}