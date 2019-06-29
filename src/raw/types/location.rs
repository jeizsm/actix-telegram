use crate::types::*;

/// This object represents a point on the map.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Location {
    /// Longitude as defined by sender
    pub(crate) longitude: Float,
    /// Latitude as defined by sender
    pub(crate) latitude: Float,
}