use document::structure::DocumentNode;
use document::Document;

use crate::context::LayoutContext;
use crate::result::LayoutResult;
use crate::rule::LayoutRule;

pub(crate) struct RootLayoutRule;

impl RootLayoutRule {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LayoutRule for RootLayoutRule {
    fn layout(
        &self,
        _node: &DocumentNode,
        _document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        ctx.push_page();

        Ok(())
    }
}
