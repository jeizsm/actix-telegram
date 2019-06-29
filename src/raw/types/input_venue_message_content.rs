use crate::types::*;

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub(crate) latitude: Float,
    /// Longitude of the venue in degrees
    pub(crate) longitude: Float,
    /// Name of the venue
    pub(crate) title: String,
    /// Address of the venue
    pub(crate) address: String,
    /// Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) foursquare_type: Option<String>,
}