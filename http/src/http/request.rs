use super::helper;
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
            // GET /search?name=abc&sort=1 HTTP/1.1\r\n\n\nHeaders\n...

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

        // Parse the first verb
        // using `variable shadowing` to overwrite the original `request`
        let (method, request) =
            helper::get_next_token(request).ok_or(ParseError::InvalidRequest)?;

        // Parse the path
        let (mut path, request) =
            helper::get_next_token(request).ok_or(ParseError::InvalidRequest)?;

        // Parse the protocol
        let (protocol, request) =
            helper::get_next_token(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // after parse, we converted &str -> HttpMethod Enum
        let method: HttpMethod = method.parse()?;

        let mut query_string = None;
        match path.find('?') {
            Some(i) => {
                // to omit the `?`, we simply move the index to i + 1 byte
                query_string = Some(&path[i + 1..]);
                path = &path[..i];
            }
            None => {}
        };

        unimplemented!()
    }
}
