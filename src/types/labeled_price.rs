use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct LabeledPrice {
    label: String,
    amount: Integer,
}