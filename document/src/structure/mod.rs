use std::collections::HashMap;
use std::fmt::Write;

pub use node::DocumentNode;
pub use source_position::SourcePosition;
pub use value::DocumentNodeValue;

mod node;
mod source_position;
mod value;

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
        let root_node = DocumentNode::new(root_node_id, DocumentNodeValue::DocumentRoot, None);

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

    /// Get a path from the root node to the given node
    /// with all ancestors included
    /// where the first item is the root node
    /// and the last item is the given node.
    pub fn get_path(&self, id: NodeId) -> Vec<&DocumentNode> {
        let mut result = Vec::new();

        self.fill_path(&mut result, id);

        result.reverse();
        result
    }

    fn fill_path<'a>(
        self: &'a DocumentStructure,
        ancestors: &mut Vec<&'a DocumentNode>,
        id: NodeId,
    ) {
        if let Some(node) = self.get_node(id) {
            ancestors.push(node);

            if let Some(parent_node) = node.parent {
                self.fill_path(ancestors, parent_node);
            }
        }
    }

    pub fn insert(&mut self, parent: NodeId, mut node: DocumentNode) {
        if let Some(parent_node) = self.nodes.get_mut(&parent) {
            node.parent = Some(parent);
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
