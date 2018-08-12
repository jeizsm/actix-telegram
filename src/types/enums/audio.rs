use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Audio {
    InputFile(InputFile),
    String(String),
}
