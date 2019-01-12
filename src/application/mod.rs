use super::TelegramApi;
use crate::types::Update;
use actix::Addr;

pub struct TelegramApplication {
    inner: Box<dyn Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + 'static>,
}

pub struct App {
    inner: Box<dyn Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + 'static>,
}

impl App {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Update, &Addr<TelegramApi>) -> Result<(), Update> + 'static
    {
        App { inner: Box::new(f) }
    }
}

pub trait UpdateHandler {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update>;
}

impl UpdateHandler for TelegramApplication {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
        (self.inner)(update, telegram_api)
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

impl IntoUpdateHandler for App {
    type Handler = TelegramApplication;

    fn into_handler(self) -> TelegramApplication {
        TelegramApplication {
            inner: self.inner,
        }
    }
}

impl<T: IntoUpdateHandler> IntoUpdateHandler for Vec<T> {
    type Handler = Vec<T::Handler>;

    fn into_handler(self) -> Self::Handler {
        self.into_iter().map(|item| item.into_handler()).collect()
    }
}
