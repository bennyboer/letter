use crate::linearization::Block;
use document::structure::DocumentStructure;

pub(crate) mod linearization;

/// Linearization of a document structure is the process
/// of turning a document tree into a flat list of "blocks".
/// Blocks in turn are an abstract representation of a group
/// of elements that belong together in a document.
/// An example for a block would be a paragraph that contains of words/sentences/glyphs
/// or an image together with a caption, etc.
pub(crate) fn linearize(document_structure: &DocumentStructure) -> Vec<Block> {
    vec![]
}
