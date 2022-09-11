use harfbuzz_rs::{shape, Face, Font, UnicodeBuffer};

use crate::{
    element::{
        Bounds, Position, Size, TextSliceContent, TypesetElement, TypesetElementContent,
        TypesetElementGroup,
    },
    linearization::TextBlock,
    linearization::TextBlockPartValue::Text,
    result::TypesetResult,
};

pub(crate) fn typeset_text_block(block: &TextBlock) -> TypesetResult<TypesetElement> {
    // Using the trivial line breaking algorithm to typeset the text (to be replaced by a better alternative)
    // TODO: Use the Knuth-Plass Algorithm to typeset the text block -> Convert to Box-Glue-Model first
    let line_width: f64 = 180.0; // TODO Get the line width per line separately, unit is probably millimeters, but should be configurable
    let font_size: f64 = 12.0; // TODO Make configurable
    let line_height: f64 = font_size * 0.5; // TODO Make configurable
    let white_space_width: f64 = calculate_text_width(" ")?; // TODO Can probably be removed when using the Knuth-Plass algorithm

    let mut offset = Position::zero();
    let mut elements = vec![];
    for part in &block.parts {
        if let Text(text_value) = &part.value {
            let text = &text_value.value;

            // TODO Preprocess text properly (split by white-space and use hyphenation based on currently set language)
            for text_part in text.split_whitespace() {
                // TODO Return complete shaper result and store in typeset element for text
                let width = (calculate_text_width(text_part)?) * font_size;

                let mut offset_after_text_part = offset.x() + width;

                let needs_whitespace_prefix = offset.x() != 0.0;
                if needs_whitespace_prefix {
                    offset_after_text_part += white_space_width;
                }

                let break_line = offset_after_text_part > line_width;
                if break_line {
                    offset = Position::absolute(0.0, offset.y() + line_height);
                    offset_after_text_part = offset.x() + width;
                }

                let bounds = Bounds {
                    position: offset,
                    size: Size {
                        width,
                        height: line_height,
                    },
                };
                let content = TypesetElementContent::TextSlice(TextSliceContent {
                    text: text_part.to_string(),
                });
                elements.push(TypesetElement::new(bounds, content));

                offset = Position::absolute(offset_after_text_part, offset.y());
            }
        }
    }

    let first_element_bounds = elements.first().map_or(Bounds::empty(), |e| *e.bounds());
    let last_element_bounds = elements.last().map_or(Bounds::empty(), |e| *e.bounds());
    let result_bounds = Bounds {
        position: first_element_bounds.position,
        size: Size::new(
            last_element_bounds.position.x() + last_element_bounds.size.width
                - first_element_bounds.position.x(),
            last_element_bounds.position.y() + last_element_bounds.size.height
                - first_element_bounds.position.y(),
        ),
    };
    let result_element = TypesetElement::new(
        result_bounds,
        TypesetElementContent::Group(TypesetElementGroup { elements }),
    );
    Ok(result_element)
}

fn calculate_text_width(text: &str) -> TypesetResult<f64> {
    // TODO This will parse the font each invovation which is expensive -> Refactor to only create font and buffer once

    let font_path = "C:/repo/kerning/fonts/Adobe/TisaPro/TisaPro.otf";
    let font_face_index = 0;
    let font_face = Face::from_file(font_path, font_face_index)?;
    let units_per_em = font_face.upem() as f64;
    let font = Font::new(font_face);

    let buffer = UnicodeBuffer::new().add_str(text);
    let output = shape(&font, buffer, &[]);

    let positions = output.get_glyph_positions();
    let infos = output.get_glyph_infos();

    let mut width: f64 = 0.0;
    for (position, _info) in positions.iter().zip(infos) {
        // println!("{:?} | {:?}", info, position);
        width += position.x_advance as f64;
    }

    Ok(width / units_per_em)
}
