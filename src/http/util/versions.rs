#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HTTPVersion {
    HTTP11,
    HTTP20,
    ERR
}

impl HTTPVersion {
    pub fn from_bytes(buffer: &[u8]) -> Self {
        match buffer[..] {
            [72, 84, 84, 80, 47, 49, 46, 49] => HTTPVersion::HTTP11,
            [72, 84, 84, 80, 47, 50, 46, 48] => HTTPVersion::HTTP20,
            _ => HTTPVersion::ERR
        }
    }

    pub fn from_str(buffer: &str) -> Self {
        match buffer {
            "HTTP/1.1" => HTTPVersion::HTTP11,
            "HTTP/2.0" => HTTPVersion::HTTP20,
            _ => HTTPVersion::ERR
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            HTTPVersion::HTTP11 => String::from("HTTP/1.1"),
            HTTPVersion::HTTP20 => String::from("HTTP/2.0"),
            HTTPVersion::ERR => String::from("")
        }
    }
}