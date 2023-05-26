use std::collections::HashMap;

use crate::style::class::ClassName;
use crate::style::id::StyleId;
use crate::style::node::NodeName;
use crate::style::PseudoClass;

#[derive(Debug, Hash, Eq, PartialEq)]
struct StyleKey {
    node_name: NodeName,
    class_name: Option<ClassName>,
    pseudo_class: Option<PseudoClass>,
}

pub struct StyleResolver {
    lookup: HashMap<StyleKey, Vec<StyleId>>,
}

#[derive(Default)]
pub struct StyleResolvingContext {
    pub level: usize,
}

impl StyleResolvingContext {
    fn pseudo_classes(&self) -> Vec<PseudoClass> {
        let mut result = Vec::new();

        result.push(PseudoClass::Level(self.level));

        result
    }
}

impl StyleResolver {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn resolve(
        &self,
        node_name: &NodeName,
        class_name: Option<&ClassName>,
        ctx: StyleResolvingContext,
    ) -> Vec<StyleId> {
        let mut styles: Vec<StyleId> = Vec::new();

        self.resolve_styles_for_node_name(node_name, &mut styles);

        if let Some(class_name) = class_name {
            self.resolve_styles_for_node_and_class_name(node_name, class_name, &mut styles);
        }

        self.resolve_styles_for_node_and_class_name_and_context(
            node_name,
            class_name,
            ctx,
            &mut styles,
        );

        styles.into_iter().collect()
    }

    pub fn register_style(
        &mut self,
        node_name: &NodeName,
        class_name: Option<&ClassName>,
        pseudo_class: Option<PseudoClass>,
        style_id: StyleId,
    ) {
        let key = StyleKey {
            node_name: node_name.to_owned(),
            class_name: class_name.cloned(),
            pseudo_class,
        };
        let styles = self.lookup.entry(key).or_insert_with(Vec::new);
        styles.push(style_id);
    }

    fn resolve_styles_for_node_name(&self, node_name: &NodeName, result: &mut Vec<StyleId>) {
        let key = StyleKey {
            node_name: node_name.to_owned(),
            class_name: None,
            pseudo_class: None,
        };

        if let Some(styles) = self.lookup.get(&key) {
            result.extend(styles);
        }
    }

    fn resolve_styles_for_node_and_class_name(
        &self,
        node_name: &NodeName,
        class_name: &ClassName,
        result: &mut Vec<StyleId>,
    ) {
        let key = StyleKey {
            node_name: node_name.to_owned(),
            class_name: Some(class_name.to_owned()),
            pseudo_class: None,
        };
        if let Some(styles) = self.lookup.get(&key) {
            result.extend(styles);
        }
    }

    fn resolve_styles_for_node_and_class_name_and_context(
        &self,
        node_name: &NodeName,
        class_name: Option<&ClassName>,
        ctx: StyleResolvingContext,
        result: &mut Vec<StyleId>,
    ) {
        let pseudo_classes = ctx.pseudo_classes();

        for pseudo_class in pseudo_classes {
            let key = StyleKey {
                node_name: node_name.to_owned(),
                class_name: class_name.cloned(),
                pseudo_class: Some(pseudo_class),
            };
            if let Some(styles) = self.lookup.get(&key) {
                result.extend(styles);
            }
        }
    }
}
