use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Animation {
    InputFile(InputFile),
    String(String),
}
