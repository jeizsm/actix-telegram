use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatId {
    Integer(Integer),
    String(String),
}