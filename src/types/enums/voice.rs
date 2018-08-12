use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Voice {
    InputFile(InputFile),
    String(String),
}
