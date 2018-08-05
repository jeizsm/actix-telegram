use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    id: String,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}