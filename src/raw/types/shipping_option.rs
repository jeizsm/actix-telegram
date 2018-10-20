use types::*;

/// This object represents one shipping option.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}
