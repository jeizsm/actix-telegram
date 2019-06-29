use crate::types::*;

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be unspecified
    pub(crate) source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Base64-encoded element hash
    pub(crate) element_hash: String,
    /// Error message
    pub(crate) message: String,
}