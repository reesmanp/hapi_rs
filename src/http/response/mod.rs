use super::{HTTPVersion, header::Header, HTTPStatusCodes};
use std::net::{TcpStream, Shutdown};
use std::io::Write;

pub struct Response {
    version: HTTPVersion,
    code: u32,
    reason: String,
    headers: Header,
    body: String,
    stream: Option<TcpStream>,
    written: bool,
    dirty: bool
}

impl Response {
    pub fn new(version: HTTPVersion, code: u32, reason: String, headers: Header, body: String, stream: TcpStream) -> Self {
        Self {
            version,
            code,
            reason,
            headers,
            body,
            stream: Some(stream),
            written: false,
            dirty: false
        }
    }

    pub fn from_stream(stream: TcpStream) -> Self {
        Self {
            version: HTTPVersion::HTTP20,
            code: HTTPStatusCodes::to_int(&HTTPStatusCodes::Ok200),
            reason: HTTPStatusCodes::get_generic_reason(&HTTPStatusCodes::Ok200),
            headers: Header::new(),
            body: String::from(""),
            stream: Some(stream),
            written: false,
            dirty: false
        }
    }

    /**
     * Getters
    */

    pub fn get_version(&self) -> HTTPVersion {
        self.version
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    pub fn get_reason(&self) -> String {
        self.reason.clone()
    }

    pub fn get_headers(&self) -> Header {
        self.headers.clone()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    fn get_response(&self) -> String {
        let no_body_codes = [HTTPStatusCodes::Continue100, HTTPStatusCodes::SwitchingProtocols101, HTTPStatusCodes::NoContent204, HTTPStatusCodes::NotModified304];

        // HTTP standard dictates that response codes of 1XX, 204, and 304 are not allowed bodies
        let body = match no_body_codes.iter().find(|ref x| self.code == HTTPStatusCodes::to_int(&x)) {
            None => self.get_body(),
            Some(_) => String::from("")
        };

        match self.written {
            false => format!(
                "{} {} {}\r\n{}\r\n{}",
                self.version.to_string(),
                self.code,
                self.reason,
                self.headers.get_headers_formatted(),
                body
            ),
            true => format!("{}", body)
        }
    }

    /**
     * Setters
    */

    pub fn set_version(&mut self, version: HTTPVersion) {
        self.version = version;
    }

    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }

    pub fn set_reason(&mut self, reason: String) {
        self.reason = reason;
    }

    pub fn set_default_code_and_reason(&mut self, code: HTTPStatusCodes) {
        self.code = code.to_int();
        self.reason = code.get_generic_reason();
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
        self.dirty = true;
    }

    /**
     * Actions
    */

    /// Write and flush the response to the stream
    pub fn write(&mut self, flush: bool) {
        if cfg!(debug_assertions) {
            println!("Response:\n{}", self.get_response());
        }

        let response = self.get_response();
        match self.stream {
            Some(ref mut tcp) => {
                if flush {
                    tcp.set_nodelay(true).unwrap();
                } else {
                    tcp.set_nodelay(false).unwrap();
                }

                tcp.write(response.as_ref()).unwrap();
                self.written = true;
            },
            None => ()
        }

        self.dirty = false;
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            version: HTTPVersion::HTTP20,
            code: HTTPStatusCodes::to_int(&HTTPStatusCodes::Ok200),
            reason: HTTPStatusCodes::get_generic_reason(&HTTPStatusCodes::Ok200),
            headers: Header::new(),
            body: String::from(""),
            stream: None,
            written: false,
            dirty: false
        }
    }
}

impl Drop for Response {
    fn drop(&mut self) {
        if self.dirty {
            self.write(true);
        }

        match self.stream {
            None => return,
            Some(ref mut tcp) => {
                tcp.shutdown(Shutdown::Both).unwrap();
                return;
            }
        }
    }
}
