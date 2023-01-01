// user implementaion of handlers

use crate::http::{handler::Handler, response::Response, HttpMethod, Request, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, requst: &Request) -> Response {
        let not_found_response: Option<String> = Some("<h1>404 Not Found!</h1>".to_string());

        match requst.method() {
            HttpMethod::GET => match requst.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome!</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hi there!</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, not_found_response),
            },
            _ => Response::new(StatusCode::NotFound, not_found_response),
        }
    }
}
