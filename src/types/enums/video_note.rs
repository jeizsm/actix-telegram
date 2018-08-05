use super::super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum VideoNote {
    InputFile(InputFile),
    String(String),
}