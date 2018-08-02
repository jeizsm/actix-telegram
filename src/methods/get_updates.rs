use actix::Message;
use serde::{Serialize, Serializer};
use std::time::Duration;
use types::{Integer, TelegramResponse};

#[derive(Debug, Serialize)]
pub struct UpdateId(i32);

#[derive(Serialize, Debug)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<UpdateId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_duration")]
    pub timeout: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl GetUpdates {
    pub fn new(timeout: Duration, offset: Option<i32>) -> Self {
        let offset = offset.map(UpdateId);
        GetUpdates {
            offset,
            timeout: Some(timeout),
            allowed_updates: None,
            limit: None,
        }
    }
}

impl Message for GetUpdates {
    type Result = Result<TelegramResponse, ()>;
}

fn serialize_duration<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    duration
        .as_ref()
        .map(|duration| duration.as_secs())
        .serialize(serializer)
}
