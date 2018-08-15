use types::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ChatIdOrUsername {
    Id(Integer),
    Username(String),
}
