extern crate core;

use document::structure::{DocumentNode, DocumentNodeValue};
use document::style::Style;
use document::Document;
use unit::{Distance, DistanceUnit};
use DocumentNodeValue::{Heading, ListItem, Paragraph, Text};

use crate::context::{LayoutContext, OneSizeFitsAllPageSizing, PageSizing};
use crate::element::{DocumentLayout, LayoutConstraints, Size};
use crate::options::LayoutOptions;
use crate::result::LayoutResult;
use crate::rule::{InlineLayoutRule, LayoutRule};

mod context;
pub mod element;
pub mod options;
pub mod result;
mod rule;

pub fn layout(document: &Document, options: LayoutOptions) -> LayoutResult<DocumentLayout> {
    let mut pass_counter = 0;
    let mut last_pass_layout = None;
    loop {
        let layout_pass_result = layout_pass(document, last_pass_layout, &options)?;
        if layout_pass_result.stable {
            return Ok(finalize_layout(layout_pass_result.layout));
        }

        last_pass_layout = Some(layout_pass_result.layout);

        pass_counter += 1;
        let max_passes_exceeded = pass_counter > options.max_passes;
        if max_passes_exceeded {
            return Err(format!(
                "Max layout passes ({}) exceeded. \
            Either you encountered an infinite loop or your document is very complex. \
            Try increasing the layout pass limit.",
                options.max_passes
            )
            .into());
        }
    }
}

fn layout_pass<'a>(
    document: &Document,
    last_pass_layout: Option<DocumentLayout<'a>>,
    _options: &LayoutOptions,
) -> LayoutResult<LayoutPassResult<'a>> {
    let page_sizing = create_page_sizing_behavior(document);
    let mut ctx = LayoutContext::new(last_pass_layout, page_sizing);

    process_node(&document.structure.root(), document, &mut ctx)?;

    let stable = ctx.is_stable();
    let layout = ctx.to_layout();

    Ok(LayoutPassResult { stable, layout })
}

fn process_node(
    node: &DocumentNode,
    document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    let structure = &document.structure;

    ctx.push_node_styles(node, document)?;
    {
        let is_consumed = layout_node_using_rule(node, document, ctx)?;

        if !is_consumed {
            let node_ids = node.children();
            for node_id in node_ids {
                if let Some(node) = structure.get_node(*node_id) {
                    process_node(node, document, ctx)?;
                }
            }
        }
    }
    ctx.pop_node_styles(node)?;

    Ok(())
}

fn layout_node_using_rule(
    node: &DocumentNode,
    document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<bool> {
    let rule = map_node_to_rule(node);
    if let Some(rule) = rule {
        rule.layout(node, document, ctx)?;
        return Ok(true);
    }

    Ok(false)
}

fn map_node_to_rule(node: &DocumentNode) -> Option<Box<dyn LayoutRule>> {
    match node.value {
        Text(_) | Paragraph | Heading | ListItem => Some(Box::new(InlineLayoutRule::new())),
        _ => None,
    }
}

fn create_page_sizing_behavior(document: &Document) -> Box<dyn PageSizing> {
    // TODO Replace `get_root_layout_constraints` by really checking for page sizing rules in the style sheet
    // TODO for now we just check the size defined on the root (document) style
    let layout_constraints = get_root_layout_constraints(document);

    Box::new(OneSizeFitsAllPageSizing::new(layout_constraints))
}

fn get_root_layout_constraints(document: &Document) -> LayoutConstraints {
    let document_styles = &document.styles;
    let styles = document_styles.resolve(&"document".into(), None);
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

fn finalize_layout(mut layout: DocumentLayout) -> DocumentLayout {
    layout.finalize();

    layout
}

struct LayoutPassResult<'a> {
    stable: bool,
    layout: DocumentLayout<'a>,
}
