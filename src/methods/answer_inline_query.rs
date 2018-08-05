use super::super::types::*;

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