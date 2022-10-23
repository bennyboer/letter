use super::{class::ClassName, node::NodeName};

pub struct ResolveStyleCommand {
    pub node_name: Option<NodeName>,
    pub classes: Vec<ClassName>,
}
