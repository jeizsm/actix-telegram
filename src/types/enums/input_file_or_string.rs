use types::*;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InputFileOrString {
    InputFile(InputFile),
    String(String),
}
