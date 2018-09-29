use super::{HTTPMethod, HTTPVersion};
use super::header::Header;
use std::vec::Vec;
//use serde

pub struct Request {
    method: HTTPMethod,
    path: String,
    version: HTTPVersion,
    headers: Header,
    payload: String
}

impl Request {
    pub fn new(method: HTTPMethod, path: String, version: HTTPVersion, headers: Header, payload: String) -> Self {
        Self {
            method,
            path,
            version,
            headers,
            payload
        }
    }

    /**
     * Getters
    */

    pub fn get_method(&self) -> HTTPMethod {
        self.method
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_version(&self) -> HTTPVersion {
        self.version
    }

    pub fn get_headers(&self) -> Header {
        self.headers.clone()
    }

    pub fn get_payload(&self) -> String {
        self.payload.clone()
    }

    /**
     * Request Parsing
    */

    pub fn parse_request(buffer: &[u8]) -> Option<Self> {
        // Safe check for stringifying request
        let str_buffer = match String::from_utf8(buffer.to_vec()) {
            Err(_) => return None,
            Ok(T) => T
        };

        let mut split_buffer = str_buffer.split("\r\n");
        let mut header_vec: Vec<&str> = vec![];
        let mut payload_vec: Vec<&str> = vec![];

        let request_line_opt = Self::parse_request_line(split_buffer.next().unwrap());

        // Combine headers
        loop {
            match split_buffer.next() {
                None => return None,
                Some(line) => match line {
                    "" => break,
                    _ => header_vec.push(line)
                }
            }
        }

        let headers_opt = Self::parse_headers(header_vec.join("\n").as_ref());

        if request_line_opt == None || headers_opt == None {
            return None;
        }

        let (method, path, version) = request_line_opt.unwrap();
        let headers = headers_opt.unwrap();

        // Grab Payload
        loop {
            match split_buffer.next() {
                None => break,
                Some(T) => payload_vec.push(T)
            }
        }
        let payload = payload_vec.join("\r\n");

        Some(
            Self {
                method,
                path,
                version,
                headers,
                payload
            }
        )
    }

    fn parse_request_line(buffer: &str) -> Option<(HTTPMethod, String, HTTPVersion)> {
        let mut split_buffer = buffer.split(" ");

        // Safe check for HTTP method
        let mut next_iter = match split_buffer.next() {
            None => "",
            Some(T) => T
        };
        let method = HTTPMethod::from_str(next_iter);

        // Safe check for request-uri
        next_iter = match split_buffer.next() {
            None => "/",
            Some(T) => T
        };
        let path = String::from(next_iter);

        // Safe check for HTTP version
        next_iter = match split_buffer.next() {
            None => "",
            Some(T) => T
        };
        let version = HTTPVersion::from_str(next_iter);

        // Check if method or version is supported
        if method == HTTPMethod::ERR || version == HTTPVersion::ERR {
            return None;
        }

        Some((
            method,
            path,
            version
        ))
    }

    fn parse_headers(buffer: &str) -> Option<Header> {
        let mut split_buffer = buffer.split("\n");
        let mut header = Header::new();

        loop {
            let (key, value) = match split_buffer.next() {
                None => break,
                Some(line) => {
                    let mut split_line = line.split(": ");
                    let temp_key = split_line.next().unwrap();
                    let temp_value = split_line.next().unwrap();
                    (temp_key, temp_value)
                }
            };

            header.insert(String::from(key), String::from(value));
        }

        Some(header)
    }

    //fn parse_payload(mut buffer: &str) -> Option
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: HTTPMethod::GET,
            path: String::from(""),
            version: HTTPVersion::HTTP11,
            headers: Header::new(),
            payload: String::from("")
        }
    }
}
