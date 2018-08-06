use super::*;

/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFrontSide {
    source: String,
    ty: String,
    file_hash: String,
    message: String,
}