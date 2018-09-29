pub mod header;
pub mod request;
pub mod response;

pub struct HTTP;

impl HTTP {
    pub fn get_generic_response_string(code: HTTPStatusCodes, version: HTTPVersion) -> String {
        format!("{:?} {:?} {:?}\r\n\r\n", version, HTTPStatusCodes::to_int(&code), HTTPStatusCodes::get_generic_reason(&code))
    }
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
    c200,
    c400,
    c404,
    c500
}

impl HTTPStatusCodes {
    pub fn get_generic_reason(code: &Self) -> String {
        match code {
            HTTPStatusCodes::c200 => String::from("OK"),
            HTTPStatusCodes::c400 => String::from("BAD REQUEST"),
            HTTPStatusCodes::c404 => String::from("NOT FOUND"),
            HTTPStatusCodes::c500 => String::from("INTERNAL SERVER ERROR")
        }
    }

    pub fn to_int(code: &Self) -> u32 {
        match code {
            HTTPStatusCodes::c200 => 200,
            HTTPStatusCodes::c400 => 400,
            HTTPStatusCodes::c404 => 404,
            HTTPStatusCodes::c500 => 500
        }
    }
}
