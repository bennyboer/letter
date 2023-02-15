use std::collections::HashMap;

use super::{ElementId, LayoutElement, Page};

pub struct DocumentLayout {
    pages: Vec<Page>,
    element_lookup: HashMap<ElementId, LayoutElement>,
}

impl DocumentLayout {
    pub fn new(pages: Vec<Page>, element_lookup: HashMap<ElementId, LayoutElement>) -> Self {
        Self {
            pages,
            element_lookup,
        }
    }

    pub fn element(&self, id: &ElementId) -> Option<&LayoutElement> {
        self.element_lookup.get(&id)
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}
