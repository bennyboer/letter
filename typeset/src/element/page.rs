use crate::element::TypesetElement;

/// A representation of a page of a typeset document
/// that contains a list of absolutely positioned elements.
pub struct Page {
    pub number: i32,
    pub elements: Vec<TypesetElement>,
}
