use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum VideoNote {
    InputFile(InputFile),
    String(String),
}
