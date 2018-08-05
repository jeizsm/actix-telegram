use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetWebhook {
    url: String,
    certificate: Option<InputFile>,
    max_connections: Option<Integer>,
    allowed_updates: Option<Vec<String>>,
}