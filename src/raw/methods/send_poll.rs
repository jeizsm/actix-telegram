use crate::types::*;

/// Use this method to send a native poll. A native poll can't be sent to a private chat. On success, the sent Message is returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Message"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct SendPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername). A native poll can't be sent to a private chat.
    pub(crate) chat_id: ChatIdOrUsername,
    /// Poll question, 1-255 characters
    pub(crate) question: String,
    /// List of answer options, 2-10 strings 1-100 characters each
    pub(crate) options: Vec<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<ReplyMarkup>,
}