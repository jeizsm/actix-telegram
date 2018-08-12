use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Photo {
    InputFile(InputFile),
    String(String),
}
