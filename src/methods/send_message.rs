use super::super::types::*;

/// Use this method to send text messages. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    chat_id: ChatId,
    text: String,
    parse_mode: Option<String>,
    disable_web_page_preview: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}