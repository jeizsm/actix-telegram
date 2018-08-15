use super::*;

/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .ogg file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Debug, TelegramApi)]
#[return_type = "Message"]
pub struct SendVoice {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub voice: Voice,
    /// Voice message caption, 0-200 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Duration of the voice message in seconds
    pub duration: Option<Integer>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}
