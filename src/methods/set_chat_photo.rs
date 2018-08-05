use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetChatPhoto {
    chat_id: ChatId,
    photo: InputFile,
}