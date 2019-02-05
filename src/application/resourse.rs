use crate::actors::TelegramApi;
use crate::types::{
    update::{Update, UpdateKind},
    Message,
};
use actix::Addr;
use failure::Error;

pub struct Resource<'a, U, S> {
    pub(super) value: U,
    pub(super) state: &'a S,
    pub(super) telegram_api: &'a Addr<TelegramApi>,
}

impl<'a, V, S> Resource<'a, V, S> {
    #[inline]
    pub fn map<M, F>(self, function: F) -> Result<Resource<'a, M, S>, Error>
    where
        F: FnOnce(V) -> Result<M, Error>,
    {
        Ok(Resource {
            value: function(self.value)?,
            state: self.state,
            telegram_api: self.telegram_api,
        })
    }

    #[inline]
    pub fn f<F>(self, function: F) -> Result<(), Error>
    where
        F: FnOnce(V, &'a Addr<TelegramApi>, &'a S) -> Result<(), Error>,
    {
        function(self.value, self.telegram_api, self.state)
    }
}

impl<'a, S> Resource<'a, &'a Message, S> {
    #[inline]
    pub fn command<F>(self, function: F, command: &str) -> Result<(), Error>
    where
        F: FnOnce(&'a Message, &'a Addr<TelegramApi>, &'a S) -> Result<(), Error>,
    {
        let message = self.value;
        ensure!(message.bot_command(command).is_some(), "is not bot command");
        function(message, self.telegram_api, self.state)
    }
}

impl<'a, S> Resource<'a, &'a Update, S> {
    #[inline]
    pub fn message(self) -> Result<Resource<'a, &'a Message, S>, Error> {
        if let UpdateKind::Message(message) = self.value.kind() {
            Ok(Resource {
                value: message,
                state: self.state,
                telegram_api: self.telegram_api,
            })
        } else {
            bail!("is not a message")
        }
    }

    #[inline(always)]
    pub fn command<F>(self, function: F, starts_with: &str) -> Result<(), Error>
    where
        F: FnOnce(&'a Message, &'a Addr<TelegramApi>, &'a S) -> Result<(), Error>,
    {
        self.message()?.command(function, starts_with)
    }
}
