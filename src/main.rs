#[macro_use]
extern crate actix_web;

use std::{env, io};
use serde::{Deserialize, Serialize};
use actix_web::{middleware, App, HttpServer};

mod constants;
mod likes;
mod response;
mod tweet;
mod schema;

// main.rs is used to route HTTP requests to the right endpoint

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(tweet::list)
            .service(tweet::get)
            .service(tweet::create)
            .service(tweet::delete)
            .service(likes::list)
            .service(likes::plus_one)
            .service(likes::minus_one)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
