use super::super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Voice {
    InputFile(InputFile),
    String(String),
}