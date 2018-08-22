use types::*;

/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: Float,
    /// Longitude of the location in degrees
    pub longitude: Float,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
}
