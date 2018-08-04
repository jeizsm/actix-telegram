use super::send_request;
use actix::{Context, Handler, Message};
use actors::TelegramApi;
use futures::Future;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU8};
use std::time::Duration;
use types::TelegramResponse;

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
        let timeout = unsafe { Some(NonZeroU16::new_unchecked(timeout)) };
        let offset =
            unsafe { offset.map(|offset| UpdateId(NonZeroU32::new_unchecked(offset as u32))) };
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

impl Handler<GetUpdates> for TelegramApi {
    type Result = Box<Future<Item = TelegramResponse, Error = ()>>;

    fn handle(&mut self, msg: GetUpdates, _ctx: &mut Context<Self>) -> Self::Result {
        let timeout = msg.timeout.map_or(self.timeout, |timeout| {
            Duration::from_secs(u64::from(timeout.get()) + 5)
        });

        send_request(&self.token, "getUpdates", timeout, &msg)
    }
}
