use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Document {
    InputFile(InputFile),
    String(String),
}
