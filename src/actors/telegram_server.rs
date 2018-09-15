use super::{App, TelegramApi};
use actix::{Actor, Addr, Context, Handler, Message};
use actix_web::{
    http::Method,
    server::{HttpHandler, HttpServer, Server},
    App as ActixApp, HttpResponse, Json, State,
};
use futures::Future;
use methods::SetWebhook;
use rustls::{
    internal::pemfile::{certs, rsa_private_keys},
    NoClientAuth, ServerConfig,
};
use std::fs::File;
use std::io::BufReader;
use std::num::NonZeroU8;
use std::sync::Arc;
use types::{AllowedUpdate, InputFile, True, Update};

struct ReqState {
    telegram_api: Addr<TelegramApi>,
    apps: Arc<Vec<App>>,
}

pub struct TelegramServer {
    addr: String,
    host: Option<String>,
    url: Option<String>,
    token: String,
    threads: usize,
    apps: Arc<Vec<App>>,
    server: Option<Addr<Server>>,
    certificate: Option<String>,
    key: Option<String>,
}

impl TelegramServer {
    pub fn new(addr: String, token: String, apps: Vec<App>) -> Self {
        TelegramServer {
            addr,
            host: None,
            url: None,
            threads: 1,
            apps: Arc::new(apps),
            server: None,
            token,
            certificate: None,
            key: None,
        }
    }

    pub fn workers(mut self, num: usize) -> Self {
        self.threads = num;
        self
    }

    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn set_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    fn url(&self) -> &str {
        self.url.as_ref().map_or(&self.token, |url| url)
    }

    pub fn certificate_and_key(mut self, certificate: String, key: String) -> Self {
        self.certificate = Some(certificate);
        self.key = Some(key);
        self
    }
}

impl Actor for TelegramServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramServer is alive");
        let token = self.token.clone();
        let url = self.url().to_owned();
        let apps = self.apps.clone();
        let mut server = HttpServer::new(move || {
            let telegram_api = TelegramApi::new((&token).to_owned(), 10).start();
            let apps = apps.clone();
            let state = ReqState { telegram_api, apps };
            ActixApp::with_state(state).resource(&url, |r| {
                r.method(Method::POST)
                    .with(|(update, state): (Json<Update>, State<ReqState>)| {
                        let mut msg = update.into_inner();
                        debug!("TelegramServer.Update received {:?}", msg);
                        for app in state.apps.iter() {
                            msg = match (app.0)(msg, &state.telegram_api) {
                                Ok(()) => {
                                    debug!("ok");
                                    return HttpResponse::Ok();
                                }
                                Err(msg) => {
                                    debug!("next");
                                    msg
                                }
                            };
                        }
                        HttpResponse::Ok()
                    })
            })
        }).workers(self.threads);
        if let Some(host) = self.host.clone() {
            server = server.server_hostname(host);
        }
        match (self.certificate.as_ref(), self.key.as_ref()) {
            (Some(certificate), Some(key)) => {
                let cert_file = &mut BufReader::new(File::open(certificate).unwrap());
                let key_file = &mut BufReader::new(File::open(key).unwrap());
                server = set_cert_for_server(server, self.addr.clone(), cert_file, key_file)
            }
            _ => {
                server = server.bind(self.addr.clone()).unwrap();
            }
        }
        self.server = Some(server.start());
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramServer is stopped");
    }
}

fn set_cert_for_server<H>(
    server: HttpServer<H>,
    addr: String,
    cert_file: &mut BufReader<File>,
    key_file: &mut BufReader<File>,
) -> HttpServer<H>
where
    H: HttpHandler,
{
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    server.bind_rustls(addr, config).unwrap()
}

#[derive(Default)]
pub struct ServerSetWebhook {
    max_connections: Option<NonZeroU8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl ServerSetWebhook {
    pub fn new() -> Self {
        Self {
            max_connections: None,
            allowed_updates: None,
        }
    }

    pub fn max_connections(mut self, num: u8) -> Self {
        self.max_connections = unsafe { Some(NonZeroU8::new_unchecked(num)) };
        self
    }

    pub fn allowed_updates(mut self, updates: Vec<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(updates);
        self
    }
}

impl Message for ServerSetWebhook {
    type Result = Result<True, ()>;
}

impl Handler<ServerSetWebhook> for TelegramServer {
    type Result = Box<Future<Item = True, Error = ()>>;

    fn handle(&mut self, msg: ServerSetWebhook, _: &mut Context<Self>) -> Self::Result {
        let telegram_api = TelegramApi::new(self.token.clone(), 10).start();
        let url = self
            .host
            .as_ref()
            .map_or(String::new(), |host| format!("{}/{}", host, self.url()));
        let set_webhook = SetWebhook {
            url,
            certificate: self.certificate.as_ref().map(|path| InputFile::Disk {
                path: path.to_owned(),
            }),
            max_connections: msg.max_connections,
            allowed_updates: msg.allowed_updates,
        };
        Box::new(
            telegram_api
                .send(set_webhook)
                .map_err(|_| ())
                .and_then(|response| response),
        )
    }
}
