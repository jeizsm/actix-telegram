use crate::actors::TelegramApi;
use crate::types::{Update, UpdateId};
use actix::{Context, Handler, Message};
use futures::Future;
use std::num::{NonZeroU16, NonZeroU8};
use std::time::Duration;
use failure::Error;

/// Use this method to receive incoming updates using long polling (wiki). An Array of Update objects is returned.
#[derive(Serialize, Debug, Setters, New)]
#[set(vis = "pub", optional)]
#[new(vis = "pub")]
pub struct OptimizedGetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<UpdateId>,
    /// Limits the number of updates to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit: Option<Limit>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<Timeout>,
    /// List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_updates: Option<Vec<String>>,
}

impl Message for OptimizedGetUpdates {
    type Result = Result<Vec<Update>, Error>;
}

impl Handler<OptimizedGetUpdates> for TelegramApi {
    type Result = Box<Future<Item = Vec<Update>, Error = Error>>;

    fn handle(&mut self, msg: OptimizedGetUpdates, _ctx: &mut Context<Self>) -> Self::Result {
        let timeout = msg.timeout.map_or(self.timeout, |timeout| {
            Duration::from_secs(u64::from(timeout.get()) + 5)
        });

        Self::send_request(&self.token, "getUpdates", timeout, &msg)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Limit(NonZeroU8);

impl From<u8> for Limit {
    #[inline(always)]
    fn from(from: u8) -> Self {
        unsafe { Limit(NonZeroU8::new_unchecked(from)) }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Timeout(NonZeroU16);

impl From<u16> for Timeout {
    #[inline(always)]
    fn from(from: u16) -> Self {
        unsafe { Timeout(NonZeroU16::new_unchecked(from)) }
    }
}

impl Timeout {
    #[inline(always)]
    pub fn get(self) -> u16 {
        self.0.get()
    }
}
