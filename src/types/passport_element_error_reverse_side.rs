use super::*;

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorReverseSide {
    source: String,
    ty: String,
    file_hash: String,
    message: String,
}