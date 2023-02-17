use unit::Distance;

use crate::glyph_shaping::GlyphDetails;

pub struct TextShaperResult {
    pub width: Distance,
    pub glyphs: Vec<GlyphDetails>,
}
