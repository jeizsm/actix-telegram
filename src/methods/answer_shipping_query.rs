use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerShippingQuery {
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<ShippingOption>>,
    error_message: Option<String>,
}