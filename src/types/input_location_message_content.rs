use super::*;

/// Represents the content of a location message to be sent as the result of an inline query. 
#[derive(Serialize, Deserialize, Debug)]
pub struct InputLocationMessageContent {
    latitude: Float,
    longitude: Float,
    live_period: Option<Integer>,
}