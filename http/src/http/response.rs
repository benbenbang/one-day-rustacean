use std::{
    fmt::{Display, Formatter},
    io::{Result, Write},
    net::TcpStream,
};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    /*
        dyn dispatch -> accept anything that has a good implementation of write, but it doesn't know which
        `write` function it should look for (because there are many kind of `write` functions). it need to
        try to figure out which `write` function in the runtime given by the trait. (by vtable) This will cause
        runtime overhead.

            pub fn send(&self, stream: &mut dyn Write) -> Result<()> {}

        static dispatch -> accept anything that has a good implementation of write. the difference between dyn dispatch
        is that it will check in the complie time. (and no need to call the vtable). The downside is that it takes longer
        to compile and the binary is larger, since compiler copy the function and implement for that trait. it might be
        an issue for the embedded system, but ok if it's in a server for instance.

            pub fn send(&self, stream: &mut impl Write) -> Result<()> {}
    */
    pub fn send(&self, stream: &mut impl Write) -> Result<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP 1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
