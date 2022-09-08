//! This module is concerned with "relative" typesetting,
//! which is the second typesetting stage.
//! When performing relative typesetting we actually layout
//! each element relative to the previous element.
//! In this stage we do not concern ourselves with the concept of pages and
//! will just assume that we typeset each block on a infinitely high page.

mod group;
mod text;

use crate::{
    linearization::{Block, BlockValue},
    result::TypesetResult,
};
pub(crate) use group::TypesetGroup;

use self::text::typeset_text_block;

pub(crate) fn typeset_relatively(blocks: &Vec<Block>) -> TypesetResult<Vec<TypesetGroup>> {
    // TODO Because the typesetting is relative we can perform this step highly concurrent.
    // TODO The blocks may be typeset by a number of worker threads

    let mut result = Vec::new();

    for block in blocks {
        process_block(block, &mut result)?;
    }

    Ok(result)
}

fn process_block(block: &Block, groups: &mut Vec<TypesetGroup>) -> TypesetResult<()> {
    match &block.value {
        BlockValue::Text(text_block) => {
            let group = typeset_text_block(&text_block)?;
            groups.push(group);
            Ok(())
        }
        BlockValue::Image(_) => Ok(()),
        BlockValue::List(_) => Ok(()),
    }
}
