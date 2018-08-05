use super::super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum FromChatId {
    Integer(Integer),
    String(String),
}