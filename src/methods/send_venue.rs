use super::super::types::*;

/// Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendVenue {
    chat_id: ChatId,
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}