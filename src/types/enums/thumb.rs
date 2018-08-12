use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Thumb {
    InputFile(InputFile),
    String(String),
}
