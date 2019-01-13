use crate::types::*;

/// Use this method to send phone contacts. On success, the sent Message is returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Message"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct SendContact {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
    /// Contact's phone number
    pub(crate) phone_number: String,
    /// Contact's first name
    pub(crate) first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<ReplyMarkup>,
}
