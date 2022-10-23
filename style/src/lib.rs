use std::collections::HashMap;

use color::Color;

mod color;
mod model;

pub fn parse(_src: &str) -> DocumentStyles {
    DocumentStyles::new()
}

pub struct DocumentStyles {
    styles: HashMap<StyleId, StyleDefinition>,
    lookup_by_node_name: HashMap<NodeName, Vec<StyleId>>,
    lookup_by_class_name: HashMap<NodeName, Vec<StyleId>>,
}

impl DocumentStyles {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            lookup_by_node_name: HashMap::new(),
            lookup_by_class_name: HashMap::new(),
        }
    }

    pub fn resolve_style(_command: ResolveStyleCommand) -> Option<StyleDefinition> {
        // TODO Lookup by node name and class name (for each class name -> one lookup)
        // TODO Merge all found style definitions together
        // TODO If having a duplicate style (for example font-size) -> NodeName < ClassName (if multiple classes reference font-size -> just choose the first occurrence)
        None
    }
}

pub struct ResolveStyleCommand {
    pub node_name: Option<NodeName>,
    pub classes: Vec<ClassName>,
}

pub type ClassName = String;
pub type NodeName = String;

pub type StyleId = usize;

pub struct StyleDefinition {
    pub id: StyleId,
    pub styles: Vec<Style>,
}

pub struct FontStyle {
    pub font_family: FontFamily,
    pub font_color: Box<dyn Color>,
}

pub struct FontFamily {
    pub preferred: FontFamilyName,
    pub alternatives: Vec<FontFamilyName>,
}

pub type FontFamilyName = String;

pub enum Style {
    Font(FontStyle),
    Color(Box<dyn Color>),
    Padding,
    Margin,
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {}
}
