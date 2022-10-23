use color::Color;

mod color;
mod model;

pub fn parse(_src: &str) -> DocumentStyles {
    DocumentStyles::new()
}

pub struct DocumentStyles {}

impl DocumentStyles {
    pub fn new() -> Self {
        Self {}
    }

    pub fn resolve_style(by_method: StyleResolveMethod) -> StyleDefinition {
        match by_method {
            StyleResolveMethod::NodeName(node_name) => todo!(),
            StyleResolveMethod::ClassName(class_name) => todo!(),
        }
    }
}

pub enum StyleResolveMethod {
    NodeName(String),
    ClassName(String),
}

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
    Color,
    Padding,
    Margin,
}

/// Defines how styles are inherited from other styles.
pub trait StyleInheritance {
    fn find_ancestor(style: StyleDefinition) -> Option<StyleId>;
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {}
}
