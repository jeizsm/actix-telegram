use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Photo {
    InputFile(InputFile),
    String(String),
}