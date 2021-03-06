use crate::types::*;

/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the venue.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be venue
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub(crate) id: String,
    /// Latitude of the venue location in degrees
    pub(crate) latitude: Float,
    /// Longitude of the venue location in degrees
    pub(crate) longitude: Float,
    /// Title of the venue
    pub(crate) title: String,
    /// Address of the venue
    pub(crate) address: String,
    /// Foursquare identifier of the venue if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) foursquare_type: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_height: Option<Integer>,
}