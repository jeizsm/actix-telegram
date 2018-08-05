use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendAudio {
    chat_id: ChatId,
    audio: Audio,
    caption: Option<String>,
    parse_mode: Option<String>,
    duration: Option<Integer>,
    performer: Option<String>,
    title: Option<String>,
    thumb: Option<Thumb>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}