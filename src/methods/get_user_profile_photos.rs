use types::*;

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Debug, Serialize, TelegramApi)]
#[return_type = "UserProfilePhotos"]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Integer>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>,
}
