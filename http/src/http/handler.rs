use super::{response::Response, ParseError, Request};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    // default handler, can be overwritten by users
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {e}");
        Response::new(super::StatusCode::BadRequest, None)
    }
}
