use std::collections::HashMap;

#[derive(Default)]
pub struct DocumentVariables {
    lookup: HashMap<String, String>,
}

impl DocumentVariables {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: &str) {
        self.lookup.insert(name.into(), value.into());
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.lookup.get(name).map(|s| s.to_string())
    }
}
