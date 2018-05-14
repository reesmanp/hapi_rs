use headers::Headers;
use cookies::Cookies;

pub fn parse_headers(_headers: String) -> Headers {
  let mut headers = Headers::new();
  headers.insert(("".to_string(), "".to_string()));
  headers
}

pub fn parse_cookies(_cookies: String) -> Cookies {
  let mut cookies = Cookies::new();
  cookies.insert(("".to_string(), "".to_string()));
  cookies
}
