use typeset::glyph_shaping::GlyphDetails;

#[derive(Debug)]
pub struct TextSliceContent {
    pub glyphs: Vec<GlyphDetails>,
}
