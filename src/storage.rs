use std::collections::HashMap;

pub struct Storage {
    data: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn find_key_by_value(
        &self,
        value: &str,
        ignored_key: Option<&str>,
    ) -> Option<String> {
        for (key, stored_value) in &self.data {
            if ignored_key == Some(key.as_str()) {
                continue;
            }

            if stored_value == value {
                return Some(key.clone());
            }
        }

        None
    }
}