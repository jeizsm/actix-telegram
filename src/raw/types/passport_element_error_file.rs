use types::*;

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct PassportElementErrorFile {
    /// Error source, must be file
    source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    type_: String,
    /// Base64-encoded file hash
    file_hash: String,
    /// Error message
    message: String,
}
