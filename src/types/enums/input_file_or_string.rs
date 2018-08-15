use types::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InputFileOrString {
    InputFile(InputFile),
    String(String),
}
