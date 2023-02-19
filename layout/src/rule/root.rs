use document::structure::DocumentNode;
use document::style::Style;
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

const ROOT_STYLE_NODE_NAME: &'static str = "document";

impl LayoutRule for RootLayoutRule {
    fn layout(
        &self,
        _node: &DocumentNode,
        document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        let document_styles = &document.styles;
        let styles = document_styles.resolve(&ROOT_STYLE_NODE_NAME.into(), None);

        let initial_layout_constraints = to_layout_constraints(&styles);
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

fn to_layout_constraints(styles: &Vec<&Style>) -> LayoutConstraints {
    let mut width = Distance::new(210.0, DistanceUnit::Millimeter);
    let mut height = Distance::new(297.0, DistanceUnit::Millimeter);

    let margin_top = Distance::zero();
    let margin_right = Distance::zero();
    let margin_bottom = Distance::zero();
    let margin_left = Distance::zero();

    for style in styles {
        match style {
            Style::Width(distance) => width = *distance,
            Style::Height(distance) => height = *distance,
            _ => {}
        }
    }

    let size = Size::new(width, height);
    LayoutConstraints::new(size, margin_top, margin_right, margin_bottom, margin_left)
}
