use crate::types::*;

/// Use this method to send a group of photos or videos as an album. On success, an array of the sent Messages is returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Vec<Message>"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
    /// A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    pub(crate) media: Vec<InputMediaPhotoOrInputMediaVideo>,
    /// Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_notification: Option<bool>,
    /// If the messages are a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_to_message_id: Option<Integer>,
}