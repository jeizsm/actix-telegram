use crate::types::*;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum InputFileOrString {
    InputFile(InputFile),
    String(String),
}
