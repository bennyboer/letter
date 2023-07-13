#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub mail: Option<String>,
}

impl Author {
    pub fn new(name: String, mail: Option<String>) -> Self {
        Self { name, mail }
    }
}
