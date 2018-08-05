use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputVenueMessageContent {
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
}