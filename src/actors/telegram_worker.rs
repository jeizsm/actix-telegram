use super::TelegramApi;
use crate::application::App;
use actix::{Actor, Addr, Context, Handler, Message};
use std::sync::Arc;
use types::Update;

pub struct TelegramWorker {
    apps: Arc<Vec<App>>,
    telegram_api: Addr<TelegramApi>,
}

impl TelegramWorker {
    pub(crate) fn new(telegram_api: Addr<TelegramApi>, apps: Arc<Vec<App>>) -> Self {
        Self { apps, telegram_api }
    }
}

impl Actor for TelegramWorker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is stopped");
    }
}

impl Handler<Update> for TelegramWorker {
    type Result = Result<(), ()>;

    fn handle(&mut self, mut msg: Update, _ctx: &mut Context<Self>) -> Self::Result {
        debug!("TelegramWorker.Update received {:?}", msg);
        for app in self.apps.iter() {
            msg = match (app.0)(msg, &self.telegram_api) {
                Ok(()) => {
                    debug!("ok");
                    return Ok(());
                }
                Err(msg) => {
                    debug!("next");
                    msg
                }
            };
        }
        Ok(())
    }
}

impl Message for Update {
    type Result = Result<(), ()>;
}
