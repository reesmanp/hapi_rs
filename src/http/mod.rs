pub mod header;
pub mod request;
pub mod response;

pub struct HTTP;

impl HTTP {
    pub fn get_generic_response_string(code: HTTPStatusCodes, version: HTTPVersion) -> String {
        format!("{:?} {:?} {:?}\r\n\r\n", version, HTTPStatusCodes::to_int(&code), HTTPStatusCodes::get_generic_reason(&code))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    COPY,
    HEAD,
    OPTIONS,
    LINK,
    UNLINK,
    PURGE,
    LOCK,
    UNLOCK,
    PROPFIND,
    VIEW,
    ERR
}

impl HTTPMethod {
    pub fn from_bytes(buffer: &[u8]) -> Self {
        match buffer[..] {
            [71, 69, 84] => HTTPMethod::GET,
            [80, 79, 83, 84] => HTTPMethod::POST,
            [80, 85, 84] => HTTPMethod::PUT,
            [80, 65, 84, 67, 72] => HTTPMethod::PATCH,
            [68, 69, 76, 69, 84, 69] => HTTPMethod::DELETE,
            [67, 79, 80, 89] => HTTPMethod::COPY,
            [72, 69, 65, 68] => HTTPMethod::HEAD,
            [79, 80, 84, 73, 79, 78, 83] => HTTPMethod::OPTIONS,
            [76, 73, 78, 75] => HTTPMethod::LINK,
            [85, 78, 76, 73, 78, 75] => HTTPMethod::UNLINK,
            [80, 85, 82, 71, 69] => HTTPMethod::PURGE,
            [76, 79, 67, 75] => HTTPMethod::LOCK,
            [85, 78, 76, 79, 67, 75] => HTTPMethod::UNLOCK,
            [80, 82, 79, 80, 70, 73, 78, 68] => HTTPMethod::PROPFIND,
            [86, 73, 69, 87] => HTTPMethod::VIEW,
            _ => HTTPMethod::ERR
        }
    }

