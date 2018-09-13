use super::TelegramApi;
use actix::{Actor, Addr, Context, Handler, Message};
use std::sync::Arc;
use types::Update;

type UpdateFunction = Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + Sync + Send + 'static;

pub struct App(pub(crate) Box<UpdateFunction>);

impl App {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + Sync + Send + 'static,
    {
        App(Box::new(f))
    }
}

pub struct TelegramWorker {
    apps: Arc<Vec<App>>,
    telegram_api: Addr<TelegramApi>,
}

impl TelegramWorker {
    pub(crate) fn new(telegram_api: Addr<TelegramApi>, apps: Arc<Vec<App>>) -> TelegramWorker {
        TelegramWorker { apps, telegram_api }
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
