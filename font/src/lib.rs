use std::collections::HashMap;

use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::{Properties, Style};
use font_kit::source::SystemSource;

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
    name_to_id: HashMap<String, FontId>,
    path_to_id: HashMap<String, FontId>,
}

impl<'a> FontManager<'a> {
    pub fn new() -> Self {
        let mut result = Self {
            font_id_generator: FontIdGenerator::new(),
            registered_fonts: HashMap::new(),
            name_to_id: HashMap::new(),
            path_to_id: HashMap::new(),
        };

        let default_font = LetterFont::from_bytes(DEFAULT_FONT_BYTES, 0);
        result.register_font(default_font);
        result
            .name_to_id
            .insert("default".to_owned(), DEFAULT_FONT_ID);

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

    pub fn get_font_mut(&mut self, font_id: &FontId) -> Option<&mut LetterFont<'a>> {
        self.registered_fonts.get_mut(font_id)
    }

    pub fn default_font_id(&self) -> FontId {
        DEFAULT_FONT_ID
    }

    pub fn find_by_name(&mut self, name: &str) -> Option<FontId> {
        if let Some(font_id) = self.name_to_id.get(name) {
            return Some(font_id.clone());
        }

        let family_name = FamilyName::Title(name.to_owned());
        let family_names = vec![family_name];

        let mut properties = Properties::new();
        properties.style = Style::Normal;

        let font_id = SystemSource::new()
            .select_best_match(&family_names, &properties)
            .ok()
            .and_then(|font| match font {
                Handle::Path { path, font_index } => LetterFont::from_path(path, font_index)
                    .ok()
                    .map(|font| self.register_font(font)),
                Handle::Memory { bytes, font_index } => {
                    let vec = bytes.clone().to_vec();
                    let letter_font = LetterFont::from_vec(vec, font_index);
                    let font_id = self.register_font(letter_font);
                    Some(font_id)
                }
            });

        if let Some(font_id) = font_id {
            self.name_to_id.insert(name.to_owned(), font_id);
        }

        font_id
    }

    pub fn find_by_path(&mut self, path: &str) -> Option<FontId> {
        if let Some(font_id) = self.path_to_id.get(path) {
            return Some(font_id.clone());
        }

        let font_id = LetterFont::from_path(path, 0)
            .ok()
            .map(|font| self.register_font(font));

        if let Some(font_id) = font_id {
            self.path_to_id.insert(path.to_owned(), font_id);
        }

        font_id
    }

    pub fn subset_fonts(&mut self) {
        for font in self.registered_fonts.values_mut() {
            font.subset();
        }
    }
}
