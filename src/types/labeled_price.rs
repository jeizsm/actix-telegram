use super::*;

/// This object represents a portion of the price for goods or services.
#[derive(Serialize, Deserialize, Debug)]
pub struct LabeledPrice {
    label: String,
    amount: Integer,
}