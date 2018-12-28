extern crate actix;
extern crate actix_net;
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
#[cfg(all(feature = "ssl", feature = "tls-server"))]
extern crate openssl;
#[cfg(all(feature = "rust-tls", feature = "tls-server"))]
extern crate rustls;
#[macro_use]
extern crate bitflags;

pub mod actors;
pub mod methods;
pub(crate) mod raw;
pub mod types;
pub mod application;

pub use crate::actors::{TelegramApi, TelegramBot};
pub use crate::application::App;

#[cfg(feature = "server")]
pub use crate::actors::TelegramServer;
