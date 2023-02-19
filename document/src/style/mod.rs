use std::collections::HashMap;

use unit::Distance;

pub use crate::style::class::ClassName;
pub use crate::style::definition::StyleDefinition;
use crate::style::id::StyleId;
pub use crate::style::node::NodeName;
use crate::style::resolver::StyleResolver;

mod class;
mod definition;
mod id;
mod node;
mod resolver;

pub struct DocumentStyles {
    styles: HashMap<StyleId, StyleDefinition>,
    resolver: StyleResolver,
    style_id_counter: StyleId,
}

impl DocumentStyles {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            resolver: StyleResolver::new(),
            style_id_counter: 0,
        }
    }

    pub fn resolve(&self, node_name: &NodeName, class_name: Option<&ClassName>) -> Vec<&Style> {
        let style_ids = self.resolver.resolve(node_name, class_name);
        style_ids
            .iter()
            .filter_map(|style_id| self.styles.get(style_id))
            .flat_map(|style_definition| &style_definition.styles)
            .collect()
    }

    pub fn register_style_definition(
        &mut self,
        node_name: &NodeName,
        class_name: Option<&ClassName>,
        style_definition: StyleDefinition,
    ) {
        let id = self.style_id_counter;
        self.style_id_counter += 1;

        self.styles.insert(id, style_definition);
        self.resolver.register_style(node_name, class_name, id);
    }
}

#[derive(Clone, Debug)]
pub enum Style {
    Width(Distance),
    Height(Distance),
}
