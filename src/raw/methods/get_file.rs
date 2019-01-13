use crate::types::*;

/// Use this method to get basic info about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "File"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct GetFile {
    /// File identifier to get info about
    pub(crate) file_id: String,
}
