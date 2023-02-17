use document::Document;
use document::structure::DocumentNode;

use crate::context::LayoutContext;
use crate::result::LayoutResult;
use crate::rule::LayoutRule;

pub(crate) struct DefaultLayoutRule;

impl DefaultLayoutRule {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LayoutRule for DefaultLayoutRule {
    fn layout(
        &self,
        _node: &DocumentNode,
        _document: &Document,
        _ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        Ok(())
    }

    fn post_layout(
        &self,
        _node: &DocumentNode,
        _document: &Document,
        _ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        Ok(())
    }
}
