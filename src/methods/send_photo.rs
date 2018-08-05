use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendPhoto {
    chat_id: ChatId,
    photo: Photo,
    caption: Option<String>,
    parse_mode: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}