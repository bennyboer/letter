use std::collections::HashMap;

/// A document has a tree-like structure.
pub struct DocumentStructure {
    nodes: HashMap<u64, DocumentNode>,
    root_node: u64,
}

pub struct DocumentNode {
    id: u64,
    children: Vec<u64>,
    name: String,
    value: DocumentNodeValue,
}

/// Node values with special meanings in the context of a document.
pub enum DocumentNodeValue {
    DocumentRoot,
    Text,
    Section,
    Header,
    Paragraph,
    Image,
}
