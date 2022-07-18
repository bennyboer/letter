/// Linearization of a document structure is the process
/// of turning a document tree into a flat list of "blocks".
/// Blocks in turn are an abstract representation of a group
/// of elements that belong together in a document.
/// An example for a block would be a paragraph that contains of words/sentences/glyphs
/// or an image together with a caption, etc.
mod block;

use crate::TypesetResult;
pub(crate) use block::Block;
use document::structure::DocumentStructure;

pub(crate) fn linearize(_document_structure: &DocumentStructure) -> TypesetResult<Vec<Block>> {
    Ok(vec![]) // TODO
}
