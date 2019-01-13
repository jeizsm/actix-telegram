use crate::types::*;

/// Represents an audio file to be treated as music to be sent.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InputMediaAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub(crate) media: InputFileOrString,
    /// Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<InputFileOrString>,
    /// Caption of the audio to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<String>,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<Integer>,
    /// Performer of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) performer: Option<String>,
    /// Title of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
}
