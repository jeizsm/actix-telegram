use crate::types::*;

/// This object represents a venue.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Venue {
    /// Venue location
    pub(crate) location: Location,
    /// Name of the venue
    pub(crate) title: String,
    /// Address of the venue
    pub(crate) address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) foursquare_id: Option<String>,
    /// Foursquare type of the venue. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) foursquare_type: Option<String>,
}
