use std::collections::HashMap;

use super::{
    class::ClassName, definition::StyleDefinition, id::StyleId, node::NodeName, target::StyleTarget,
};

pub(crate) struct StyleResolver {
    lookup_by_node_name: HashMap<NodeName, Vec<StyleId>>,
    lookup_by_class_name: HashMap<ClassName, Vec<StyleId>>,
}

impl StyleResolver {
    pub(crate) fn new() -> Self {
        Self {
            lookup_by_node_name: HashMap::new(),
            lookup_by_class_name: HashMap::new(),
        }
    }

    pub(crate) fn resolve(&self, _target: StyleTarget) -> Option<StyleDefinition> {
        // TODO Lookup by node name and class name (for each class name -> one lookup)
        // TODO Merge all found style definitions together
        // TODO If having a duplicate style (for example font-size) -> NodeName < ClassName (if multiple classes reference font-size -> just choose the first occurrence)

        None
    }

    pub(crate) fn register_style_target(&mut self, style_id: StyleId, target: StyleTarget) {
        if let Some(node_name) = target.node_name() {
            self.register_style_by_node_name(style_id, node_name.to_string());
        }

        self.register_style_by_classes(style_id, &target.classes());
    }

    fn register_style_by_node_name(&mut self, style_id: StyleId, node_name: NodeName) {
        self.lookup_by_node_name
            .entry(node_name)
            .or_insert_with(|| Vec::new())
            .push(style_id);
    }

    fn register_style_by_classes(&mut self, style_id: StyleId, classes: &[ClassName]) {
        for class_name in classes {
            self.lookup_by_class_name
                .entry(class_name.to_string())
                .or_insert_with(|| Vec::new())
                .push(style_id);
        }
    }
}
