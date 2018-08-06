use super::super::types::*;

/// Use this method to send answers to an inline query. On success, True is returned.No more than 50 results per query are allowed.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    cache_time: Option<Integer>,
    is_personal: Option<bool>,
    next_offset: Option<String>,
    switch_pm_text: Option<String>,
    switch_pm_parameter: Option<String>,
}