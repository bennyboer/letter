use super::TypesetElement;

/// A representation of a page of a typeset document
/// that contains a list of absolutely positioned elements.
#[derive(Debug)]
pub struct Page {
    number: usize,
    elements: Vec<TypesetElement>,
}

impl Page {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            elements: Vec::new(),
        }
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn add_element(&mut self, element: TypesetElement) -> &Self {
        self.elements.push(element);
        self
    }

    pub fn elements(&self) -> &[TypesetElement] {
        &self.elements
    }
}
