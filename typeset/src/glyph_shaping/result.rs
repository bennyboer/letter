use unit::Distance;

use crate::glyph_shaping::GlyphDetails;

pub(crate) struct _TextShaperResult {
    pub(crate) width: Distance,
    pub(crate) glyphs: Vec<GlyphDetails>,
}
