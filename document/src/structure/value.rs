/// Node values with special meanings in the context of a document.
pub enum DocumentNodeValue {
    DocumentRoot,
    Text(String),
    Section,
    Header(String),
    Paragraph,
    Image {
        source: String,
        width: Option<usize>,
        height: Option<usize>,
    },
}
