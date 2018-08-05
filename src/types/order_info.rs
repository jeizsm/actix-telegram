use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}