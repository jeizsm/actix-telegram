use actix::{Actor, Context};

pub struct TelegramApi {
    token: String,
}

impl TelegramApi {
    fn new(token: String) -> TelegramApi {
        TelegramApi { token }
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
