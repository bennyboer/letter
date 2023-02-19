use crate::structure::{DocumentNodeValue, NodeId, SourcePosition};

#[derive(Debug)]
pub struct DocumentNode {
    pub id: NodeId,
    name: Option<String>,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub value: DocumentNodeValue,
    pub source_position: Option<SourcePosition>,
    // TODO attributes map (for example for the class attribute)
}

impl DocumentNode {
    pub fn new(
        id: NodeId,
        name: Option<String>,
        value: DocumentNodeValue,
        source_position: Option<SourcePosition>,
    ) -> Self {
        Self {
            id,
            name,
            parent: None,
            children: Vec::new(),
            value,
            source_position,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }
}
