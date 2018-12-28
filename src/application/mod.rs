use super::TelegramApi;
use crate::types::Update;
use actix::Addr;

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

pub trait UpdateHandler {
    fn handle(&self, Update, &Addr<TelegramApi>) -> Result<(), Update>;
}

impl UpdateHandler for App {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
        (self.0)(update, telegram_api)
    }
}
