use std::collections::HashMap;

pub use font_family::FontFamilySource;
pub use font_variation_settings::{FontVariation, FontVariationSettings};
use unit::Distance;
use unit::DistanceUnit::{Centimeter, Millimeter, Points};

pub use crate::style::class::ClassName;
pub use crate::style::definition::StyleDefinition;
use crate::style::id::StyleId;
pub use crate::style::node::NodeName;
pub use crate::style::pseudo_class::PseudoClass;
use crate::style::resolver::StyleResolver;
pub use crate::style::resolver::StyleResolvingContext;
pub use crate::style::text_alignment::TextAlignment;

mod class;
mod definition;
mod font_family;
mod font_variation_settings;
mod id;
mod node;
mod pseudo_class;
mod resolver;
mod text_alignment;

const ROOT_NODE_NAME: &'static str = "document";
const HEADING_NODE_NAME: &'static str = "heading";

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
        self.resolve(
            &ROOT_NODE_NAME.into(),
            None,
            StyleResolvingContext::default(),
        )
    }

    pub fn resolve(
        &self,
        node_name: &NodeName,
        class_name: Option<&ClassName>,
        ctx: StyleResolvingContext,
    ) -> Vec<&Style> {
        let style_ids = self.resolver.resolve(node_name, class_name, ctx);
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
        pseudo_class: Option<PseudoClass>,
        style_definition: StyleDefinition,
    ) {
        let id = self.style_id_counter;
        self.style_id_counter += 1;

        self.styles.insert(id, style_definition);
        self.resolver
            .register_style(node_name, class_name, pseudo_class, id);
    }
}

fn fill_default_styles(styles: &mut DocumentStyles) {
    styles.register_style_definition(
        &ROOT_NODE_NAME.into(),
        None,
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

    fill_default_heading_styles(styles);
}

fn fill_default_heading_styles(styles: &mut DocumentStyles) {
    struct HeadingSettings {
        font_size: Distance,
        margin_top: Distance,
        margin_bottom: Distance,
    }
    let fallback_heading_settings = HeadingSettings {
        font_size: Distance::new(14.0, Points),
        margin_top: Distance::new(3.0, Millimeter),
        margin_bottom: Distance::new(2.0, Millimeter),
    };
    styles.register_style_definition(
        &HEADING_NODE_NAME.into(),
        None,
        None,
        StyleDefinition {
            styles: vec![
                Style::FontSize(fallback_heading_settings.font_size),
                Style::MarginTop(fallback_heading_settings.margin_top),
                Style::MarginBottom(fallback_heading_settings.margin_bottom),
            ],
        },
    );

    let heading_settings_per_level = [
        HeadingSettings {
            font_size: Distance::new(48.0, Points),
            margin_top: Distance::new(0.0, Millimeter),
            margin_bottom: Distance::new(10.0, Millimeter),
        },
        HeadingSettings {
            font_size: Distance::new(32.0, Points),
            margin_top: Distance::new(5.0, Millimeter),
            margin_bottom: Distance::new(5.0, Millimeter),
        },
        HeadingSettings {
            font_size: Distance::new(24.0, Points),
            margin_top: Distance::new(5.0, Millimeter),
            margin_bottom: Distance::new(4.0, Millimeter),
        },
        HeadingSettings {
            font_size: Distance::new(20.0, Points),
            margin_top: Distance::new(5.0, Millimeter),
            margin_bottom: Distance::new(4.0, Millimeter),
        },
        HeadingSettings {
            font_size: Distance::new(16.0, Points),
            margin_top: Distance::new(4.0, Millimeter),
            margin_bottom: Distance::new(3.0, Millimeter),
        },
    ];
    for (level, settings) in heading_settings_per_level.into_iter().enumerate() {
        styles.register_style_definition(
            &HEADING_NODE_NAME.into(),
            None,
            Some(PseudoClass::Level(level)),
            StyleDefinition {
                styles: vec![
                    Style::FontSize(settings.font_size),
                    Style::MarginTop(settings.margin_top),
                    Style::MarginBottom(settings.margin_bottom),
                ],
            },
        )
    }
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
    TextAlignment(TextAlignment),
    FirstLineIndent(Distance),
}
