use std::collections::HashMap;

use font::{FontId, FontManager, LetterFont};

use super::{ElementId, LayoutElement, Page};

pub struct DocumentLayout<'a> {
    pages: Vec<Page>,
    element_lookup: HashMap<ElementId, LayoutElement>,
    pub font_manager: FontManager<'a>,
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

    pub fn get_font(&self, id: &FontId) -> Option<&LetterFont> {
        self.font_manager.get_font(id)
    }

    pub(crate) fn finalize(&mut self) {
        self.font_manager.subset_fonts();
    }
}
