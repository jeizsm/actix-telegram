use types::*;

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportFile {
    /// Unique identifier for this file
    pub file_id: String,
    /// File size
    pub file_size: Integer,
    /// Unix time when the file was uploaded
    pub file_date: Integer,
}
