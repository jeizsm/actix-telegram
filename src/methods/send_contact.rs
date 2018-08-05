use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendContact {
    chat_id: ChatId,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<ReplyMarkup>,
}