use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ChatId {
    Integer(Integer),
    String(String),
}
