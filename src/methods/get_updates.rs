use actix::Message;
use types::TelegramResponse;
use std::num::{NonZeroU8, NonZeroU32, NonZeroU16};

#[derive(Debug, Serialize)]
struct UpdateId(NonZeroU32);

#[derive(Serialize, Debug)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<UpdateId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<NonZeroU8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<NonZeroU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<Vec<String>>,
}

impl GetUpdates {
    pub fn new(timeout: u16, offset: Option<i32>) -> Self {
        let timeout = unsafe { Some(NonZeroU16::new_unchecked(timeout)) } ;
        let offset = unsafe { offset.map(|offset| UpdateId(NonZeroU32::new_unchecked(offset as u32))) };
        let allowed_updates = None;
        let limit = None;
        GetUpdates {
            offset,
            timeout,
            allowed_updates,
            limit,
        }
    }
}

impl Message for GetUpdates {
    type Result = Result<TelegramResponse, ()>;
}
