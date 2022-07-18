pub(crate) mod image;
pub(crate) mod list;
pub(crate) mod text;

use document::structure::NodeId;
use image::ImageBlock;
use list::ListBlock;
use text::TextBlock;

pub(crate) struct Block {
    pub(crate) document_structure_node: NodeId,
    pub(crate) value: BlockValue,
}

impl Block {
    pub fn new(document_structure_node: NodeId, value: BlockValue) -> Self {
        Self {
            document_structure_node,
            value,
        }
    }
}

pub(crate) enum BlockValue {
    Text(TextBlock),
    Image(ImageBlock),
    List(ListBlock),
}
