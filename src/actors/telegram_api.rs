use actix::{Actor, Context};
use std::time::Duration;

pub struct TelegramApi {
    pub(crate) token: String,
    pub(crate) timeout: Duration,
}

impl TelegramApi {
    pub fn new(token: String, timeout: Duration) -> TelegramApi {
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
