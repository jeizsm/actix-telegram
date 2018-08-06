use super::*;

/// Contains information about why a request was unsuccessful.
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<Integer>,
    retry_after: Option<Integer>,
}