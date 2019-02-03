use crate::actors::TelegramApi;
use crate::types::{
    update::{Update, UpdateKind},
    Message,
};
use actix::Addr;

pub struct Resource<'a, U, S> {
    pub(super) value: U,
    pub(super) state: &'a S,
    pub(super) telegram_api: &'a Addr<TelegramApi>,
}

impl<'a, V, S> Resource<'a, V, S> {
    #[inline]
    pub fn map<M, F>(self, function: F) -> Resource<'a, M, S>
    where
        F: FnOnce(V) -> M,
    {
        Resource {
            value: function(self.value),
            state: self.state,
            telegram_api: self.telegram_api,
        }
    }

    #[inline]
    pub fn f<F>(self, function: F) -> bool
    where
        F: FnOnce(V, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        function(self.value, self.telegram_api, self.state)
    }
}

impl<'a, S> Resource<'a, Option<&'a Message>, S> {
    #[inline]
    pub fn command<F>(self, function: F, command: &str) -> bool
    where
        F: FnOnce(&'a Message, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        if let Some(message) = self.value {
            if message.bot_command(command).is_some() {
                return function(message, self.telegram_api, self.state);
            }
        }
        false
    }
}

impl<'a, S> Resource<'a, &'a Update, S> {
    #[inline]
    pub fn message(self) -> Resource<'a, Option<&'a Message>, S> {
        if let UpdateKind::Message(message) = self.value.kind() {
            Resource {
                value: Some(message),
                state: self.state,
                telegram_api: self.telegram_api,
            }
        } else {
            Resource {
                value: None,
                state: self.state,
                telegram_api: self.telegram_api,
            }
        }
    }

    #[inline(always)]
    pub fn command<F>(self, function: F, starts_with: &str) -> bool
    where
        F: FnOnce(&'a Message, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        self.message().command(function, starts_with)
    }
}
