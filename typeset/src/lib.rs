extern crate core;

use crate::element::Page;
use crate::result::TypesetResult;
use document::Document;

pub mod element;
pub(crate) mod linearization;
pub(crate) mod relative;
pub mod result;

pub fn typeset(document: &Document) -> TypesetResult<Vec<Page>> {
    let blocks = linearization::linearize(&document.structure)?;

    println!("### BLOCKS ###");
    println!("{:#?}", blocks);

    let groups = relative::typeset_relatively(&blocks)?;

    println!("### GROUPS ###");
    println!("{:#?}", groups);

    Ok(vec![]) // TODO
}
