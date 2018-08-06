use super::*;

/// This object represents one shipping option.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}