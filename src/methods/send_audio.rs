use super::super::types::*;

/// For sending voice messages, use the sendVoice method instead.
/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .mp3 format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
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