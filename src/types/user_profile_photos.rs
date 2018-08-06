use super::*;

/// This object represent a user's profile pictures.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfilePhotos {
    total_count: Integer,
    photos: Vec<Vec<PhotoSize>>,
}