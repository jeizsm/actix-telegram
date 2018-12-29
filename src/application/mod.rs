use super::TelegramApi;
use crate::types::Update;
use actix::Addr;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct App {
    inner: Arc<Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + Send + Sync + 'static>,
}

impl App {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + Send + Sync + 'static,
    {
        App {
            inner: Arc::new(f),
        }
    }
}

pub trait UpdateHandler {
    fn handle(&self, Update, &Addr<TelegramApi>) -> Result<(), Update>;
}

impl UpdateHandler for App {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
        (self.inner)(update, telegram_api)
    }
}

impl UpdateHandler for Vec<App> {
    fn handle(&self, mut update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
        for app in self {
            update = match app.handle(update, telegram_api) {
                Ok(()) => {
                    debug!("ok");
                    return Ok(());
                }
                Err(update) => {
                    debug!("next");
                    update
                }
            };
        }
        Err(update)
    }
}
