use std::collections::HashMap;

use super::{
    definition::StyleDefinition, id::StyleId, resolver::StyleResolver, target::StyleTarget,
};

pub struct DocumentStyles {
    styles: HashMap<StyleId, StyleDefinition>,
    style_resolver: StyleResolver,
}

impl DocumentStyles {
    pub fn new() -> Self {
        let style_resolver = StyleResolver::new();

        Self {
            styles: HashMap::new(),
            style_resolver,
        }
    }

    pub fn resolve_style(&self, target: StyleTarget) -> Option<StyleDefinition> {
        self.style_resolver.resolve(target)
    }

    pub fn add_style_definition(&mut self, target: StyleTarget, style_definition: StyleDefinition) {
        let style_id = style_definition.id;
        self.styles.insert(style_id, style_definition);
        self.style_resolver.register_style_target(style_id, target);
    }
}
