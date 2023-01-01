use crate::http::handler::Handler;
use crate::http::response::Response;
use crate::http::{handler, request, StatusCode};
use crate::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // Self & Server are interchangable!
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("running on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // use annotation to break the outer loop from inner loop
        // 'outer: loop {
        //     'inner: loop {}
        // }

        // let tup = (1, "a", {});

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            /*
                                we have several ways to write this block:
                                    1. do &buffer as &[u8]
                                    2. let res: &Result<Request, _> = &buffer[..].try_info();
                                    3. the following

                                what the following block doing is:
                                - the we try to parse from the request
                                - if it's ok, we build the response with status code 200
                                - it it fails, we then build the 400 request response
                                - then we assign the Response::new to response by `let response = match` statement
                            */
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response. got error: {}", e);
                            }
                        }
                        Err(e) => panic!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => panic!("Encounter error: {}", e),
            }
        }
    }
}
