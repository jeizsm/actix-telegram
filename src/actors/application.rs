use types::Update;
use actix::Addr;
use super::TelegramApi;

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
