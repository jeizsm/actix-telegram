use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputLocationMessageContent {
    latitude: Float,
    longitude: Float,
    live_period: Option<Integer>,
}