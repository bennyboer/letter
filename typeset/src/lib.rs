extern crate core;

use crate::result::TypesetResult;
use document::Document;
use element::DocumentLayout;

pub(crate) mod absolute;
pub(crate) mod context;
pub mod element;
pub(crate) mod linearization;
pub(crate) mod relative;
pub mod result;

pub fn typeset(document: &Document) -> TypesetResult<DocumentLayout> {
    let blocks = linearization::linearize(&document.structure)?;

    return absolute::typeset_absolutely(&blocks);
}
