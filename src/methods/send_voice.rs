use super::super::types::*;

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