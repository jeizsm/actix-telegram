use crate::types::*;

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct PassportFile {
    /// Unique identifier for this file
    pub(crate) file_id: String,
    /// File size
    pub(crate) file_size: Integer,
    /// Unix time when the file was uploaded
    pub(crate) file_date: Integer,
}