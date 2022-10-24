use std::collections::HashMap;

use super::{
    definition::StyleDefinition, id::StyleId, resolver::StyleResolver, target::StyleTarget,
};

pub struct DocumentStyles {
    styles: HashMap<StyleId, StyleDefinition>,
    style_resolver: StyleResolver,
    style_id_counter: u64,
}

impl DocumentStyles {
    pub fn new() -> Self {
        let style_resolver = StyleResolver::new();

        Self {
            styles: HashMap::new(),
            style_resolver,
            style_id_counter: 1,
        }
    }

    pub fn derive_style(&self, style_ids: Vec<StyleId>) -> Option<StyleDefinition> {
        let style_definitions_to_merge: Vec<&StyleDefinition> = style_ids
            .iter()
            .map(|style_id| self.styles.get(style_id))
            .filter(|style_definition| style_definition.is_some())
            .map(|style_definition| style_definition.unwrap())
            .collect();

        if style_definitions_to_merge.is_empty() {
            return None;
        }

        Some(self.merge_style_definitions(&style_definitions_to_merge))
    }

    pub fn resolve_style(&self, target: StyleTarget) -> Option<StyleId> {
        self.style_resolver.resolve(target)
    }

    pub fn add_style_definition(
        &mut self,
        target: StyleTarget,
        style_definition: StyleDefinition,
    ) -> StyleId {
        let style_id = self.unused_style_id();
        self.styles.insert(style_id, style_definition);
        self.style_resolver.register_style_target(style_id, target);

        style_id
    }

    fn merge_style_definitions(&self, style_definitions: &[&StyleDefinition]) -> StyleDefinition {
        StyleDefinition { styles: vec![] } // TODO Merge multiple style definitions
    }

    fn unused_style_id(&mut self) -> StyleId {
        let result = self.style_id_counter;
        self.style_id_counter += 1;
        result
    }
}
