use crate::element::LayoutConstraints;

use super::ElementId;

/// A representation of a page of a typeset document
/// that contains a list of absolutely positioned elements.
#[derive(Debug)]
pub struct Page {
    number: usize,
    constraints: LayoutConstraints,
    elements: Vec<ElementId>,
}

impl Page {
    pub fn new(number: usize, constraints: LayoutConstraints) -> Self {
        Self {
            number,
            constraints,
            elements: Vec::new(),
        }
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn add_element(&mut self, element_id: ElementId) -> &Self {
        self.elements.push(element_id);
        self
    }

    pub fn elements(&self) -> &[ElementId] {
        &self.elements
    }

    pub fn constraints(&self) -> &LayoutConstraints {
        &self.constraints
    }
}
