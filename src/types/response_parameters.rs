use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<Integer>,
    retry_after: Option<Integer>,
}