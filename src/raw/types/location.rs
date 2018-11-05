use types::*;

/// This object represents a point on the map.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Location {
    /// Longitude as defined by sender
    longitude: Float,
    /// Latitude as defined by sender
    latitude: Float,
}
