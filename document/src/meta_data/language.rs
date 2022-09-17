use sys_locale::get_locale;

pub struct DocumentLanguage {
    /// ISO 639-1 language codes.
    /// For example "de", "en", ...
    pub language_code: String,

    /// Region code. For example "US", "DE", ...
    pub region_code: Option<String>,
}

impl DocumentLanguage {
    pub fn new(language_code: &str, region_code: Option<&str>) -> Self {
        Self {
            language_code: language_code.to_string(),
            region_code: region_code.map(|code| code.to_string()),
        }
    }
}

impl Default for DocumentLanguage {
    fn default() -> Self {
        let locale = get_locale().unwrap_or(String::from("en-US"));
        let parts: Vec<&str> = locale.split("-").collect();

        let (language_code, region_code) = if parts.len() > 1 {
            (parts[0], Some(parts[1]))
        } else {
            (parts[0], None)
        };

        DocumentLanguage::new(language_code, region_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let language = DocumentLanguage::default();
        assert!(
            !language.language_code.is_empty(),
            "Language code should have been set to the system default"
        );
    }
}
