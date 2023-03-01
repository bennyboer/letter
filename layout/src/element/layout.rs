use std::collections::HashMap;

use font::{FontManager, LetterFont};

use super::{ElementId, LayoutElement, Page};

pub struct DocumentLayout<'a> {
    pages: Vec<Page>,
    element_lookup: HashMap<ElementId, LayoutElement>,
    font_manager: FontManager<'a>,
}

impl<'a> DocumentLayout<'a> {
    pub fn new(
        pages: Vec<Page>,
        element_lookup: HashMap<ElementId, LayoutElement>,
        font_manager: FontManager<'a>,
    ) -> Self {
        Self {
            pages,
            element_lookup,
            font_manager,
        }
    }

    pub fn element(&self, id: &ElementId) -> Option<&LayoutElement> {
        self.element_lookup.get(&id)
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn get_default_font(&self) -> &LetterFont {
        self.font_manager.get_font(&self.font_manager.default_font_id()).unwrap()
    }
}
