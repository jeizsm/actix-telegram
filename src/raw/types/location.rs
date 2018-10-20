use types::*;

/// This object represents a point on the map.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: Float,
    /// Latitude as defined by sender
    pub latitude: Float,
}
