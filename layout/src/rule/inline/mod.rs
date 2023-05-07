use document::structure::DocumentNode;
use document::Document;

mod item;
mod transformer;

use crate::context::LayoutContext;
use crate::result::LayoutResult;
use crate::rule::LayoutRule;

pub(crate) struct InlineLayoutRule;

impl InlineLayoutRule {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LayoutRule for InlineLayoutRule {
    fn layout(
        &self,
        node: &DocumentNode,
        document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        let items = transformer::to_box_glue_model(node, document, ctx)?;

        println!("Items: {:?}", items);

        // TODO Convert node content to box-glue-model and use Knuth-Plass algorithm to layout the paragraph
        // TODO Iterate over all children and add all text nodes as boxes - split words using hyphenator - save original nodes for each box to resolve styles later on
        // TODO Resolve styles of all boxes to determine width of the boxes (font, font-size, ...)
        Ok(())
    }
}
