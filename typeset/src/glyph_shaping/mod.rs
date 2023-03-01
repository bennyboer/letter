use harfbuzz_rs::{shape, UnicodeBuffer};

use font::LetterFont;
pub use glyph::GlyphDetails;
use unit::{Distance, DistanceUnit};

use crate::glyph_shaping::result::TextShaperResult;
use crate::result::TypesetResult;

mod glyph;
mod result;

// TODO Extract shape_text to some kind of shaper-service that can be mocked in tests
pub fn shape_text(
    text: &str,
    font_size: Distance,
    font: &LetterFont,
) -> TypesetResult<TextShaperResult> {
    // TODO This will parse the font each invocation which is expensive -> Refactor to only create font and buffer once
    let internal_font = font.to_internal();
    let units_per_em = internal_font.face().upem() as usize;

    let buffer = UnicodeBuffer::new().add_str(text);
    let output = shape(&internal_font, buffer, &[]);

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
            internal_font.get_glyph_h_advance(codepoint) as f64,
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
            codepoint,
            cluster: info.cluster,
            x_advance,
            font_x_advance,
        };

        width += x_advance;
        glyphs.push(glyph_details);
    }

    Ok(TextShaperResult { width, glyphs })
}
