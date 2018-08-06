use super::*;

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportFile {
    file_id: String,
    file_size: Integer,
    file_date: Integer,
}