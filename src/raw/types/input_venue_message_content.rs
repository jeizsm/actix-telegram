use crate::types::*;

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    latitude: Float,
    /// Longitude of the venue in degrees
    longitude: Float,
    /// Name of the venue
    title: String,
    /// Address of the venue
    address: String,
    /// Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
}
