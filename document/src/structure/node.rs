use crate::structure::{DocumentNodeValue, NodeId};

#[derive(Debug)]
pub struct DocumentNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<u64>,
    pub value: DocumentNodeValue,
    // TODO attributes map (for example for the class attribute)
}

impl DocumentNode {
    pub fn new(id: NodeId, value: DocumentNodeValue) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
            value,
        }
    }

    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }
}
