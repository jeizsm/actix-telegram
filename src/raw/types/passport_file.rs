use types::*;

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct PassportFile {
    /// Unique identifier for this file
    file_id: String,
    /// File size
    file_size: Integer,
    /// Unix time when the file was uploaded
    file_date: Integer,
}
