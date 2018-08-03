use super::send_request;
use super::TelegramRequest;
use actix::Message;
use futures::Future;
use types::TelegramResponse;

#[derive(Serialize, Debug)]
pub struct GetMe;

impl Message for GetMe {
    type Result = Result<TelegramResponse, ()>;
}

impl TelegramRequest for GetMe {
    fn send(&self, token: &str) -> Box<Future<Item = TelegramResponse, Error = ()>> {
        send_request(token, "getMe", self)
    }
}