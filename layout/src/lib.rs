use std::collections::HashMap;

use document::Document;

use crate::context::LayoutContext;
use crate::element::DocumentLayout;
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
        let layout_pass_result = layout_pass(document, &last_pass_layout, &options)?;
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
    last_pass_layout: &Option<DocumentLayout>,
    options: &LayoutOptions,
) -> LayoutResult<LayoutPassResult> {
    let ctx = LayoutContext::new();

    // TODO Initialize first page in context with page size and margins from styles

    Ok(LayoutPassResult {
        stable: true,
        layout: DocumentLayout::new(vec![], HashMap::new()),
    })
}

struct LayoutPassResult {
    stable: bool,
    layout: DocumentLayout,
}
