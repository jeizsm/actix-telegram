use super::super::types::*;

/// Use this method to receive incoming updates using long polling (wiki). An Array of Update objects is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUpdates {
    offset: Option<Integer>,
    limit: Option<Integer>,
    timeout: Option<Integer>,
    allowed_updates: Option<Vec<String>>,
}