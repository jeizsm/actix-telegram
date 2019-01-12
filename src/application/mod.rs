mod resourse;

use super::TelegramApi;
use crate::types::Update;
use actix::Addr;
pub use self::resourse::Resource;

pub struct TelegramApplication<S> {
    state: S,
    inner: Box<dyn Fn(Resource<&Update, S>) -> bool + 'static>,
}

pub struct App<S> {
    state: S,
    inner: Box<dyn Fn(Resource<&Update, S>) -> bool + 'static>,
}

impl<S> App<S> {
    pub fn new<F>(f: F, state: S) -> Self
    where
        F: Fn(Resource<&Update, S>) -> bool + 'static,
    {
        App { inner: Box::new(f), state }
    }
}

pub trait UpdateHandler {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update>;
}

impl<S> UpdateHandler for TelegramApplication<S> {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
        let res = {
            let resource = Resource {
                value: &update,
                state: &self.state,
                telegram_api: telegram_api,
            };
            (self.inner)(resource)
        };
        if res {
            Ok(())
        } else {
            Err(update)
        }
    }
}

impl<H: UpdateHandler> UpdateHandler for Vec<H> {
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

pub trait IntoUpdateHandler {
    type Handler: UpdateHandler;

    fn into_handler(self) -> Self::Handler;
}

impl<S> IntoUpdateHandler for App<S> {
    type Handler = TelegramApplication<S>;

    fn into_handler(self) -> TelegramApplication<S> {
        TelegramApplication { inner: self.inner, state: self.state }
    }
}

impl<T: IntoUpdateHandler> IntoUpdateHandler for Vec<T> {
    type Handler = Vec<T::Handler>;

    fn into_handler(self) -> Self::Handler {
        self.into_iter().map(|item| item.into_handler()).collect()
    }
}
