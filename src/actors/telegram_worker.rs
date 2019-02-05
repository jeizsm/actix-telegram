use super::TelegramApi;
use crate::application::UpdateHandler;
use crate::types::Update;
use actix::{Actor, Addr, Context, Handler, Message};

pub struct TelegramWorker<H>
where
    H: UpdateHandler + 'static,
{
    apps: H,
    telegram_api: Addr<TelegramApi>,
}

impl<H> TelegramWorker<H>
where
    H: UpdateHandler + 'static,
{
    pub(crate) fn new(telegram_api: Addr<TelegramApi>, apps: H) -> Self {
        Self { apps, telegram_api }
    }
}

impl<H> Actor for TelegramWorker<H>
where
    H: UpdateHandler + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is stopped");
    }
}

impl<H> Handler<Update> for TelegramWorker<H>
where
    H: UpdateHandler + 'static,
{
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: Update, _ctx: &mut Context<Self>) -> Self::Result {
        debug!("TelegramWorker.Update received {:?}", msg);
        if self.apps.handle(msg, &self.telegram_api).is_some() {
            debug!("update is not handled");
            Err(())
        } else {
            Ok(())
        }
    }
}

impl Message for Update {
    type Result = Result<(), ()>;
}
