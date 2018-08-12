use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PngSticker {
    InputFile(InputFile),
    String(String),
}
