use crate::types::Integer;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ChatIdOrUsername {
    Id(Integer),
    Username(String),
}

impl From<i64> for ChatIdOrUsername {
    fn from(number: i64) -> Self {
        ChatIdOrUsername::Id(number)
    }
}

impl From<String> for ChatIdOrUsername {
    fn from(username: String) -> Self {
        ChatIdOrUsername::Username(username)
    }
}
