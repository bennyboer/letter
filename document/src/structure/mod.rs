mod node;
mod value;

pub use node::DocumentNode;
pub use value::DocumentNodeValue;

use std::collections::HashMap;

type NodeId = u64;

/// A document features a tree-like structure containing
/// headings, paragraphs, ... on different levels.
pub struct DocumentStructure {
    nodes: HashMap<NodeId, DocumentNode>,
    root_node: NodeId,
}

impl DocumentStructure {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_node: 0,
        }
    }

    pub fn root(&self) -> &DocumentNode {
        self.get_node(self.root_node).unwrap()
    }

    pub fn get_node(&self, id: NodeId) -> Option<&DocumentNode> {
        self.nodes.get(&id)
    }
}
