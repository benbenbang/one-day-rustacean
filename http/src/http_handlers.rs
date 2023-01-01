// user implementaion of handlers

use std::fs;

use crate::http::{handler::Handler, response::Response, HttpMethod, Request, StatusCode};

pub struct WebsiteHandler {
    pub public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, requst: &Request) -> Response {
        let not_found_response: Option<String> = Some("<h1>404 Not Found!</h1>".to_string());

        match requst.method() {
            HttpMethod::GET => match requst.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                _ => Response::new(StatusCode::NotFound, not_found_response),
            },
            _ => Response::new(StatusCode::NotFound, not_found_response),
        }
    }
}
