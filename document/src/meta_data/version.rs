#[derive(Debug)]
pub struct DocumentVersion {
    pub value: String,
}

impl DocumentVersion {
    pub fn new(value: &str) -> Self {
        if value.is_empty() {
            panic!("Document version value cannot be empty");
        }

        Self {
            value: value.to_string(),
        }
    }
}

impl Default for DocumentVersion {
    fn default() -> Self {
        DocumentVersion::new("0.0.1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let version = DocumentVersion::default();
        assert_eq!(version.value, "0.0.1", "Default version should be 0.0.1");
    }
}
