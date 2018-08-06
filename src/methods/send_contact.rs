use super::super::types::*;

/// Use this method to send phone contacts. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendContact {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    /// Contact's phone number
    pub phone_number: Option<String>,
    /// Contact's first name
    pub first_name: Option<String>,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}