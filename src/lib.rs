extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_telegram_derive;
extern crate multipart_rfc7578;
#[cfg(all(feature = "tls", feature = "tls-server"))]
extern crate native_tls;
#[cfg(all(feature = "alpn", feature = "tls-server"))]
extern crate openssl;
#[cfg(all(feature = "rust-tls", feature = "tls-server"))]
extern crate rustls;

pub mod actors;
pub mod methods;
pub mod types;

pub use actors::{App, TelegramApi, TelegramBot};

#[cfg(feature = "server")]
pub use actors::TelegramServer;
