extern crate core;

use document::structure::{DocumentNode, DocumentNodeValue};
use document::style::{NodeName, Style};
use document::Document;
use unit::{Distance, DistanceUnit};

use crate::context::{LayoutContext, LayoutStyle, OneSizeFitsAllPageSizing, PageSizing};
use crate::element::{DocumentLayout, LayoutConstraints, Size};
use crate::options::LayoutOptions;
use crate::result::LayoutResult;
use crate::rule::{LayoutRule, RootLayoutRule, TextLayoutRule};

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
            return Ok(layout_pass_result.layout);
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

fn layout_pass(
    document: &Document,
    last_pass_layout: Option<DocumentLayout>,
    _options: &LayoutOptions,
) -> LayoutResult<LayoutPassResult> {
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

    push_node_styles(node, document, ctx)?;
    {
        layout_node_using_rule(node, document, ctx)?;

        let node_ids = node.children();
        for node_id in node_ids {
            if let Some(node) = structure.get_node(*node_id) {
                process_node(node, document, ctx)?;
            }
        }
    }
    pop_node_styles(node, document, ctx)?;

    Ok(())
}

fn push_node_styles(
    node: &DocumentNode,
    document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    let node_name: Option<NodeName> = node.name().map(|name| name.into());
    if let Some(node_name) = node_name {
        let class_name = None; // TODO Get class name from node
        let styles = document.styles.resolve(&node_name, class_name);
        let layout_style = apply_to_layout_style(ctx.current_style(), &styles);

        ctx.push_style(layout_style);
    }

    Ok(())
}

fn pop_node_styles(
    node: &DocumentNode,
    _ctx: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    if node.name().is_some() {
        ctx.pop_style();
    }

    Ok(())
}

fn apply_to_layout_style(layout_style: &LayoutStyle, styles: &Vec<&Style>) -> LayoutStyle {
    let mut result = layout_style.clone();

    for style in styles {
        match style {
            Style::Width(distance) => {
                println!("Setting width to {:?}", distance);
                result.set_size(result.size().with_width(*distance))
            }
            Style::Height(distance) => result.set_size(result.size().with_height(*distance)),
        };
    }

    result
}

fn layout_node_using_rule(
    node: &DocumentNode,
    document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    let rule = map_node_to_rule(node);
    if let Some(rule) = rule {
        rule.layout(node, document, ctx)?;
    }

    Ok(())
}

fn map_node_to_rule(node: &DocumentNode) -> Option<Box<dyn LayoutRule>> {
    match node.value {
        DocumentNodeValue::DocumentRoot => Some(Box::new(RootLayoutRule::new())),
        DocumentNodeValue::Text(_) => Some(Box::new(TextLayoutRule::new())),
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

struct LayoutPassResult {
    stable: bool,
    layout: DocumentLayout,
}
