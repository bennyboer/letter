use crate::structure::{DocumentNodeValue, NodeId, SourcePosition};

#[derive(Debug)]
pub struct DocumentNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub value: DocumentNodeValue,
    pub source_position: Option<SourcePosition>,
    // TODO attributes map (for example for the class attribute)
}

impl DocumentNode {
    pub fn new(
        id: NodeId,
        value: DocumentNodeValue,
        source_position: Option<SourcePosition>,
    ) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
            value,
            source_position,
        }
    }

    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }
}
