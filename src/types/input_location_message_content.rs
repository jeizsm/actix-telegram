use super::*;

/// Represents the content of a location message to be sent as the result of an inline query. 
#[derive(Serialize, Deserialize, Debug)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: Float,
    /// Longitude of the location in degrees
    pub longitude: Float,
    /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<Integer>,
}