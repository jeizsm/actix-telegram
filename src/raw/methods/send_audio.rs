use crate::types::*;

/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .mp3 format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
/// For sending voice messages, use the sendVoice method instead.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Message"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct SendAudio {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub(crate) audio: InputFileOrString,
    /// Audio caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<String>,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<Integer>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    /// Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<InputFileOrString>,
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
