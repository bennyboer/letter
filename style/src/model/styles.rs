use std::collections::HashMap;

use super::{commands::ResolveStyleCommand, definition::StyleDefinition, id::StyleId, resolver::StyleResolver};

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

    pub fn resolve_style(&self, command: ResolveStyleCommand) -> Option<StyleDefinition> {
        self.style_resolver.resolve(command)
    }
}
