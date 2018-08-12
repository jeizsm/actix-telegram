use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Video {
    InputFile(InputFile),
    String(String),
}
