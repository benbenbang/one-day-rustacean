pub fn get_next_token(request: &str) -> Option<(&str, &str)> {
    /*
        clumsy approach:

        let mut iter = request.chars();
        loop {
            let item = iter.next();
            match item {
                Some(c) => {}
                None => break,
            }
        }


        usage:

        case 1.

            match request_helper::get_next_token(request) {
                Some((method, request)) => {}
                None => return Err(ParseError::InvalidRequest),
            };

        clumsy :point_up:

        but we can do:
        case 2.

            request_helper::get_next_token(request).ok_or(ParseError::InvalidRequest)

    */

    for (idx, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..idx], &request[idx + 1..]));
        }
    }

    None
}
