use crate::types::*;

/// This object represents a venue.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Venue {
    /// Venue location
    location: Location,
    /// Name of the venue
    title: String,
    /// Address of the venue
    address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    /// Foursquare type of the venue. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
}
