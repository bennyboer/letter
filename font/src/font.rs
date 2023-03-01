use harfbuzz_rs::{Face, Font, Owned};

pub struct LetterFont<'a> {
    internal_font: Owned<Font<'a>>,
}

impl <'a> LetterFont<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let font_face = Face::from_bytes(bytes, 0);
        let font = Font::new(font_face);
        
        Self {
            internal_font: font,
        }
    }
    
    pub fn to_internal(&self) -> &Font<'a> {
        &self.internal_font
    }
}
