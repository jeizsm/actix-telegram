use crate::types::*;

/// This object represent a user's profile pictures.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub(crate) total_count: Integer,
    /// Requested profile pictures (in up to 4 sizes each)
    pub(crate) photos: Vec<Vec<PhotoSize>>,
}
