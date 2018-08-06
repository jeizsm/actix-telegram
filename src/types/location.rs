use super::*;

/// This object represents a point on the map.
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    longitude: Float,
    latitude: Float,
}