use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}