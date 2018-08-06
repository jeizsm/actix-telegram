use super::super::types::*;

/// Use this method to send video files, Telegram clients support mp4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
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