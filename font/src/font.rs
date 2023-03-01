use std::path::Path;

use harfbuzz_rs::{Blob, Face, Font, Owned, Shared};

pub struct LetterFont<'a> {
    internal_font: Owned<Font<'a>>,
}

impl<'a> LetterFont<'a> {
    fn new<T: Into<Shared<Face<'a>>>>(font_face: T) -> Self {
        let font = Font::new(font_face);

        Self {
            internal_font: font,
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
}
