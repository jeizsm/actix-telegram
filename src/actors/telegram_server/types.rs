use actix::{Addr, Message};
use actors::{App, TelegramApi};
use std::num::NonZeroU8;
use std::sync::Arc;
use types::AllowedUpdate;
use types::True;

bitflags! {
    pub(crate) struct OptionFlags: u8 {
        const SEND_SET_WEBHOOK = 0b00000001;
        #[cfg(feature = "tls-server")]
        const SELF_SIGNED = 0b00000010;
    }
}

impl Default for OptionFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl OptionFlags {
    fn new() -> Self {
        OptionFlags::SEND_SET_WEBHOOK
    }
}

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
    pub fn new() -> Self {
        Self {
            max_connections: None,
            allowed_updates: None,
            #[cfg(feature = "tls-server")]
            send_certificate: false,
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

    #[cfg(feature = "tls-server")]
    pub fn send_certificate(mut self, send_certificate: bool) -> Self {
        self.send_certificate = send_certificate;
        self
    }
}

impl Message for ServerSetWebhook {
    type Result = Result<True, ()>;
}
