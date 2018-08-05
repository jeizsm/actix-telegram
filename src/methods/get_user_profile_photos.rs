use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserProfilePhotos {
    user_id: Integer,
    offset: Option<Integer>,
    limit: Option<Integer>,
}