use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendVideo {
    chat_id: ChatId,
    video: Video,
    duration: Option<Integer>,
    width: Option<Integer>,
    height: Option<Integer>,
    thumb: Option<Thumb>,
    caption: Option<String>,
    parse_mode: Option<String>,
    supports_streaming: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}