use super::super::types::*;

/// Use this method to send .webp stickers. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendSticker {
    chat_id: ChatId,
    sticker: Sticker,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}