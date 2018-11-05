use types::*;

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be reverse_side
    source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “driver_license”, “identity_card”
    #[serde(rename = "type")]
    type_: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    file_hash: String,
    /// Error message
    message: String,
}
