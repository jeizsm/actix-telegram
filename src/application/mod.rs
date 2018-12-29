use super::TelegramApi;
use crate::types::Update;
use actix::Addr;
use std::sync::Arc;

#[derive(Clone)]
pub struct App<S> {
    state: S,
    inner: Arc<Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + Send + Sync + 'static>,
}

impl<S> App<S> {
    pub fn new<F>(f: F, state: S) -> Self
    where
        F: Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + Send + Sync + 'static,
    {
        Self {
            state,
            inner: Arc::new(f),
        }
    }
}

pub trait UpdateHandler {
    fn handle(&self, Update, &Addr<TelegramApi>) -> Result<(), Update>;
}

impl<S> UpdateHandler for App<S> {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
        (self.inner)(update, telegram_api)
    }
}

impl<S> UpdateHandler for Vec<App<S>> {
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
