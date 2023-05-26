use document::structure::DocumentNode;
use document::Document;

use crate::context::LayoutContext;
use crate::result::LayoutResult;
use crate::rule::LayoutRule;

pub(crate) struct SectionLayoutRule;

impl SectionLayoutRule {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LayoutRule for SectionLayoutRule {
    fn layout(
        &self,
        _node: &DocumentNode,
        _document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        ctx.push_section();

        Ok(())
    }

    fn after_layout(
        &self,
        _node: &DocumentNode,
        _document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        ctx.pop_section();

        Ok(())
    }

    fn is_consuming(&self) -> bool {
        false
    }
}
