use actix::Addr;
use actors::TelegramApi;
use application::App;
use std::sync::Arc;

bitflags! {
    pub(crate) struct OptionFlags: u8 {
        const SEND_SET_WEBHOOK = 0b0000_0001;
        #[cfg(feature = "tls-server")]
        const SELF_SIGNED = 0b0000_0010;
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
