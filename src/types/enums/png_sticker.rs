use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum PngSticker {
    InputFile(InputFile),
    String(String),
}