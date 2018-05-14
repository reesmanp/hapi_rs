use headers::Headers;
use cookies::Cookies;
use util::{parse_cookies, parse_headers};

pub struct Response {
    status_code: u32,
    status_code_text: String,
    cookies: Cookies,
    headers: Headers,
    response: String
}

impl Response {
  pub fn new() -> Response {
    Response {
      status_code: 200,
      status_code_text: "OK".to_string(),
      cookies: parse_cookies(String::new()),
      headers: parse_headers(String::new()),
      response: "".to_string()
    }
  }

  pub fn respond(&self) -> String {
    format!(
      "HTTP/1.1 {} {}\r\n\r\n{:?}\r\n{:?}\r\n{}",
      self.status_code,
      self.status_code_text,
      self.headers,
      self.cookies,
      self.response
    )
  }
}
