use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    position: Integer,
    user: User,
    score: Integer,
}