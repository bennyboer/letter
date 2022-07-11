pub struct DocumentEncoding {
    name: String,
}

impl DocumentEncoding {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(self) -> String {
        self.name
    }
}

impl Default for DocumentEncoding {
    fn default() -> Self {
        DocumentEncoding::new("utf-8")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let encoding = DocumentEncoding::default();
        assert_eq!(encoding.name, "utf-8", "Default encoding should be UTF-8");
    }
}
