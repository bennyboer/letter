use document::Document;

use crate::result::LayoutResult;

pub mod result;

pub fn layout(_document: &Document) -> LayoutResult<()> {
    println!("Hello, world!");

    Ok(())
}
