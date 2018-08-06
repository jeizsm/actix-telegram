use super::super::types::*;

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserProfilePhotos {
    user_id: Integer,
    offset: Option<Integer>,
    limit: Option<Integer>,
}