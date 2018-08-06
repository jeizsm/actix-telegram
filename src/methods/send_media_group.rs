use super::super::types::*;

/// Use this method to send a group of photos or videos as an album. On success, an array of the sent Messages is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendMediaGroup {
    chat_id: ChatId,
    media: Media,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
}