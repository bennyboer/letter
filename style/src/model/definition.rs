use super::{id::StyleId, style::Style};

pub struct StyleDefinition {
    pub id: StyleId,
    pub styles: Vec<Style>,
}
