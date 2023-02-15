use harfbuzz_rs::{shape, Face, Font, UnicodeBuffer};
use unit::{Distance, DistanceUnit};

use crate::{
    context::TypesettingContext,
    element::{
        Bounds, ElementId, GlyphDetails, Position, Size, TextSliceContent, TypesetElement,
        TypesetElementContent, TypesetElementGroup,
    },
    linearization::TextBlock,
    linearization::TextBlockPartValue::Text,
    result::TypesetResult,
};

/// Typeset the given text block relative to the given anchor element.
#[deprecated]
pub(crate) fn typeset_text_block(
    text: &str,
    anchor: Position,
    ctx: &mut TypesettingContext,
) -> TypesetResult<TypesetElement> {
    // Using the trivial line breaking algorithm to typeset the text (to be replaced by a better alternative)
    // TODO: Use the Knuth-Plass Algorithm to typeset the text block -> Convert to Box-Glue-Model first
    let line_width = Distance::new(170.0, DistanceUnit::Millimeter); // TODO Get the line width per line separately - configurable!
    let font_size = Distance::new(12.0, DistanceUnit::Points); // TODO Make configurable
    let line_height = font_size * 1.2; // TODO Make configurable
    let white_space_width: Distance = shape_text(" ", font_size)?.width; // TODO Can probably be removed when using the Knuth-Plass algorithm

    let element_id = ElementId::new();
    let mut offset = Position::relative_to(element_id, Distance::zero(), Distance::zero());
    let mut elements = vec![];
    let mut last_element = element_id;
    let mut line_x_advance = Distance::zero();
    let mut max_size = Size::new(Distance::zero(), line_height);
    for part in &block.parts {
        if let Text(text_value) = &part.value {
            let text = &text_value.value;

            // TODO Preprocess text properly (split by white-space and use hyphenation based on currently set language)
            for text_part in text.split_whitespace() {
                // TODO Return complete shaper result and store in typeset element for text
                let shaped_text_part = shape_text(text_part, font_size)?;
                let text_part_width = shaped_text_part.width;

                let needs_whitespace_prefix = line_x_advance != Distance::zero();
                if needs_whitespace_prefix {
                    line_x_advance += white_space_width;
                    offset = Position::relative_to(
                        last_element,
                        offset.x() + white_space_width,
                        offset.y(),
                    );
                }

                let break_line = line_x_advance + text_part_width > line_width;
                if break_line {
                    offset = Position::relative_to(
                        last_element,
                        offset.x() - line_x_advance,
                        offset.y() + line_height,
                    );
                    line_x_advance = Distance::zero();

                    max_size.height += line_height;
                }

                let bounds = Bounds::new(offset, Size::new(text_part_width, line_height));
                let content = TypesetElementContent::TextSlice(TextSliceContent {
                    glyphs: shaped_text_part.glyphs,
                    text: text_part.to_string(),
                });
                let element = TypesetElement::new(bounds, content);
                last_element = element.id();
                elements.push(element.id());
                ctx.register_element(element); // TODO Refactor be be a TypesetElementSink -> sink.process(element);

                line_x_advance += text_part_width;
                offset = Position::relative_to(last_element, text_part_width, Distance::zero());

                if line_x_advance > max_size.width {
                    max_size.width = line_x_advance;
                }
            }
        }
    }

    let result_bounds = Bounds::new(anchor, max_size);
    let result_element = TypesetElement::of(
        element_id,
        result_bounds,
        TypesetElementContent::Group(TypesetElementGroup { elements }),
    );
    Ok(result_element)
}
