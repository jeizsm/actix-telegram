use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChat {
    chat_id: ChatId,
}