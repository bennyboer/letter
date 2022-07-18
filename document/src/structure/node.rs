use crate::structure::source_span::SourceSpan;
use crate::structure::{DocumentNodeValue, NodeId};

#[derive(Debug)]
pub struct DocumentNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub value: DocumentNodeValue,
    pub source_span: Option<SourceSpan>,
    // TODO attributes map (for example for the class attribute)
}

impl DocumentNode {
    pub fn new(id: NodeId, value: DocumentNodeValue, source_span: Option<SourceSpan>) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
            value,
            source_span,
        }
    }

    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }
}
