use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Video {
    InputFile(InputFile),
    String(String),
}
