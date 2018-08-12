use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Sticker {
    InputFile(InputFile),
    String(String),
}
