use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookInfo {
    url: String,
    has_custom_certificate: bool,
    pending_update_count: Integer,
    last_error_date: Option<Integer>,
    last_error_message: Option<String>,
    max_connections: Option<Integer>,
    allowed_updates: Option<Vec<String>>,
}