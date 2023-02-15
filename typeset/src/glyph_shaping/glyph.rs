use unit::Distance;

#[derive(Debug)]
pub struct GlyphDetails {
    pub codepoint: u32,
    pub cluster: u32,

    /// Horizontal advance from the text shaping process.
    /// This includes distance adjustments like kerning.
    pub x_advance: Distance,

    /// X-advance directly from the used font.
    /// This will not take any distance adjustments into account like kerning.
    /// TODO Can we remove this field?
    pub font_x_advance: Distance,
}
