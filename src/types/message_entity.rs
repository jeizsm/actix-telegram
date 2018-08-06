use super::*;

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc. 
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    ty: String,
    offset: Integer,
    length: Integer,
    url: Option<String>,
    user: Option<User>,
}