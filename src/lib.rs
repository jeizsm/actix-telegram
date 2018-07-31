extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod methods;
mod types;

use types::TelegramResponse;
use futures::Future;
use std::time::Duration;
use std::time::Instant;
use futures::Stream;
use actix_web::{actix::{Actor, Context, StreamHandler, Handler, Message, WrapFuture, AsyncContext}};
use tokio::timer::Interval;
use methods::GetUpdates;

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
            .map(|_| GetUpdates { allowed_updates: None, limit: None, timeout: None, offset: None })
            .map_err(|e| panic!("interval errored; err={:?}", e));
        Self::add_stream(stream, ctx);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

pub trait TelegramRequest {
    fn send(&self, token: &str) -> Box<Future<Item = TelegramResponse, Error = ()>>;
}

/// Define handler for all messages
impl<T> Handler<T> for Telegram
    where T: TelegramRequest + Message<Result = Result<TelegramResponse, ()>> {
    type Result = Box<Future<Item = TelegramResponse, Error = ()>>;

    fn handle(&mut self, msg: T, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        msg.send(&self.token)
    }
}

impl StreamHandler<GetUpdates, ()> for Telegram {
    fn handle(&mut self, item: GetUpdates, ctx: &mut Context<Telegram>) {
        println!("PING");
        ctx.spawn(
            item.send(&self.token).and_then(|body| {
                println!("Response: {:?}", body);
                Ok(())
            }).into_actor(self)
        );
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        println!("finished");
    }
}
