use super::{HTTPVersion, header::Header, HTTPStatusCodes};
use std::net::TcpStream;
use std::io::Write;

pub struct Response {
    version: HTTPVersion,
    code: u32,
    reason: String,
    headers: Header,
    stream: Option<TcpStream>
}

impl Response {
    pub fn new(version: HTTPVersion, code: u32, reason: String, headers: Header, stream: TcpStream) -> Self {
        Self {
            version,
            code,
            reason,
            headers,
            stream: Some(stream)
        }
    }

    pub fn from_stream(stream: TcpStream) -> Self {
        Self {
            version: HTTPVersion::HTTP20,
            code: HTTPStatusCodes::to_int(&HTTPStatusCodes::c200),
            reason: HTTPStatusCodes::get_generic_reason(&HTTPStatusCodes::c200),
            headers: Header::new(),
            stream: Some(stream)
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

    fn get_response(&self) -> String {
        format!(
            "{} {} {}\r\n{}\r\n",
            HTTPVersion::to_string(self.version),
            self.code,
            self.reason,
            self.headers.get_headers_formatted() // TODO: Need body
        )
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

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    /**
     * Actions
    */

    pub fn flush(&mut self) {
        // Write and flush the response to the stream
        println!("Response:\n{}", self.get_response());
        let response = self.get_response();
        match self.stream {
            Some(ref mut tcp) => {
                tcp.write(response.as_ref()).unwrap();
                tcp.flush().unwrap();
            },
            None => ()
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            version: HTTPVersion::HTTP20,
            code: HTTPStatusCodes::to_int(&HTTPStatusCodes::c200),
            reason: HTTPStatusCodes::get_generic_reason(&HTTPStatusCodes::c200),
            headers: Header::new(),
            stream: None
        }
    }
}
