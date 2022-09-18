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
pub(crate) fn typeset_text_block(
    block: &TextBlock,
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

struct TextShaperResult {
    width: Distance,
    glyphs: Vec<GlyphDetails>,
}

// TODO Extract shape_text to some kind of shaper-service that can be mocked in tests
fn shape_text(text: &str, font_size: Distance) -> TypesetResult<TextShaperResult> {
    // TODO This will parse the font each invovation which is expensive -> Refactor to only create font and buffer once

    let font_path = "C:/repo/kerning/fonts/Adobe/TisaPro/TisaPro.otf";
    let font_face_index = 0;
    let font_face = Face::from_file(font_path, font_face_index)?;
    let units_per_em = font_face.upem() as usize;
    let font = Font::new(font_face);

    let buffer = UnicodeBuffer::new().add_str(text);
    let output = shape(&font, buffer, &[]);

    let positions = output.get_glyph_positions();
    let infos = output.get_glyph_infos();

    let mut width = Distance::new(
        0.0,
        DistanceUnit::FontUnits {
            units_per_em,
            font_size: font_size.value(DistanceUnit::Millimeter),
        },
    );
    let mut glyphs = Vec::new();
    for (position, info) in positions.iter().zip(infos) {
        let codepoint = info.codepoint;
        let font_x_advance = Distance::new(
            font.get_glyph_h_advance(codepoint) as f64,
            DistanceUnit::FontUnits {
                units_per_em,
                font_size: font_size.value(DistanceUnit::Millimeter),
            },
        );
        let x_advance = Distance::new(
            position.x_advance as f64,
            DistanceUnit::FontUnits {
                units_per_em,
                font_size: font_size.value(DistanceUnit::Millimeter),
            },
        );
        let glyph_details = GlyphDetails {
            codepoint: codepoint,
            cluster: info.cluster,
            x_advance,
            font_x_advance,
        };

        width += x_advance;
        glyphs.push(glyph_details);
    }

    Ok(TextShaperResult { width, glyphs })
}
