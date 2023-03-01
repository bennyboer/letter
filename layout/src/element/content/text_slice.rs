use font::FontId;
use typeset::glyph_shaping::GlyphDetails;
use unit::Distance;

#[derive(Debug)]
pub struct TextSliceContent {
    pub font: FontId,
    pub font_size: Distance,
    pub glyphs: Vec<GlyphDetails>,
}
