use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportFile {
    file_id: String,
    file_size: Integer,
    file_date: Integer,
}