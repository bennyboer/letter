use harfbuzz_rs::{shape, Face, Font, UnicodeBuffer};

pub use glyph::GlyphDetails;
use unit::{Distance, DistanceUnit};

use crate::glyph_shaping::result::TextShaperResult;
use crate::result::TypesetResult;

mod glyph;
mod result;

// TODO Extract shape_text to some kind of shaper-service that can be mocked in tests
pub fn shape_text(text: &str, font_size: Distance) -> TypesetResult<TextShaperResult> {
    // TODO This will parse the font each invocation which is expensive -> Refactor to only create font and buffer once

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
