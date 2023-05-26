use document::structure::DocumentNode;
use document::Document;
pub(crate) use inline::InlineLayoutRule;
pub(crate) use section::SectionLayoutRule;

use crate::context::LayoutContext;
use crate::result::LayoutResult;

mod inline;
mod section;

/// A rule for laying out a document structure node on a page.
/// The result are absolute positioned elements that can be added to a page.
/// The rule may modify the passed `LayoutContext` to influence the layout of the following nodes.
pub(crate) trait LayoutRule {
    fn layout(
        &self,
        node: &DocumentNode,
        document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()>;

    /// Called after the node and all of its children have been laid out.
    fn after_layout(
        &self,
        _node: &DocumentNode,
        _document: &Document,
        _ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        Ok(())
    }

    /// Whether this rule will take care of laying out this node and all of its children.
    /// Layout rules that match child elements will not be applied.
    fn is_consuming(&self) -> bool {
        true
    }
}
