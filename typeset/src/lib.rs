extern crate core;

use document::Document;
use element::DocumentLayout;

use crate::result::TypesetResult;

pub(crate) mod absolute;
pub(crate) mod context;
pub mod element;
pub(crate) mod linearization;
pub(crate) mod relative;
pub mod result;

#[deprecated]
pub fn typeset(document: &Document) -> TypesetResult<DocumentLayout> {
    let blocks = linearization::linearize(&document.structure)?;

    return absolute::typeset_absolutely(&blocks);
}
