use types::*;

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be unspecified
    source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    type_: String,
    /// Base64-encoded element hash
    element_hash: String,
    /// Error message
    message: String,
}
