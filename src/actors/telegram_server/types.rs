use actix::{Addr, Message};
use actors::{App, TelegramApi};
use std::num::NonZeroU8;
use std::sync::Arc;
use types::AllowedUpdate;
use types::True;

pub(super) struct ReqState {
    pub(super) telegram_api: Addr<TelegramApi>,
    pub(super) apps: Arc<Vec<App>>,
}

#[derive(Default)]
pub struct ServerSetWebhook {
    pub(super) max_connections: Option<NonZeroU8>,
    pub(super) allowed_updates: Option<Vec<AllowedUpdate>>,
    #[cfg(feature = "tls-server")]
    pub(super) send_certificate: bool,
}

impl ServerSetWebhook {
    #[cfg(feature = "tls-server")]
    pub fn new(send_certificate: bool) -> Self {
        Self {
            max_connections: None,
            allowed_updates: None,
            send_certificate
        }
    }
    #[cfg(not(feature = "tls-server"))]
    pub fn new() -> Self {
        Self {
            max_connections: None,
            allowed_updates: None
        }
    }

    pub fn max_connections(mut self, num: u8) -> Self {
        self.max_connections = unsafe { Some(NonZeroU8::new_unchecked(num)) };
        self
    }

    pub fn allowed_updates(mut self, updates: Vec<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(updates);
        self
    }
}

impl Message for ServerSetWebhook {
    type Result = Result<True, ()>;
}
