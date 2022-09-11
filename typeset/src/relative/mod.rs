//! This module is concerned with "relative" typesetting,
//! which is the second typesetting stage.
//! When performing relative typesetting we actually layout
//! each element relative to the previous element.
//! In this stage we do not concern ourselves with the concept of pages and
//! will just assume that we typeset each block on a infinitely high page.

mod text;

use crate::{
    element::TypesetElement,
    linearization::{Block, BlockValue},
    result::TypesetResult,
};

use self::text::typeset_text_block;

pub(crate) fn typeset_relatively(block: &Block) -> TypesetResult<TypesetElement> {
    match &block.value {
        BlockValue::Text(text_block) => Ok(typeset_text_block(&text_block)?),
        _ => Err(format!(
            "The block value '{:?}' is currently unsupported by relative typesetting",
            block.value
        ))?,
    }
}
