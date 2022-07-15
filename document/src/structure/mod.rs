mod node;
mod value;

pub use node::DocumentNode;
pub use value::DocumentNodeValue;

use std::collections::HashMap;
use std::fmt::Write;

pub type NodeId = u64;

/// A document features a tree-like structure containing
/// headings, paragraphs, ... on different levels.
pub struct DocumentStructure {
    nodes: HashMap<NodeId, DocumentNode>,
    root_node: NodeId,
    node_id_counter: u64,
}

impl DocumentStructure {
    pub fn new() -> Self {
        let root_node_id = 0;
        let root_node = DocumentNode::new(root_node_id, DocumentNodeValue::DocumentRoot);

        let mut nodes = HashMap::new();
        nodes.insert(root_node_id, root_node);

        Self {
            nodes,
            root_node: root_node_id,
            node_id_counter: 1,
        }
    }

    pub fn root(&self) -> &DocumentNode {
        self.get_node(self.root_node)
            .expect("Root node must be present at all times")
    }

    pub fn get_node(&self, id: NodeId) -> Option<&DocumentNode> {
        self.nodes.get(&id)
    }

    pub fn insert(&mut self, parent: NodeId, node: DocumentNode) {
        if let Some(parent_node) = self.nodes.get_mut(&parent) {
            parent_node.add_child(node.id);
            self.nodes.insert(node.id, node);
        }
    }

    pub fn unused_node_id(&mut self) -> NodeId {
        let result = self.node_id_counter;
        self.node_id_counter += 1;
        result
    }

    pub fn fmt_pretty(&self) -> String {
        let mut result = String::new();

        let level = 0;
        let root_node = self.root();
        self.pretty_fmt_node(&mut result, root_node, level);

        result
    }

    fn pretty_fmt_node(&self, result: &mut String, node: &DocumentNode, level: usize) {
        let indent = " ".repeat(level * 2);
        write!(result, "{}[{}]\n", indent, node.value).unwrap();

        for child_node_id in &node.children {
            if let Some(child_node) = self.get_node(*child_node_id) {
                self.pretty_fmt_node(result, child_node, level + 1);
            }
        }
    }
}
