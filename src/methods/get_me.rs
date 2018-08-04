use super::send_request;
use actix::{Context, Handler, Message};
use actors::TelegramApi;
use futures::Future;
use types::TelegramResponse;

#[derive(Serialize, Debug)]
pub struct GetMe;

impl Message for GetMe {
    type Result = Result<TelegramResponse, ()>;
}

impl Handler<GetMe> for TelegramApi {
    type Result = Box<Future<Item = TelegramResponse, Error = ()>>;

    fn handle(&mut self, msg: GetMe, _ctx: &mut Context<Self>) -> Self::Result {
        send_request(&self.token, "getMe", self.timeout, &msg)
    }
}
