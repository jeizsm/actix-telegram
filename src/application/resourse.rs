use crate::actors::TelegramApi;
use crate::types::Message;
use actix::Addr;

pub struct Resource<'a, U, S> {
    pub(super) value: U,
    pub(super) state: &'a S,
    pub(super) telegram_api: &'a Addr<TelegramApi>,
}

impl<'a, V, S> Resource<'a, V, S> {
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

    pub fn f<F>(self, function: F) -> bool
    where
        F: FnOnce(V, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        function(self.value, self.telegram_api, self.state)
    }
}

impl<'a, S> Resource<'a, Option<Message>, S> {
    pub fn command<F>(self, starts_with: &str, function: F) -> bool
    where
        F: FnOnce(Message, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        if let Some(message) = self.value {
            if message
                .text()
                .as_ref()
                .map_or(false, |text| text.starts_with(starts_with))
            {
                function(message, self.telegram_api, self.state)
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'a, S> Resource<'a, Option<&'a Message>, S> {
    pub fn command<F>(self, starts_with: &str, function: F) -> bool
    where
        F: FnOnce(&'a Message, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        if let Some(message) = self.value {
            if message
                .text()
                .as_ref()
                .map_or(false, |text| text.starts_with(starts_with))
            {
                function(message, self.telegram_api, self.state)
            } else {
                false
            }
        } else {
            false
        }
    }
}
