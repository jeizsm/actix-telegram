use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerPreCheckoutQuery {
    pre_checkout_query_id: String,
    ok: bool,
    error_message: Option<String>,
}