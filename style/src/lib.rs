use color::Color;

mod color;
mod model;

pub fn parse(src: &str) -> DocumentStyles {
    DocumentStyles::new()
}

pub struct DocumentStyles {

}

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
    ClassName(String)
}

pub type StyleId = usize;

pub struct StyleDefinition {
    pub id: StyleId,
    pub styles: Vec<Style>
}

pub struct FontStyle {
    font_family: String,
    font_color: Box<dyn Color>
}

pub enum Style {
    Font(FontStyle),
    COLOR,
    PADDING,
    MARGIN
}

/// Defines how styles are inherited from other styles.
pub trait StyleInheritance {

    fn find_ancestor(style: StyleDefinition) -> Option<StyleId>;

}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {

    }

}