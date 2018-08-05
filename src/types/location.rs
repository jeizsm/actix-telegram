use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    longitude: Float,
    latitude: Float,
}