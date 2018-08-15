use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Box<Message>),
    True(True),
}
