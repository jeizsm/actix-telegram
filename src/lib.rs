extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;

pub mod actors;
pub mod methods;
pub mod types;

pub use actors::{TelegramApi, TelegramBot};
