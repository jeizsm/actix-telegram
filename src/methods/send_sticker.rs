use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendSticker {
    chat_id: ChatId,
    sticker: Sticker,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}