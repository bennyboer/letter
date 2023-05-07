use std::collections::HashMap;

use crate::structure::{DocumentNodeValue, NodeId, SourcePosition};
use crate::style::ClassName;

const CLASS_ATTRIBUTE: &'static str = "class";

#[derive(Debug)]
pub struct DocumentNode {
    pub id: NodeId,
    name: Option<String>,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub value: DocumentNodeValue,
    pub source_position: Option<SourcePosition>,
    pub attributes: HashMap<String, String>,
}

impl DocumentNode {
    pub fn new(
        id: NodeId,
        name: Option<String>,
        value: DocumentNodeValue,
        attributes: HashMap<String, String>,
        source_position: Option<SourcePosition>,
    ) -> Self {
        Self {
            id,
            name,
            parent: None,
            children: Vec::new(),
            value,
            source_position,
            attributes,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn value(&self) -> &DocumentNodeValue {
        &self.value
    }

    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    pub fn class_name(&self) -> Option<&ClassName> {
        self.attributes.get(CLASS_ATTRIBUTE)
    }
}
