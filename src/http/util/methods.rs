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

    pub fn to_string(&self) -> String {
        match self {
            HTTPMethod::GET => String::from("GET"),
            HTTPMethod::POST => String::from("POST"),
            HTTPMethod::PUT => String::from("PUT"),
            HTTPMethod::PATCH => String::from("PATCH"),
            HTTPMethod::DELETE => String::from("DELETE"),
            HTTPMethod::COPY => String::from("COPY"),
            HTTPMethod::HEAD => String::from("HEAD"),
            HTTPMethod::OPTIONS => String::from("OPTIONS"),
            HTTPMethod::LINK => String::from("LINK"),
            HTTPMethod::UNLINK => String::from("UNLINK"),
            HTTPMethod::PURGE => String::from("PURGE"),
            HTTPMethod::LOCK => String::from("LOCK"),
            HTTPMethod::UNLOCK => String::from("UNLOCK"),
            HTTPMethod::PROPFIND => String::from("PROPFIND"),
            HTTPMethod::VIEW => String::from("VIEW"),
            HTTPMethod::ERR => String::from("ERR")
        }
    }
}