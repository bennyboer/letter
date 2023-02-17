use document::structure::DocumentNode;
use document::Document;
use unit::{Distance, DistanceUnit};

use crate::context::LayoutContext;
use crate::element::{LayoutConstraints, Size};
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
        // TODO Read initial layout constraints from document (root) style
        let initial_layout_constraints = LayoutConstraints::new(
            Size::new(
                Distance::new(210.0, DistanceUnit::Millimeter),
                Distance::new(297.0, DistanceUnit::Millimeter),
            ),
            Distance::zero(),
            Distance::zero(),
            Distance::zero(),
            Distance::zero(),
        );

        ctx.push_layout_constraints(initial_layout_constraints);

        ctx.push_page();

        Ok(())
    }

    fn post_layout(
        &self,
        _node: &DocumentNode,
        _document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        ctx.pop_layout_constraints();

        Ok(())
    }
}
