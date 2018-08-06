use super::super::types::*;

/// Use this method to send a group of photos or videos as an album. On success, an array of the sent Messages is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    /// A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    pub media: Option<Media>,
    /// Sends the messages silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the messages are a reply, ID of the original message
    pub reply_to_message_id: Option<Integer>,
}