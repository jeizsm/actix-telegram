use crate::types::*;

/// This object represent a user's profile pictures.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    total_count: Integer,
    /// Requested profile pictures (in up to 4 sizes each)
    photos: Vec<Vec<PhotoSize>>,
}
