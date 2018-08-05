use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: Integer,
}