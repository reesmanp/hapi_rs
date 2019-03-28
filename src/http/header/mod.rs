use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone)]
pub struct Header {
    values: HashMap<String, String>
}

impl Header {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.values.remove(&key);
    }

    pub fn get_header(&self, key: String) -> Option<&String> {
        self.values.get(&key)
    }

    pub fn get_headers_formatted(&self) -> String {
        let mut header_vec = vec![];

        for (key, value) in self.values.iter() {
            header_vec.push(format!("{}: {}", key, value));
        }

        header_vec.join("\n")
    }
}
