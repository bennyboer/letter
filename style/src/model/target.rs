use super::{class::ClassName, node::NodeName};

pub struct StyleTarget {
    node_name: Option<NodeName>,
    classes: Vec<ClassName>,
}

impl StyleTarget {
    pub fn new(node_name: Option<NodeName>, classes: Vec<ClassName>) -> Self {
        Self { node_name, classes }
    }

    pub fn node_name(&self) -> Option<&NodeName> {
        self.node_name.as_ref()
    }

    pub fn classes(&self) -> &[ClassName] {
        &self.classes
    }
}
