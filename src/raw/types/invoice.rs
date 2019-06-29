use crate::types::*;

/// This object contains basic information about an invoice.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Invoice {
    /// Product name
    pub(crate) title: String,
    /// Product description
    pub(crate) description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub(crate) start_parameter: String,
    /// Three-letter ISO 4217 currency code
    pub(crate) currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub(crate) total_amount: Integer,
}