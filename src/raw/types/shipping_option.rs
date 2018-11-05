use types::*;

/// This object represents one shipping option.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct ShippingOption {
    /// Shipping option identifier
    id: String,
    /// Option title
    title: String,
    /// List of price portions
    prices: Vec<LabeledPrice>,
}
