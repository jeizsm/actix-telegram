use super::*;

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFile {
    source: String,
    ty: String,
    file_hash: String,
    message: String,
}