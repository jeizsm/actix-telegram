use crate::types::*;

/// Represents a photo to be sent.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InputMediaPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub(crate) media: InputFileOrString,
    /// Caption of the photo to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<String>,
}
