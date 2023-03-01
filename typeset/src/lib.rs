extern crate core;

pub mod glyph_shaping;
pub mod result;

#[cfg(test)]
mod tests {
    use harfbuzz_rs::{Face, Font};

    #[test]
    fn should_subset_font() {
        let font_path = "C:/repo/kerning/fonts/Adobe/TisaPro/TisaPro.otf";
        let font_face_index = 0;
        let font_face = Face::from_file(font_path, font_face_index).unwrap();
        let units_per_em = font_face.upem() as usize;
        let font = Font::new(font_face);

        let result = harfbuzz_rs::subset(&font);
        println!("{:?}", result);
    }
}
