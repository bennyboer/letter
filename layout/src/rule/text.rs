use document::structure::{DocumentNode, DocumentNodeValue};
use document::Document;
use typeset::glyph_shaping::shape_text;
use unit::{Distance, DistanceUnit};

use crate::context::LayoutContext;
use crate::element::content::{LayoutElementContent, TextSliceContent};
use crate::element::{Bounds, LayoutElement, Position, Size};
use crate::result::LayoutResult;
use crate::rule::LayoutRule;

pub(crate) struct TextLayoutRule;

impl TextLayoutRule {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LayoutRule for TextLayoutRule {
    fn layout(
        &self,
        node: &DocumentNode,
        document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        return if let DocumentNodeValue::Text(text) = &node.value {
            layout_text(text, node, document, ctx)
        } else {
            Err(format!("Expected text node, got: {:?}", node).into())
        };
    }

    fn post_layout(
        &self,
        _node: &DocumentNode,
        _document: &Document,
        _ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        Ok(())
    }
}

fn layout_text(
    text: &str,
    _node: &DocumentNode,
    _document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    let origin = ctx.offset();

    // Using the trivial line breaking algorithm to typeset the text (to be replaced by a better alternative)
    // TODO: Use the Knuth-Plass Algorithm to typeset the text block -> Convert to Box-Glue-Model first
    let line_width = Distance::new(170.0, DistanceUnit::Millimeter); // TODO Get the line width per line separately - configurable!
    let font_size = Distance::new(12.0, DistanceUnit::Points); // TODO Make configurable
    let line_height = font_size * 1.2; // TODO Make configurable
    let white_space_width: Distance = shape_text(" ", font_size)?.width; // TODO Can probably be removed when using the Knuth-Plass algorithm

    let mut y_offset = Distance::zero();
    let mut x_offset = Distance::zero();

    // TODO Preprocess text properly (split by white-space and use hyphenation based on currently set language)
    for text_part in text.split_whitespace() {
        // TODO Return complete shaper result and store in typeset element for text
        let shaped_text_part = shape_text(text_part, font_size)?;
        let text_part_width = shaped_text_part.width;

        let needs_whitespace_prefix = x_offset != Distance::zero();
        if needs_whitespace_prefix {
            x_offset += white_space_width;
        }

        let break_line = x_offset + text_part_width > line_width;
        if break_line {
            y_offset += line_height;
            x_offset = Distance::zero();
        }

        let element = {
            let position = Position::relative_to(&origin, x_offset, y_offset);
            let size = Size::new(text_part_width, line_height);
            let bounds = Bounds::new(position, size);

            let content = LayoutElementContent::TextSlice(TextSliceContent {
                glyphs: shaped_text_part.glyphs,
            });

            LayoutElement::new(bounds, content)
        };
        ctx.register_element(element);

        x_offset += text_part_width;
    }

    let paragraph_spacing = Distance::new(6.0, DistanceUnit::Points); // TODO Make configurable
    let new_origin = Position::relative_to(
        &origin,
        Distance::zero(),
        y_offset + line_height + paragraph_spacing,
    );
    ctx.set_offset(new_origin);

    Ok(())
}
