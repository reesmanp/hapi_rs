pub(crate) mod header;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod util;

pub use self::header::Header;
pub use self::request::Request;
pub use self::response::Response;
pub use self::util::{
    methods::HTTPMethod,
    status_codes::HTTPStatusCodes,
    versions::HTTPVersion
};

pub struct HTTP;

impl HTTP {
    pub fn get_generic_response_string(code: HTTPStatusCodes, version: HTTPVersion) -> String {
        format!("{} {} {}\r\n\r\n", version.to_string(), code.to_int(), code.get_generic_reason())
    }
}
