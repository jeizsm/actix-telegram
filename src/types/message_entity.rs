use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    ty: String,
    offset: Integer,
    length: Integer,
    url: Option<String>,
    user: Option<User>,
}