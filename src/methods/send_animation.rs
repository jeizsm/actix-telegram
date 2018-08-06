use super::super::types::*;

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendAnimation {
    chat_id: ChatId,
    animation: Animation,
    duration: Option<Integer>,
    width: Option<Integer>,
    height: Option<Integer>,
    thumb: Option<Thumb>,
    caption: Option<String>,
    parse_mode: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}