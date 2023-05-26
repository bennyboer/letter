use std::collections::HashMap;

pub use font_family::FontFamilySource;
pub use font_variation_settings::{FontVariation, FontVariationSettings};
use unit::Distance;
use unit::DistanceUnit::{Centimeter, Millimeter, Points};

pub use crate::style::class::ClassName;
pub use crate::style::definition::StyleDefinition;
use crate::style::id::StyleId;
pub use crate::style::node::NodeName;
use crate::style::resolver::StyleResolver;

mod class;
mod definition;
mod font_family;
mod font_variation_settings;
mod id;
mod node;
mod resolver;

const ROOT_NODE_NAME: &'static str = "document";

pub struct DocumentStyles {
    styles: HashMap<StyleId, StyleDefinition>,
    resolver: StyleResolver,
    style_id_counter: StyleId,
}

impl DocumentStyles {
    pub fn new() -> Self {
        let mut result = Self {
            styles: HashMap::new(),
            resolver: StyleResolver::new(),
            style_id_counter: 0,
        };

        fill_default_styles(&mut result);

        result
    }

    pub fn root_style(&self) -> Vec<&Style> {
        self.resolve(&ROOT_NODE_NAME.into(), None)
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

fn fill_default_styles(styles: &mut DocumentStyles) {
    styles.register_style_definition(
        &ROOT_NODE_NAME.into(),
        None,
        StyleDefinition {
            styles: vec![
                Style::FontSize(Distance::new(12.0, Points)),
                Style::LineHeight(1.25),
                Style::MarginTop(Distance::new(2.0, Centimeter)),
                Style::MarginBottom(Distance::new(2.0, Centimeter)),
                Style::MarginLeft(Distance::new(2.0, Centimeter)),
                Style::MarginRight(Distance::new(2.0, Centimeter)),
                Style::Width(Distance::new(210.0, Millimeter)),
                Style::Height(Distance::new(297.0, Millimeter)),
            ],
        },
    );
}

#[derive(Clone, Debug)]
pub enum Style {
    Width(Distance),
    Height(Distance),

    MarginTop(Distance),
    MarginRight(Distance),
    MarginBottom(Distance),
    MarginLeft(Distance),

    PaddingTop(Distance),
    PaddingRight(Distance),
    PaddingBottom(Distance),
    PaddingLeft(Distance),

    FontSize(Distance),
    FontFamily(FontFamilySource),
    FontVariationSettings(FontVariationSettings),

    LineHeight(f64),
}
