pub(crate) mod header;
pub(crate) mod request;
pub(crate) mod response;

pub use self::header::Header;
pub use self::request::Request;
pub use self::response::Response;

pub struct HTTP;

impl HTTP {
    pub fn get_generic_response_string(code: HTTPStatusCodes, version: HTTPVersion) -> String {
        format!("{} {} {}\r\n\r\n", version.to_string(), code.to_int(), code.get_generic_reason())
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

    pub fn to_string(&self) -> String {
        match self {
            HTTPVersion::HTTP10 => String::from("HTTP/1.0"),
            HTTPVersion::HTTP11 => String::from("HTTP/1.1"),
            HTTPVersion::HTTP20 => String::from("HTTP/2.0"),
            HTTPVersion::ERR => String::from("")
        }
    }
}

#[allow(dead_code)]
pub enum HTTPStatusCodes {
    // 1xx Informational Response
    Continue100,
    SwitchingProtocols101,
    Processing102,
    EarlyHints103,

    // 2xx Success
    Ok200,
    Created201,
    Accepted202,
    NonAuthoritativeInformation203,
    NoContent204,
    ResetContent205,
    PartialContent206,
    MultiStatus207,
    AlreadyReported208,
    IMUsed226,

    // 3xx Redirection
    MultipleChoices300,
    MovedPermanently301,
    Found302,
    SeeOther303,
    NotModified304,
    UseProxy305,
    SwitchProxy306,
    TemporaryRedirect307,
    PermanentRedirect308,

    // 4xx Client Errors
    BadRequest400,
    Unauthorized401,
    PaymentRequired402,
    Forbidden403,
    NotFound404,
    MethodNotAllowed405,
    NotAcceptable406,
    ProxyAuthenticationRequired407,
    RequestTimeout408,
    Conflict409,
    Gone410,
    LengthRequired411,
    PreconditionFailed412,
    PayloadTooLarge413,
    URITooLong414,
    UnsupportedMediaType415,
    RangeNotSatisfiable416,
    ExpectationFailed417,
    ImATeapot418,
    MisdirectedRequest421,
    UnprocessableEntity422,
    Locked423,
    FailedDependency424,
    UpgradeRequired426,
    PreconditionRequired428,
    TooManyRequests429,
    RequestHeaderFieldsTooLarge431,
    UnavailableForLegalReasons451,

