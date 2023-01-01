use super::helper;
use super::method::HttpMethod;
use super::ParseError;
use super::QueryString;
use std::convert::TryFrom;
use std::str;

// 'buf is the life time of `buf` in the server.rs
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: HttpMethod,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from<'l>(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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
        let (protocol, _) = helper::get_next_token(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // after parse, we converted &str -> HttpMethod Enum
        let method: HttpMethod = method.parse()?;

        /*
            case 1.

                let mut query_string = None;
                match path.find('?') {
                    Some(i) => {
                        // to omit the `?`, we simply move the index to i + 1 byte
                        query_string = Some(&path[i + 1..]);
                        path = &path[..i];
                    }
                    None => {}
                };

            :shrug: tedious again...

            case 2.

                let mut query_string = None;
                let q = path.find("?");
                if q.is_some() {
                    let i = q.unwrap();
                    query_string = Some(&path[i + 1..]);
                    path = &path[..i]
                }

            still a bit smell even we don't need to handle `None` this time

            case 3.
            in rust, we can do pattern match + if condistion
            so we don't need to `unwrap` (which causes panic)
        */

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        };

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}
