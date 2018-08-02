use actix::{Message};
use actix::{Actor, Context, Handler};
use types::Update;

pub struct TelegramWorker {
    apps: Box<Fn(Update)>,
}

impl TelegramWorker {
    pub fn new() -> TelegramWorker {
        let app = |a| {
            debug!("TelegramWorker.App {:?}", a);
        };
        TelegramWorker {
            apps: Box::new(app),
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

    fn handle(&mut self, msg: Update, _ctx: &mut Context<Self>) -> Self::Result {
        debug!("TelegramWorker.Update received {:?}", msg);
        (self.apps)(msg);
        Ok(())
    }
}

impl Message for Update {
    type Result = Result<(), ()>;
}
