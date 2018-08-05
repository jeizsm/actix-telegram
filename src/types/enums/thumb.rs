use super::super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Thumb {
    InputFile(InputFile),
    String(String),
}