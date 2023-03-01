use std::collections::HashMap;

pub use font::LetterFont;
pub use id::FontId;

use crate::id::FontIdGenerator;

mod font;
mod id;

const DEFAULT_FONT_ID: FontId = 0;
const DEFAULT_FONT_BYTES: &[u8] =
    include_bytes!("../../assets/fonts/lexend/Lexend-VariableFont_wght.ttf");

pub struct FontManager<'a> {
    font_id_generator: FontIdGenerator,
    registered_fonts: HashMap<FontId, LetterFont<'a>>,
}

impl<'a> FontManager<'a> {
    pub fn new() -> Self {
        let mut result = Self {
            font_id_generator: FontIdGenerator::new(),
            registered_fonts: HashMap::new(),
        };

        let default_font = LetterFont::from_bytes(DEFAULT_FONT_BYTES);
        result.register_font(default_font);

        result
    }

    pub fn register_font(&mut self, font: LetterFont<'a>) -> FontId {
        let font_id = self.font_id_generator.next();
        self.registered_fonts.insert(font_id.clone(), font);

        return font_id;
    }

    pub fn get_font(&self, font_id: &FontId) -> Option<&LetterFont> {
        self.registered_fonts.get(font_id)
    }

    pub fn default_font_id(&self) -> FontId {
        DEFAULT_FONT_ID
    }
}
