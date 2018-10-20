use types::*;

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be unspecified
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub type_: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}
