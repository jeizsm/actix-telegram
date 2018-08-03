use actix::{Actor, Context, Handler, Message, Addr};
use types::Update;
use super::telegram_api::TelegramApi;

pub struct TelegramWorker {
    apps: Vec<Box<Fn(Update) -> Result<(), Update>>>,
    telegram_api: Addr<TelegramApi>,
}

impl TelegramWorker {
    pub fn new(telegram_api: Addr<TelegramApi>) -> TelegramWorker {
        let app = |a| {
            debug!("TelegramWorker.App {:?}", a);
            Ok(())
        };
        let second_app = |a| {
            debug!("TelegramWorker.App {:?}", a);
            Err(a)
        };
        TelegramWorker {
            apps: vec![Box::new(second_app), Box::new(app)],
            telegram_api: telegram_api,
        }
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
        for app in &self.apps {
            msg = match (app)(msg) {
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
