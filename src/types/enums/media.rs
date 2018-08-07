use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Media {
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}