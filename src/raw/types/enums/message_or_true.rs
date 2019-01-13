use crate::types::*;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(True),
}
