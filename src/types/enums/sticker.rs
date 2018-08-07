use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Sticker {
    InputFile(InputFile),
    String(String),
}
