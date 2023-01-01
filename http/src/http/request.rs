use super::method::HttpMethod;
use super::ParseError;
use std::convert::TryFrom;
use std::str;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: HttpMethod,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*
            try_from: to parse the http request string.

            example:
            // GET /search?name=abc&sort=1 HTTP/1.1

            original implemetations:
            case1: match str::from_utf8(buf).expect(&ParseError::InvalidEncoding.message()){
                Ok(request) => {}
                Err(_) => return Err(ParseError::InvalidEncoding)
            };

            but since it's a commom pattern in rs, there's an `or` for this pattern match. See case 2

            case2: let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

            with  an `or`, a `?` and a proper import for the error, we can make the pattern match into a one liner.
            but what if we want to make it even more cleaner?

            in case 3, which is the following implmentation, we removed the `or` and the `?`, the comiplier would complain
            that it cannot return a utf8error. so what we can do here is implementing a `utf8error` for our parse error. see
            the `error.rs` file.
        */
        let request = str::from_utf8(buf)?;

        unimplemented!()
    }
}
