use super::*;

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    id: String,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}