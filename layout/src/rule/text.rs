use document::structure::{DocumentNode, DocumentNodeValue};
use document::Document;
use typeset::glyph_shaping::shape_text;
use unit::Distance;

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
}

fn layout_text(
    text: &str,
    _node: &DocumentNode,
    _document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    let mut bounds = ctx.bounds();
    let style = ctx.current_style();
    let size = style.size();

    // Using the trivial line breaking algorithm to typeset the text (to be replaced by a better alternative)
    // TODO: Use the Knuth-Plass Algorithm to typeset the text block -> Convert to Box-Glue-Model first
    // TODO Get the line width per line separately - configurable!
    let line_width = if size.width < bounds.size().width {
        size.width
    } else {
        bounds.size().width
    };

    let font_size = *style.font_size();
    let font_family = style.font_family().clone();
    let font = ctx.find_font(&font_family).ok_or(format!(
        "Could not find font for font-family: {:?}",
        font_family
    ))?;
    let line_height = font_size * 1.2; // TODO Make configurable
    let white_space_width: Distance = shape_text(" ", font_size, ctx.get_font(&font))?.width; // TODO Can probably be removed when using the Knuth-Plass algorithm

    let mut y_offset = Distance::zero();
    let mut x_offset = Distance::zero();

    if line_height > bounds.size().height {
        bounds = ctx.choose_next_bounds();
    }

    // TODO Preprocess text properly (split by white-space and use hyphenation based on currently set language)
    for text_part in text.split_whitespace() {
        // TODO Return complete shaper result and store in typeset element for text
        let shaped_text_part = shape_text(text_part, font_size, ctx.get_font(&font))?;
        let text_part_width = shaped_text_part.width;

        let needs_whitespace_prefix = x_offset != Distance::zero();
        if needs_whitespace_prefix {
            x_offset += white_space_width;
        }

        let break_line = x_offset + text_part_width > line_width;
        if break_line {
            if y_offset + line_height > bounds.size().height {
                bounds = ctx.choose_next_bounds();
                y_offset = Distance::zero();
            } else {
                y_offset += line_height;
            }

            x_offset = Distance::zero();
        }

        let element = {
            let position = Position::relative_to(&bounds.position(), x_offset, y_offset);
            let size = Size::new(text_part_width, line_height);
            let bounds = Bounds::new(position, size);

            let content = LayoutElementContent::TextSlice(TextSliceContent {
                font_size,
                glyphs: shaped_text_part.glyphs,
            });

            LayoutElement::new(bounds, content)
        };
        ctx.register_element(element);

        x_offset += text_part_width;
    }

    let total_height = y_offset + line_height;
    let new_bounds_position =
        Position::relative_to(&bounds.position(), Distance::zero(), total_height);
    let new_height = bounds.size().height - total_height;
    let new_bounds = Bounds::new(new_bounds_position, bounds.size().with_height(new_height));
    ctx.set_bounds(new_bounds);

    Ok(())
}
