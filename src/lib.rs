extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;

mod methods;
mod types;

use actix::{ActorFuture, Message, WrapFuture};
use actix::{Actor, Addr, Arbiter, AsyncContext, Context, Handler, StreamHandler};
use actix_web::{client, HttpMessage};
use futures::{Future,Stream};
use std::time::{Duration, Instant};
use tokio::timer::Interval;
use types::Update;

pub struct TelegramBot {
    token: String,
    timeout: i32,
    offset: Option<i32>,
    workers: Vec<Addr<TelegramWorker>>,
    threads: u8,
}

impl TelegramBot {
    pub fn new(token: String, timeout: i32) -> Self {
        TelegramBot {
            token,
            timeout,
            offset: None,
            workers: Vec::new(),
            threads: 1,
        }
    }
}

impl Actor for TelegramBot {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("TelegramBot is alive");
        let timeout = Duration::from_secs(self.timeout as u64);

        let workers = (0..self.threads)
            .map(|_i| Arbiter::start(|_a| TelegramWorker::new()))
            .collect();
        self.workers = workers;

        let stream = Interval::new(Instant::now(), timeout).map(|_| PollUpdates);
        ctx.add_stream(stream);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramBot is stopped");
    }
}

pub struct TelegramApi {
    token: String,
}

impl TelegramApi {
    fn new(token: String) -> TelegramApi {
        TelegramApi { token }
    }
}

impl Actor for TelegramApi {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramApi is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramApi is stopped");
    }
}

pub struct TelegramWorker {
    apps: Box<Fn(Update)>,
}

impl TelegramWorker {
    fn new() -> TelegramWorker {
        let app = |a| {
            debug!("TelegramWorker.App {:?}", a);
        };
        TelegramWorker {
            apps: Box::new(app),
        }
    }
}

impl Actor for TelegramWorker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is stopped");
    }
}

#[derive(Serialize, Debug)]
pub struct PollUpdates;

impl StreamHandler<PollUpdates, tokio::timer::Error> for TelegramBot {
    fn handle(&mut self, _msg: PollUpdates, ctx: &mut Context<Self>) {
        let msg = methods::GetUpdates::new(self.timeout, self.offset);
        debug!("TelegramBot.GetUpdates {:?}", msg);

        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        let mut client = client::post(url);
        client
            .header("User-Agent", "actix-web")
            .timeout(Duration::from_secs(self.timeout as u64 + 1));

        let future = client
            .json(msg)
            .unwrap()
            .send()
            .map_err(|e| debug!("request error {:?}", e))
            .and_then(|response| {
                response
                    .json()
                    .map_err(|e| debug!("parsing json error {:?}", e))
            });
        let actor_future = future.into_actor(self).map(
            |response: types::TelegramResponse, actor, _ctx| {
                debug!("response received {:?}", response);
                actor.offset = response.result.last().map(|i| i.update_id + 1);
                for result in response.result {
                    actor.workers[0].do_send(result);
                }
            },
        );
        ctx.wait(actor_future);
    }
}

impl Handler<Update> for TelegramWorker {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: Update, _ctx: &mut Context<Self>) -> Self::Result {
        debug!("TelegramWorker.Update received {:?}", msg);
        (self.apps)(msg);
        Ok(())
    }
}

impl Message for Update {
    type Result = Result<(), ()>;
}
