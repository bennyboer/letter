use std::collections::{HashMap, HashSet};

use crate::style::class::ClassName;
use crate::style::id::StyleId;
use crate::style::node::NodeName;

#[derive(Debug, Hash, Eq, PartialEq)]
struct StyleKey {
    node_name: NodeName,
    class_name: Option<ClassName>,
}

pub struct StyleResolver {
    lookup: HashMap<StyleKey, Vec<StyleId>>,
}

impl StyleResolver {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn resolve(&self, node_name: &NodeName, class_name: Option<&ClassName>) -> Vec<StyleId> {
        let mut styles: HashSet<StyleId> = HashSet::new();

        self.resolve_styles_for_node_name(node_name, &mut styles);

        if let Some(class_name) = class_name {
            self.resolve_styles_for_node_and_class_name(node_name, class_name, &mut styles);
        }

        styles.into_iter().collect()
    }

    pub fn register_style(
        &mut self,
        node_name: &NodeName,
        class_name: Option<&ClassName>,
        style_id: StyleId,
    ) {
        let key = StyleKey {
            node_name: node_name.to_owned(),
            class_name: class_name.cloned(),
        };
        let styles = self.lookup.entry(key).or_insert_with(Vec::new);
        styles.push(style_id);
    }

    fn resolve_styles_for_node_name(&self, node_name: &NodeName, result: &mut HashSet<StyleId>) {
        let key = StyleKey {
            node_name: node_name.to_owned(),
            class_name: None,
        };
        if let Some(styles) = self.lookup.get(&key) {
            result.extend(styles);
        }
    }

    fn resolve_styles_for_node_and_class_name(
        &self,
        node_name: &NodeName,
        class_name: &ClassName,
        result: &mut HashSet<StyleId>,
    ) {
        let key = StyleKey {
            node_name: node_name.to_owned(),
            class_name: Some(class_name.to_owned()),
        };
        if let Some(styles) = self.lookup.get(&key) {
            result.extend(styles);
        }
    }
}
