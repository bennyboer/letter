use document::structure::DocumentNode;
use document::Document;
pub(crate) use root::RootLayoutRule;
pub(crate) use text::TextLayoutRule;

use crate::context::LayoutContext;
use crate::result::LayoutResult;

mod root;
mod text;

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
}
