use super::*;

/// Represents the content of a venue message to be sent as the result of an inline query. 
#[derive(Serialize, Deserialize, Debug)]
pub struct InputVenueMessageContent {
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
}