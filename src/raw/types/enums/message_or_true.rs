use types::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(True),
}
