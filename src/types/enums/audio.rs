use super::super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Audio {
    InputFile(InputFile),
    String(String),
}