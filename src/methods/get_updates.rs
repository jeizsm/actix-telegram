use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUpdates {
    offset: Option<Integer>,
    limit: Option<Integer>,
    timeout: Option<Integer>,
    allowed_updates: Option<Vec<String>>,
}