use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendVideoNote {
    chat_id: ChatId,
    video_note: VideoNote,
    duration: Option<Integer>,
    length: Option<Integer>,
    thumb: Option<Thumb>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}