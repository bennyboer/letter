use crate::structure::DocumentNodeValue;

pub struct DocumentNode {
    pub id: u64,
    pub children: Vec<u64>,
    pub value: DocumentNodeValue,
}

impl DocumentNode {
    pub fn new(id: u64, value: DocumentNodeValue) -> Self {
        Self {
            id,
            children: Vec::new(),
            value,
        }
    }
}
