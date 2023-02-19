use document::structure::{DocumentNode, DocumentNodeValue};
use document::Document;

use crate::context::LayoutContext;
use crate::element::DocumentLayout;
use crate::options::LayoutOptions;
use crate::result::LayoutResult;
use crate::rule::{DefaultLayoutRule, LayoutRule, RootLayoutRule, TextLayoutRule};

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
    let mut ctx = LayoutContext::new(last_pass_layout);

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
    }
    pop_node_styles(node, document, ctx)?;

    let node_ids = node.children();
    for node_id in node_ids {
        if let Some(node) = structure.get_node(*node_id) {
            process_node(node, document, ctx)?;
        }
    }

    Ok(())
}

fn push_node_styles(
    node: &DocumentNode,
    document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    // TODO Remove constraints stack in context and replace by a field `style_stack: Vec<NodeStyles>`
    // TODO Push node styles to context (Push to `style_stack`)

    Ok(())
}

fn pop_node_styles(
    _node: &DocumentNode,
    _ctx: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    // TODO Pop node styles from context (Pop from `style_stack`)

    Ok(())
}

fn layout_node_using_rule(
    node: &DocumentNode,
    document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    let rule = map_node_to_rule(node);
    rule.layout(node, document, ctx)
}

fn map_node_to_rule(node: &DocumentNode) -> Box<dyn LayoutRule> {
    match node.value {
        DocumentNodeValue::DocumentRoot => Box::new(RootLayoutRule::new()),
        DocumentNodeValue::Text(_) => Box::new(TextLayoutRule::new()),
        _ => Box::new(DefaultLayoutRule::new()),
    }
}

struct LayoutPassResult {
    stable: bool,
    layout: DocumentLayout,
}
