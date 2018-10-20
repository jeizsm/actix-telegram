use types::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(True),
}
