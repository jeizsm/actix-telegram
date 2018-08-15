use super::*;

/// Use this method to send photos. On success, the sent Message is returned.
#[derive(Serialize, Debug, TelegramApi)]
#[return_type = "Message"]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. More info on Sending Files »
    pub photo: Photo,
    /// Photo caption (may also be used when resending photos by file_id), 0-200 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}
