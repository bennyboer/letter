use std::collections::HashMap;

use document::Document;

use crate::element::DocumentLayout;
use crate::result::LayoutResult;

pub mod element;
pub mod result;

pub fn layout(_document: &Document) -> LayoutResult<DocumentLayout> {
    // TODO

    Ok(DocumentLayout::new(vec![], HashMap::new()))
}
