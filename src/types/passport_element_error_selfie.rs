use super::*;

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorSelfie {
    source: String,
    ty: String,
    file_hash: String,
    message: String,
}