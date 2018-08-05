use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMediaGroup {
    chat_id: ChatId,
    media: Media,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
}