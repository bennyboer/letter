use std::collections::HashMap;

use super::{
    commands::ResolveStyleCommand, definition::StyleDefinition, id::StyleId, node::NodeName,
};

pub(crate) struct StyleResolver {
    lookup_by_node_name: HashMap<NodeName, Vec<StyleId>>,
    lookup_by_class_name: HashMap<NodeName, Vec<StyleId>>,
}

impl StyleResolver {
    pub fn new() -> Self {
        Self {
            lookup_by_node_name: HashMap::new(),
            lookup_by_class_name: HashMap::new(),
        }
    }

    pub fn resolve(&self, _command: ResolveStyleCommand) -> Option<StyleDefinition> {
        // TODO Lookup by node name and class name (for each class name -> one lookup)
        // TODO Merge all found style definitions together
        // TODO If having a duplicate style (for example font-size) -> NodeName < ClassName (if multiple classes reference font-size -> just choose the first occurrence)

        None
    }
}
