extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use] extern crate serde_derive;
extern crate serde;

mod methods;
mod types;

use std::time::Duration;
use std::time::Instant;
use futures::Stream;
use actix_web::{actix::{Actor, Context, StreamHandler}};
use tokio::timer::Interval;
use methods::GetMe;

// Define actor
pub struct Telegram {
    token: String,
}

impl Telegram {
    pub fn new(token: String) -> Self {
        Telegram { token: token }
    }
}

// Provide Actor implementation for our actor
impl Actor for Telegram {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
        let stream = Interval::new(Instant::now(), Duration::from_secs(10))
            .map(|_| GetMe)
            .map_err(|e| panic!("interval errored; err={:?}", e));
        Self::add_stream(stream, ctx);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}
