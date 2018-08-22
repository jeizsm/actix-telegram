use types::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatIdOrUsername {
    Id(Integer),
    Username(String),
}
