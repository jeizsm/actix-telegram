use types::*;

/// Represents a video to be sent.
#[derive(Debug, Serialize, Getters)]
#[get(vis = "pub")]
pub struct InputMediaVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    type_: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    media: InputFileOrString,
    /// Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<InputFileOrString>,
    /// Caption of the video to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    /// Video duration
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    /// Pass True, if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
}
