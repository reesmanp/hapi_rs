use super::{HTTPVersion, header::Header};

pub struct Response {
    version: HTTPVersion,
    code: u32,
    reason: String,
    headers: Header
}

impl Response {
    pub fn new(version: HTTPVersion, code: u32, reason: String, headers: Header) -> Self {
        Self {
            version,
            code,
            reason,
            headers
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

    pub fn flush(&mut self) {}
}

impl Default for Response {
    fn default() -> Self {
        Self {
            version: HTTPVersion::HTTP20,
            code: 200,
            reason: String::from("Ok"),
            headers: Header::new()
        }
    }
}
