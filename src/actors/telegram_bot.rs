use super::telegram_worker::TelegramWorker;
use actix::{Actor, ActorFuture, Addr, Arbiter, AsyncContext, Context, StreamHandler, WrapFuture};
use actix_web::{client, HttpMessage};
use futures::{Future, Stream};
use methods::GetUpdates;
use std::time::{Duration, Instant};
use tokio::timer::{self, Interval};
use types::TelegramResponse;

#[derive(Serialize, Debug)]
struct PollUpdates;

pub struct TelegramBot {
    token: String,
    timeout: Duration,
    offset: Option<i32>,
    workers: Vec<Addr<TelegramWorker>>,
    threads: usize,
}

impl TelegramBot {
    pub fn new(token: String, timeout: Duration) -> Self {
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

        let workers = (0..self.threads)
            .map(|_i| Arbiter::start(|_a| TelegramWorker::new()))
            .collect();
        self.workers = workers;

        let stream = Interval::new(Instant::now(), self.timeout).map(|_| PollUpdates);
        ctx.add_stream(stream);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramBot is stopped");
    }
}

impl StreamHandler<PollUpdates, timer::Error> for TelegramBot {
    fn handle(&mut self, _msg: PollUpdates, ctx: &mut Context<Self>) {
        let msg = GetUpdates::new(self.timeout, self.offset);
        debug!("TelegramBot.GetUpdates {:?}", msg);

        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        let mut client = client::post(url);
        client
            .header("User-Agent", "actix-web")
            .timeout(self.timeout);

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
            |response: TelegramResponse, actor, _ctx| {
                debug!("response received {:?}", response);
                actor.offset = response.result.last().map(|i| i.update_id + 1);
                for (i, result) in response.result.into_iter().enumerate() {
                    actor.workers[i % actor.threads].do_send(result);
                }
            },
        );
        ctx.wait(actor_future);
    }
}
