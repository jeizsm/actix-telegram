extern crate actix_web;
extern crate futures;
#[macro_use] extern crate serde_derive;

use futures::future::Future;
use actix_web::{actix, actix::System, client, HttpMessage};
use std::env;

#[derive(Deserialize, Debug)]
struct Me {
    id: i64,
    is_bot: bool,
    first_name: String,
    username: String,
}

#[derive(Deserialize, Debug)]
struct GetMe {
    ok: bool,
    result: Me,
}

fn main() {
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let method = "getMe";
    let url = format!("https://api.telegram.org/bot{}/{}", token, method);
    actix::run(||
        client::get(url)   // <- Create request builder
            .header("User-Agent", "Actix-web")
            .finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|_| ())
            .and_then(|response| {                // <- server http response
                response
                    .json()
                    .map_err(|_| ())
                    .and_then(|body: GetMe| {
                        println!("Response: {:?}", body);
                        System::current().stop();
                        Ok(())
                    })
            })
    );
}
