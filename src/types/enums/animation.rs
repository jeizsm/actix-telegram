use super::super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Animation {
    InputFile(InputFile),
    String(String),
}