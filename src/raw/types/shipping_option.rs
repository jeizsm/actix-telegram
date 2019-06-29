use crate::types::*;

/// This object represents one shipping option.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct ShippingOption {
    /// Shipping option identifier
    pub(crate) id: String,
    /// Option title
    pub(crate) title: String,
    /// List of price portions
    pub(crate) prices: Vec<LabeledPrice>,
}