use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FromChatId {
    Integer(Integer),
    String(String),
}
