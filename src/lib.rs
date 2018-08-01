extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use] extern crate serde_derive;
extern crate serde;

mod methods;
mod types;

use futures::Future;
use std::time::Duration;
use actix_web::{actix::{Actor, Context, Handler, AsyncContext}};
use methods::GetUpdates;

pub struct TelegramBot {
    timeout: i32,
    offset: Option<i32>,
}

impl TelegramBot {
    pub fn new(timeout: i32) -> Self {
        TelegramBot { timeout, offset: None }
    }
}

impl Actor for TelegramBot {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("TelegramBot is alive");
        let timeout = Duration::from_secs(self.timeout as u64);

        ctx.run_interval(timeout, |actor, ctx| {
            let get_updates = GetUpdates::new(actor.timeout, actor.offset);
            ctx.address().do_send(get_updates);
        });
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("TelegramBot is stopped");
    }
}

/// Define handler for all messages
impl Handler<GetUpdates> for TelegramBot {
    type Result = Box<Future<Item = (), Error = ()>>;

    fn handle(&mut self, _msg: GetUpdates, _ctx: &mut Context<Self>) -> Self::Result {
        println!("GetUpdates received");
        Box::new(futures::future::ok(()))
        // msg.send(&self.token)
    }
}
