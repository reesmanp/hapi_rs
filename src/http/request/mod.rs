use super::{HTTPMethod, HTTPVersion};
use super::header::Header;
use std::vec::Vec;
use serde

pub struct Request {
    method: HTTPMethod,
    path: String,
    version: HTTPVersion,
    headers: Header
}

impl Request {
    pub fn new(method: HTTPMethod, path: String, version: HTTPVersion, headers: Header) -> Self {
        Self {
            method,
            path,
            version,
            headers
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

    /**
     * Request Parsing
    */

    pub fn parse_request(mut buffer: &[u8]) -> Option<Self> {
        let mut str_buffer = String::from_utf8(buffer.to_vec()).unwrap();
        let mut split_buffer = str_buffer.split("\r\n");
        let mut header_vec: Vec<&str> = vec![];

        let request_line_opt = Self::parse_request_line(split_buffer.next().unwrap());

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

        loop {
            let next = split_buffer.next();
            match next {
                None => break,
                Some(message) => continue //println!("{:?}", message)
            }
        }

        if request_line_opt == None || headers_opt == None {
            return None;
        }

        let (method, path, version) = request_line_opt.unwrap();
        let headers = headers_opt.unwrap();

        Some(
            Self {
                method,
                path,
                version,
                headers
            }
        )
    }

    fn parse_request_line(mut buffer: &str) -> Option<(HTTPMethod, String, HTTPVersion)> {
        let mut split_buffer = buffer.split(" ");

        let method = HTTPMethod::from_str(split_buffer.next().unwrap());
        let path = String::from(split_buffer.next().unwrap());
        let version = HTTPVersion::from_str(split_buffer.next().unwrap());

        if method == HTTPMethod::ERR || version == HTTPVersion::ERR {
            return None;
        }

        Some((
            method,
            path,
            version
        ))
    }

    fn parse_headers(mut buffer: &str) -> Option<Header> {
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
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: HTTPMethod::GET,
            path: String::from(""),
            version: HTTPVersion::HTTP11,
            headers: Header::new()
        }
    }
}
