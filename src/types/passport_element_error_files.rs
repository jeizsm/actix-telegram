use super::*;

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFiles {
    source: String,
    ty: String,
    file_hashes: Vec<String>,
    message: String,
}