use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfilePhotos {
    total_count: Integer,
    photos: Vec<Vec<PhotoSize>>,
}