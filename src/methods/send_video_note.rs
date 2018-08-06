use super::super::types::*;

/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
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