    // 5xx Server Errors
    InternalServerError500,
    NotImplemented501,
    BadGateway502,
    ServiceUnavailable503,
    GatewayTimeout504,
    HTTPVersionNotSupported505,
    VariantAlsoNegotiates506,
    InsufficientStorage507,
    LoopDetected508,
    NotExtended510,
    NetworkAuthenticationRequired511
}

impl HTTPStatusCodes {
    pub fn get_generic_reason(&self) -> String {
        match self {
            HTTPStatusCodes::Continue100 => String::from("Continue"),
            HTTPStatusCodes::SwitchingProtocols101 => String::from("Switching Protocols"),
            HTTPStatusCodes::Processing102 => String::from("Processing"),
            HTTPStatusCodes::EarlyHints103 => String::from("Early Hints"),

            HTTPStatusCodes::Ok200 => String::from("OK"),
            HTTPStatusCodes::Created201 => String::from("Created"),
            HTTPStatusCodes::Accepted202 => String::from("Accepted"),
            HTTPStatusCodes::NonAuthoritativeInformation203 => String::from("Non-Authoritative Information"),
            HTTPStatusCodes::NoContent204 => String::from("No Content"),
            HTTPStatusCodes::ResetContent205 => String::from("Reset Content"),
            HTTPStatusCodes::PartialContent206 => String::from("Partial Content"),
            HTTPStatusCodes::MultiStatus207 => String::from("Multi-Status"),
            HTTPStatusCodes::AlreadyReported208 => String::from("Already Reported"),
            HTTPStatusCodes::IMUsed226 => String::from("IM Used"),

            HTTPStatusCodes::MultipleChoices300 => String::from("Multiple Choices"),
            HTTPStatusCodes::MovedPermanently301 => String::from("Moved Permanently"),
            HTTPStatusCodes::Found302 => String::from("Found"),
            HTTPStatusCodes::SeeOther303 => String::from("See Other"),
            HTTPStatusCodes::NotModified304 => String::from("Not Modified"),
            HTTPStatusCodes::UseProxy305 => String::from("Use Proxy"),
            HTTPStatusCodes::SwitchProxy306 => String::from("Switch Proxy"),
            HTTPStatusCodes::TemporaryRedirect307 => String::from("Temporary Redirect"),
            HTTPStatusCodes::PermanentRedirect308 => String::from("Permanent Redirect"),

            HTTPStatusCodes::BadRequest400 => String::from("Bad Request"),
            HTTPStatusCodes::Unauthorized401 => String::from("Unauthorized"),
            HTTPStatusCodes::PaymentRequired402 => String::from("Payment Required"),
            HTTPStatusCodes::Forbidden403 => String::from("Forbidden"),
            HTTPStatusCodes::NotFound404 => String::from("Not Found"),
            HTTPStatusCodes::MethodNotAllowed405 => String::from("Method Not Allowed"),
            HTTPStatusCodes::NotAcceptable406 => String::from("Not Acceptable"),
            HTTPStatusCodes::ProxyAuthenticationRequired407 => String::from("Proxy Authentication Required"),
            HTTPStatusCodes::RequestTimeout408 => String::from("Request Time-out"),
            HTTPStatusCodes::Conflict409 => String::from("Conflict"),
            HTTPStatusCodes::Gone410 => String::from("Gone"),
            HTTPStatusCodes::LengthRequired411 => String::from("Length Required"),
            HTTPStatusCodes::PreconditionFailed412 => String::from("Precondition Failed"),
            HTTPStatusCodes::PayloadTooLarge413 => String::from("Request Entity Too Large"),
            HTTPStatusCodes::URITooLong414 => String::from("Request-URI Too Large"),
            HTTPStatusCodes::UnsupportedMediaType415 => String::from("Unsupported Media Type"),
            HTTPStatusCodes::RangeNotSatisfiable416 => String::from("Request Range Not Satisfiable"),
            HTTPStatusCodes::ExpectationFailed417 => String::from("Expectation Failed"),
            HTTPStatusCodes::ImATeapot418 => String::from("I'm a teapot"),
            HTTPStatusCodes::MisdirectedRequest421 => String::from("Misdirected Request"),
            HTTPStatusCodes::UnprocessableEntity422 => String::from("Unprocessable Entity"),
            HTTPStatusCodes::Locked423 => String::from("Locked"),
            HTTPStatusCodes::FailedDependency424 => String::from("Failed Dependency"),
            HTTPStatusCodes::UpgradeRequired426 => String::from("Upgrade Required"),
            HTTPStatusCodes::PreconditionRequired428 => String::from("Precondition Required"),
            HTTPStatusCodes::TooManyRequests429 => String::from("Too Many Requests"),
            HTTPStatusCodes::RequestHeaderFieldsTooLarge431 => String::from("Request Header Fields Too Large"),
            HTTPStatusCodes::UnavailableForLegalReasons451 => String::from("Unavailable For Legal Reasons"),

            HTTPStatusCodes::InternalServerError500 => String::from("Internal Server Error"),
            HTTPStatusCodes::NotImplemented501 => String::from("Not Implemented"),
            HTTPStatusCodes::BadGateway502 => String::from("Bad Gateway"),
            HTTPStatusCodes::ServiceUnavailable503 => String::from("Service Unavailable"),
            HTTPStatusCodes::GatewayTimeout504 => String::from("Gateway Time-out"),
            HTTPStatusCodes::HTTPVersionNotSupported505 => String::from("HTTP Version Not Supported"),
            HTTPStatusCodes::VariantAlsoNegotiates506 => String::from("Variant Also Negotiates"),
            HTTPStatusCodes::InsufficientStorage507 => String::from("Insufficient Storage"),
            HTTPStatusCodes::LoopDetected508 => String::from("Loop Detected"),
            HTTPStatusCodes::NotExtended510 => String::from("Not Extended"),
            HTTPStatusCodes::NetworkAuthenticationRequired511 => String::from("Network Authentication Required")
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            HTTPStatusCodes::Continue100 => 100,
            HTTPStatusCodes::SwitchingProtocols101 => 101,
            HTTPStatusCodes::Processing102 => 102,
            HTTPStatusCodes::EarlyHints103 => 103,

            HTTPStatusCodes::Ok200 => 200,
            HTTPStatusCodes::Created201 => 201,
            HTTPStatusCodes::Accepted202 => 202,
            HTTPStatusCodes::NonAuthoritativeInformation203 => 203,
            HTTPStatusCodes::NoContent204 => 204,
            HTTPStatusCodes::ResetContent205 => 205,
            HTTPStatusCodes::PartialContent206 => 206,
            HTTPStatusCodes::MultiStatus207 => 207,
            HTTPStatusCodes::AlreadyReported208 => 208,
            HTTPStatusCodes::IMUsed226 => 226,

            HTTPStatusCodes::MultipleChoices300 => 300,
            HTTPStatusCodes::MovedPermanently301 => 301,
            HTTPStatusCodes::Found302 => 302,
            HTTPStatusCodes::SeeOther303 => 303,
            HTTPStatusCodes::NotModified304 => 304,
            HTTPStatusCodes::UseProxy305 => 305,
            HTTPStatusCodes::SwitchProxy306 => 306,
            HTTPStatusCodes::TemporaryRedirect307 => 307,
            HTTPStatusCodes::PermanentRedirect308 => 308,

            HTTPStatusCodes::BadRequest400 => 400,
            HTTPStatusCodes::Unauthorized401 => 401,
            HTTPStatusCodes::PaymentRequired402 => 402,
            HTTPStatusCodes::Forbidden403 => 403,
            HTTPStatusCodes::NotFound404 => 404,
            HTTPStatusCodes::MethodNotAllowed405 => 405,
            HTTPStatusCodes::NotAcceptable406 => 406,
            HTTPStatusCodes::ProxyAuthenticationRequired407 => 407,
            HTTPStatusCodes::RequestTimeout408 => 408,
            HTTPStatusCodes::Conflict409 => 409,
            HTTPStatusCodes::Gone410 => 410,
            HTTPStatusCodes::LengthRequired411 => 411,
            HTTPStatusCodes::PreconditionFailed412 => 412,
            HTTPStatusCodes::PayloadTooLarge413 => 413,
            HTTPStatusCodes::URITooLong414 => 414,
            HTTPStatusCodes::UnsupportedMediaType415 => 415,
            HTTPStatusCodes::RangeNotSatisfiable416 => 416,
            HTTPStatusCodes::ExpectationFailed417 => 417,
            HTTPStatusCodes::ImATeapot418 => 418,
            HTTPStatusCodes::MisdirectedRequest421 => 421,
            HTTPStatusCodes::UnprocessableEntity422 => 422,
            HTTPStatusCodes::Locked423 => 423,
            HTTPStatusCodes::FailedDependency424 => 424,
            HTTPStatusCodes::UpgradeRequired426 => 426,
            HTTPStatusCodes::PreconditionRequired428 => 428,
            HTTPStatusCodes::TooManyRequests429 => 429,
            HTTPStatusCodes::RequestHeaderFieldsTooLarge431 => 431,
            HTTPStatusCodes::UnavailableForLegalReasons451 => 451,

            HTTPStatusCodes::InternalServerError500 => 500,
            HTTPStatusCodes::NotImplemented501 => 501,
            HTTPStatusCodes::BadGateway502 => 502,
            HTTPStatusCodes::ServiceUnavailable503 => 503,
            HTTPStatusCodes::GatewayTimeout504 => 504,
            HTTPStatusCodes::HTTPVersionNotSupported505 => 505,
            HTTPStatusCodes::VariantAlsoNegotiates506 => 506,
            HTTPStatusCodes::InsufficientStorage507 => 507,
            HTTPStatusCodes::LoopDetected508 => 508,
            HTTPStatusCodes::NotExtended510 => 510,
            HTTPStatusCodes::NetworkAuthenticationRequired511 => 511
        }
    }
}
