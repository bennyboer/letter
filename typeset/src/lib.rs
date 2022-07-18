extern crate core;

use crate::element::Page;
use crate::result::TypesetResult;
use document::Document;

pub mod element;
pub(crate) mod linearization;
pub mod result;

pub fn typeset(document: &Document) -> TypesetResult<Vec<Page>> {
    let _blocks = linearization::linearize(&document.structure)?;

    Ok(vec![]) // TODO
}
