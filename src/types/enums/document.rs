use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Document {
    InputFile(InputFile),
    String(String),
}
