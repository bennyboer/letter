use std::collections::{HashMap, HashSet};
use std::path::Path;

use harfbuzz_rs::{subset, Blob, Face, Font, Owned, Shared, Tag, Variation};

use crate::variation::{FontVariationId, FontVariationIdGenerator};
use crate::LetterFontVariation;

pub struct LetterFont<'a> {
    internal_font: Owned<Font<'a>>,
    used_codepoints: HashSet<u32>,
    variation_id_lookup: HashMap<Vec<LetterFontVariation>, FontVariationId>,
    variation_lookup: HashMap<FontVariationId, Vec<LetterFontVariation>>,
    variation_id_generator: FontVariationIdGenerator,
    subsetted_fonts: HashMap<FontVariationId, Owned<Font<'a>>>,
}

impl<'a> LetterFont<'a> {
    fn new<T: Into<Shared<Face<'a>>>>(font_face: T) -> Self {
        let font = Font::new(font_face);

        Self {
            internal_font: font,
            used_codepoints: HashSet::new(),
            variation_id_lookup: HashMap::new(),
            variation_lookup: HashMap::new(),
            variation_id_generator: FontVariationIdGenerator::new(),
            subsetted_fonts: HashMap::new(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P, index: u32) -> std::io::Result<LetterFont<'a>> {
        let font_face = Face::from_file(path, index)?;

        Ok(LetterFont::new(font_face))
    }

    pub fn from_bytes(bytes: &'a [u8], index: u32) -> Self {
        let font_face = Face::from_bytes(bytes, index);

        Self::new(font_face)
    }

    pub fn from_vec(vec: Vec<u8>, index: u32) -> Self {
        let blob = Blob::with_bytes_owned(vec, |t| t.as_ref());
        let font_face = Face::new(blob, index);

        Self::new(font_face)
    }

    pub fn to_internal(&self) -> &Font<'a> {
        &self.internal_font
    }

    pub fn mark_codepoint_as_used(&mut self, codepoint: u32) {
        self.used_codepoints.insert(codepoint);
    }

    pub fn set_variations(&mut self, variations: &[LetterFontVariation]) -> FontVariationId {
        let variations_key: Vec<LetterFontVariation> = variations.iter().cloned().collect();
        let variation_id = match self.variation_id_lookup.get(&variations_key) {
            None => {
                let variation_id = self.variation_id_generator.next();

                self.variation_id_lookup
                    .insert(variations_key.clone(), variation_id);
                self.variation_lookup.insert(variation_id, variations_key);

                variation_id
            }
            Some(variation_id) => *variation_id,
        };

        let internal_variations = Self::map_to_internal_variations(variations);
        self.internal_font.set_variations(&internal_variations);

        variation_id
    }

    pub fn get_subsetted_font_data(&self, variation_id: &FontVariationId) -> Option<Vec<u8>> {
        self.subsetted_fonts
            .get(variation_id)
            .map(|font| font.face().face_data().to_vec())
    }

    pub(crate) fn subset(&mut self) {
        let used_codepoints: Vec<u32> = self.used_codepoints.iter().copied().collect();

        for variation_id in self.variation_lookup.keys() {
            let variations = self.variation_lookup.get(variation_id).unwrap();
            let variations = Self::map_to_internal_variations(variations);
            self.internal_font.set_variations(&variations);

            let bytes = subset(&mut self.internal_font, &used_codepoints, &variations);

            let blob = Blob::with_bytes_owned(bytes, |t| t.as_ref());
            let font_face = Face::new(blob, 0);

            let font = Font::new(font_face);
            self.subsetted_fonts.insert(*variation_id, font);
        }
    }

    fn map_to_internal_variations(variations: &[LetterFontVariation]) -> Vec<Variation> {
        variations
            .iter()
            .map(|v| {
                let chars: Vec<char> = v.name().chars().collect();
                let tag = Tag::new(chars[0], chars[1], chars[2], chars[3]);

                Variation::new(tag, v.value() as f32)
            })
            .collect()
    }
}
