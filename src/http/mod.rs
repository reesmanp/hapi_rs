pub mod request;
pub mod header;

use self::request::Request;

pub struct HTTP;

impl HTTP {
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
}

pub enum HTTPStatusCodes {
    c200,
    c400,
    c500
}

impl HTTPStatusCodes {
    pub fn get_generic_response(code: Self) -> String {
        match code {
            HTTPStatusCodes::c200 => String::from("HTTP/2.0 200 OK\r\n\r\n"),
            HTTPStatusCodes::c400 => String::from("HTTP/2.0 400 BAD REQUEST\r\n\r\n"),
            HTTPStatusCodes::c500 => String::from("HTTP/2.0 500 INTERNAL SERVER ERROR\r\n\r\n")
        }
    }
}
