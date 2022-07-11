pub struct Author {
    pub name: String,
    pub mail: Option<String>,
}

impl Author {
    pub fn new(name: &str, mail: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            mail: mail.map(|s| s.to_string()),
        }
    }
}
