use crate::types::*;

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct PassportElementErrorFiles {
    /// Error source, must be files
    pub(crate) source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// List of base64-encoded file hashes
    pub(crate) file_hashes: Vec<String>,
    /// Error message
    pub(crate) message: String,
}
