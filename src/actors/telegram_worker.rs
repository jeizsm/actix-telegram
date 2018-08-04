use super::TelegramApi;
use actix::{Actor, Addr, Context, Handler, Message};
use types::Update;

struct App(Box<Fn(Update) -> Result<(), Update>>);

impl App {
    fn new<F: 'static>(f: F) -> Self
    where
        F: Fn(Update) -> Result<(), Update>,
    {
        App(Box::new(f))
    }
}

pub(crate) struct TelegramWorker {
    apps: Vec<App>,
    telegram_api: Addr<TelegramApi>,
}

impl TelegramWorker {
    pub(crate) fn new(telegram_api: Addr<TelegramApi>) -> TelegramWorker {
        let app = App::new(|a| {
            debug!("TelegramWorker.App {:?}", a);
            Ok(())
        });
        let second_app = App::new(|a| {
            debug!("TelegramWorker.App {:?}", a);
            Err(a)
        });
        TelegramWorker {
            apps: vec![second_app, app],
            telegram_api,
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
            msg = match (app.0)(msg) {
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