    pub fn from_str(buffer: &str) -> Self {
        match buffer {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            "PUT" => HTTPMethod::PUT,
            "PATCH" => HTTPMethod::PATCH,
            "DELETE" => HTTPMethod::DELETE,
            "COPY" => HTTPMethod::COPY,
            "HEAD" => HTTPMethod::HEAD,
            "OPTIONS" => HTTPMethod::OPTIONS,
            "LINK" => HTTPMethod::LINK,
            "UNLINK" => HTTPMethod::UNLINK,
            "PURGE" => HTTPMethod::PURGE,
            "LOCK" => HTTPMethod::LOCK,
            "UNLOCK" => HTTPMethod::UNLOCK,
            "PROPFIND" => HTTPMethod::PROPFIND,
            "VIEW" => HTTPMethod::VIEW,
            _ => HTTPMethod::ERR
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HTTPVersion {
    HTTP10,
    HTTP11,
    HTTP20,
    ERR
}

impl HTTPVersion {
    pub fn from_bytes(buffer: &[u8]) -> Self {
        match buffer[..] {
            [72, 84, 84, 80, 47, 49, 46, 48] => HTTPVersion::HTTP10,
            [72, 84, 84, 80, 47, 49, 46, 49] => HTTPVersion::HTTP11,
            [72, 84, 84, 80, 47, 50, 46, 48] => HTTPVersion::HTTP20,
            _ => HTTPVersion::ERR
        }
    }

    pub fn from_str(buffer: &str) -> Self {
        match buffer {
            "HTTP/1.0" => HTTPVersion::HTTP10,
            "HTTP/1.1" => HTTPVersion::HTTP11,
            "HTTP/2.0" => HTTPVersion::HTTP20,
            _ => HTTPVersion::ERR
        }
    }

    pub fn to_string(version: Self) -> String {
        match version {
            HTTPVersion::HTTP10 => String::from("HTTP/1.0"),
            HTTPVersion::HTTP11 => String::from("HTTP/1.1"),
            HTTPVersion::HTTP20 => String::from("HTTP/2.0"),
            HTTPVersion::ERR => String::from("")
        }
    }
}

pub enum HTTPStatusCodes {
    c100, c101,
    c200, c201, c202, c203,
    c204, c205, c206,
    c300, c301, c302, c303, c304, c305, c306, c307,
    c400, c401, c402, c403, c404, c405, c406, c407, c408, c409, c410, c411, c412, c413, c414, c415, c416, c417, c418,
    c500, c501, c502, c503, c504, c505
}

impl HTTPStatusCodes {
    pub fn get_generic_reason(code: &Self) -> String {
        match code {
            HTTPStatusCodes::c100 => String::from("Continue"),
            HTTPStatusCodes::c101 => String::from("Switching Protocols"),

            HTTPStatusCodes::c200 => String::from("OK"),
            HTTPStatusCodes::c201 => String::from("Created"),
            HTTPStatusCodes::c202 => String::from("Accepted"),
            HTTPStatusCodes::c203 => String::from("Non-Authoritative Information"),
            HTTPStatusCodes::c204 => String::from("No Content"),
            HTTPStatusCodes::c205 => String::from("Reset Content"),
            HTTPStatusCodes::c206 => String::from("Partial Content"),

            HTTPStatusCodes::c300 => String::from("Multiple Choices"),
            HTTPStatusCodes::c301 => String::from("Moved Permanently"),
            HTTPStatusCodes::c302 => String::from("Found"),
            HTTPStatusCodes::c303 => String::from("See Other"),
            HTTPStatusCodes::c304 => String::from("Not Modified"),
            HTTPStatusCodes::c305 => String::from("Use Proxy"),
            HTTPStatusCodes::c306 => String::from("Switch Proxy"),
            HTTPStatusCodes::c307 => String::from("Temporary Redirect"),

            HTTPStatusCodes::c400 => String::from("Bad Request"),
            HTTPStatusCodes::c401 => String::from("Unauthorized"),
            HTTPStatusCodes::c402 => String::from("Payment Required"),
            HTTPStatusCodes::c403 => String::from("Forbidden"),
            HTTPStatusCodes::c404 => String::from("Not Found"),
            HTTPStatusCodes::c405 => String::from("Method Not Allowed"),
            HTTPStatusCodes::c406 => String::from("Not Acceptable"),
            HTTPStatusCodes::c407 => String::from("Proxy Authentication Required"),
            HTTPStatusCodes::c408 => String::from("Request Time-out"),
            HTTPStatusCodes::c409 => String::from("Conflict"),
            HTTPStatusCodes::c410 => String::from("Gone"),
            HTTPStatusCodes::c411 => String::from("Length Required"),
            HTTPStatusCodes::c412 => String::from("Precondition Failed"),
            HTTPStatusCodes::c413 => String::from("Request Entity Too Large"),
            HTTPStatusCodes::c414 => String::from("Request-URI Too Large"),
            HTTPStatusCodes::c415 => String::from("Unsupported Media Type"),
            HTTPStatusCodes::c416 => String::from("Request Range Not Satisfiable"),
            HTTPStatusCodes::c417 => String::from("Expectation Failed"),
            HTTPStatusCodes::c418 => String::from("I'm a teapot"),

            HTTPStatusCodes::c500 => String::from("Internal Server Error"),
            HTTPStatusCodes::c501 => String::from("Not Implemented"),
            HTTPStatusCodes::c502 => String::from("Bad Gateway"),
            HTTPStatusCodes::c503 => String::from("Service Unavailable"),
            HTTPStatusCodes::c504 => String::from("Gateway Time-out"),
            HTTPStatusCodes::c505 => String::from("HTTP Version Not Supported")
        }
    }

    pub fn to_int(code: &Self) -> u32 {
        match code {
            HTTPStatusCodes::c100 => 100,
            HTTPStatusCodes::c101 => 101,

            HTTPStatusCodes::c200 => 200,
            HTTPStatusCodes::c201 => 201,
            HTTPStatusCodes::c202 => 202,
            HTTPStatusCodes::c203 => 203,
            HTTPStatusCodes::c204 => 204,
            HTTPStatusCodes::c205 => 205,
            HTTPStatusCodes::c206 => 206,

            HTTPStatusCodes::c300 => 300,
            HTTPStatusCodes::c301 => 301,
            HTTPStatusCodes::c302 => 302,
            HTTPStatusCodes::c303 => 303,
            HTTPStatusCodes::c304 => 304,
            HTTPStatusCodes::c305 => 305,
            HTTPStatusCodes::c306 => 306,
            HTTPStatusCodes::c307 => 307,

            HTTPStatusCodes::c400 => 400,
            HTTPStatusCodes::c401 => 401,
            HTTPStatusCodes::c402 => 402,
            HTTPStatusCodes::c403 => 403,
            HTTPStatusCodes::c404 => 404,
            HTTPStatusCodes::c405 => 405,
            HTTPStatusCodes::c406 => 406,
            HTTPStatusCodes::c407 => 407,
            HTTPStatusCodes::c408 => 408,
            HTTPStatusCodes::c409 => 409,
            HTTPStatusCodes::c410 => 410,
            HTTPStatusCodes::c411 => 411,
            HTTPStatusCodes::c412 => 412,
            HTTPStatusCodes::c413 => 413,
            HTTPStatusCodes::c414 => 414,
            HTTPStatusCodes::c415 => 415,
            HTTPStatusCodes::c416 => 416,
            HTTPStatusCodes::c417 => 417,
            HTTPStatusCodes::c418 => 418,

            HTTPStatusCodes::c500 => 500,
            HTTPStatusCodes::c501 => 501,
            HTTPStatusCodes::c502 => 502,
            HTTPStatusCodes::c503 => 503,
            HTTPStatusCodes::c504 => 504,
            HTTPStatusCodes::c505 => 505
        }
    }
}
