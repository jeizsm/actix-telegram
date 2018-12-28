use crate::types::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChatIdOrUsername {
    Id(Integer),
    Username(String),
}
