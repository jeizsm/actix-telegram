use crate::types::*;

/// This object represents a file ready to be downloaded. The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
///
/// Maximum file size to download is 20 MB
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct File {
    /// Unique identifier for this file
    file_id: String,
    /// File size, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
    /// File path. Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_path: Option<String>,
}
