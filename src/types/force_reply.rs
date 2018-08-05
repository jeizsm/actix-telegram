use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForceReply {
    force_reply: True,
    selective: Option<bool>,
}