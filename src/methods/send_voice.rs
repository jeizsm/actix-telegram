use super::super::types::*;

/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .ogg file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendVoice {
    chat_id: ChatId,
    voice: Voice,
    caption: Option<String>,
    parse_mode: Option<String>,
    duration: Option<Integer>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}