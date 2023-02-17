use std::collections::HashMap;

use document::structure::DocumentNode;
use document::Document;
use unit::{Distance, DistanceUnit};

use crate::context::LayoutContext;
use crate::element::{DocumentLayout, LayoutConstraints, Size};
use crate::options::LayoutOptions;
use crate::result::LayoutResult;

mod context;
pub mod element;
pub mod options;
pub mod result;

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
    init_ctx(&mut ctx, document);

    process_node(&document.structure.root(), document, &mut ctx);

    Ok(LayoutPassResult {
        stable: true,
        layout: DocumentLayout::new(vec![], HashMap::new()),
    })
}

fn process_node(node: &DocumentNode, document: &Document, ctx: &mut LayoutContext) {
    let structure = &document.structure;

    println!("Processing node: {:?}", node);

    // TODO Apply the nodes styles to the current context (e. g. push layout constraints, ...)
    // TODO Layout node using their assigned `LayoutRule` (if they have one). For example a break node may simply modify the layout constraints or push another page (if it is a page break)

    let node_ids = node.children();
    for node_id in node_ids {
        if let Some(node) = structure.get_node(*node_id) {
            process_node(node, document, ctx);
        }
    }
}

fn init_ctx(ctx: &mut LayoutContext, _document: &Document) {
    // TODO Read initial layout constraints from document styles
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
}

struct LayoutPassResult {
    stable: bool,
    layout: DocumentLayout,
}
