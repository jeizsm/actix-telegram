use super::*;

/// 
/// Maximum file size to download is 20 MB
/// 
/// This object represents a file ready to be downloaded. The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    file_id: String,
    file_size: Option<Integer>,
    file_path: Option<String>,
}