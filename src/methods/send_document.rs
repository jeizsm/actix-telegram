use super::super::types::*;

/// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendDocument {
    chat_id: ChatId,
    document: Document,
    thumb: Option<Thumb>,
    caption: Option<String>,
    parse_mode: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}