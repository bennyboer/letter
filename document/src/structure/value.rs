use std::fmt;
use std::fmt::{Display, Formatter};

/// Node values with special meanings in the context of a document.
#[derive(Debug)]
pub enum DocumentNodeValue {
    DocumentRoot,
    Text(String),
    Section,
    Header,
    Paragraph,
    Image {
        source: String,
        width: Option<String>,
        height: Option<String>,
    },
    Custom(String),
}

impl Display for DocumentNodeValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
