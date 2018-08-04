use actix::{Actor, Context};
use std::time::Duration;

pub struct TelegramApi {
    pub(crate) token: String,
    pub(crate) timeout: Duration,
}

impl TelegramApi {
    pub fn new(token: String, timeout: u16) -> TelegramApi {
        let timeout = Duration::from_secs(timeout as u64);
        TelegramApi { token, timeout }
    }
}

impl Actor for TelegramApi {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramApi is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramApi is stopped");
    }
}
