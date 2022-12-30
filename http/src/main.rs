use http::HttpMethod;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let addr: &str = "0.0.0.0:8080";
    let method = HttpMethod::GET;

    // let req = Request {
    //     path: "/user".to_string(),
    //     method: method,
    //     query_string: "id=10",
    // };

    let server = Server::new(addr.to_string());
    server.run();
}
