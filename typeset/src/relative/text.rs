use harfbuzz_rs::{shape, Face, Font, UnicodeBuffer};

use crate::{
    element::{Bounds, Position, Size, TextSliceContent, TypesetElement, TypesetElementContent},
    linearization::TextBlock,
    linearization::TextBlockPartValue::Text,
    result::TypesetResult,
};

use super::TypesetGroup;

pub(crate) fn typeset_text_block(block: &TextBlock) -> TypesetResult<TypesetGroup> {
    // Using the trivial line breaking algorithm to typeset the text (to be replaced by a better alternative)
    // TODO: Use the Knuth-Plass Algorithm to typeset the text block -> Convert to Box-Glue-Model first
    let line_width: f64 = 500.0; // TODO Get the line width per line separately, unit is probably millimeters, but should be configurable
    let font_size: f64 = 12.0; // TODO Make configurable
    let line_height: f64 = font_size * 1.5; // TODO Make configurable
    let white_space_width: f64 = calculate_text_width(" ")?; // TODO Can probably be removed when using the Knuth-Plass algorithm

    let mut offset = Position { x: 0.0, y: 0.0 };
    let mut elements = vec![];
    for part in &block.parts {
        if let Text(text_value) = &part.value {
            let text = &text_value.value;

            // TODO Preprocess text properly (split by white-space and use hyphenation based on currently set language)
            for text_part in text.split_whitespace() {
                // TODO Return complete shaper result and store in typeset element for text
                let width = (calculate_text_width(text_part)?) * font_size;

                let mut offset_after_text_part = offset.x + width;

                let needs_whitespace_prefix = offset.x != 0.0;
                if needs_whitespace_prefix {
                    offset_after_text_part += white_space_width;
                }

                let break_line = offset_after_text_part > line_width;
                if break_line {
                    offset.x = 0.0;
                    offset.y += line_height;
                    offset_after_text_part = offset.x + width;
                }

                elements.push(TypesetElement {
                    bounds: Bounds {
                        position: offset,
                        size: Size {
                            width,
                            height: line_height,
                        },
                    },
                    content: TypesetElementContent::TextSlice(TextSliceContent {
                        text: text_part.to_string(),
                    }),
                });

                offset.x = offset_after_text_part;
            }
        }
    }

    Ok(TypesetGroup { elements })
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
