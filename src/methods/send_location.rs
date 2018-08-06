use super::super::types::*;

/// Use this method to send point on the map. On success, the sent Message is returned.
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