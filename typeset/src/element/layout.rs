use std::collections::HashMap;

use super::{ElementId, Page, TypesetElement};

pub struct DocumentLayout {
    pages: Vec<Page>,
    element_lookup: HashMap<ElementId, TypesetElement>,
}

impl DocumentLayout {
    pub fn new(pages: Vec<Page>, element_lookup: HashMap<ElementId, TypesetElement>) -> Self {
        Self {
            pages,
            element_lookup,
        }
    }

    pub fn element(&self, id: &ElementId) -> Option<&TypesetElement> {
        self.element_lookup.get(&id)
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}
