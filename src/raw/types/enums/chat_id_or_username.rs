use crate::types::*;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ChatIdOrUsername {
    Id(Integer),
    Username(String),
}