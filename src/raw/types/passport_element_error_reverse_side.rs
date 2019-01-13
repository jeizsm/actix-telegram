use crate::types::*;

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be reverse_side
    pub(crate) source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “driver_license”, “identity_card”
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub(crate) file_hash: String,
    /// Error message
    pub(crate) message: String,
}
