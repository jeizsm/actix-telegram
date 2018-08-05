use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerCallbackQuery {
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<String>,
    cache_time: Option<Integer>,
}