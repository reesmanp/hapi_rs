use headers::Headers;
use cookies::Cookies;
use util::{parse_cookies, parse_headers};

pub struct Request {
    pub method: String,
    pub path: String,
    pub cookies: Cookies,
    pub headers: Headers
}

impl Request {
  pub fn new(
    method: String,
    path: String,
    headers: String,
    cookies: String
    ) -> Request {
    Request {
      method: method,
      path: path,
      cookies: parse_cookies(cookies),
      headers: parse_headers(headers)
    }
  }
}
