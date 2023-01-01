#![allow(dead_code, unused)]

use http::handler::Handler;
use http::HttpMethod;
use http::Request;
use http_handlers::WebsiteHandler;
use server::Server;
use std::env;

mod http;
mod http_handlers;
mod server;

fn main() {
    let addr: &str = "0.0.0.0:8080";
    let method = HttpMethod::GET;
    let default_pb_path = env!("CARGO_MANIFEST_DIR").to_string();
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_pb_path);

    // let req = Request {
    //     path: "/user".to_string(),
    //     method: method,
    //     query_string: "id=10",
    // };

    let server = Server::new(addr.to_string());
    server.run(WebsiteHandler { public_path });
}
