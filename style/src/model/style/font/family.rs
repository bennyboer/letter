pub type FontFamilyName = String;

#[derive(Debug)]
pub enum FontFamily {
    SansSerif,
    Serif,
    Custom(FontFamilyName),
}

#[derive(Debug)]
pub struct FontFamilyStyle {
    families_ordered_by_preference: Vec<FontFamily>,
}

#[derive(Debug)]
pub enum FontFamilyCreateError {
    MustProvideAtLeastOneFontFamily,
}

impl FontFamilyStyle {
    pub fn new(
        families_ordered_by_preference: Vec<FontFamily>,
    ) -> Result<Self, FontFamilyCreateError> {
        if families_ordered_by_preference.is_empty() {
            return Err(FontFamilyCreateError::MustProvideAtLeastOneFontFamily);
        }

        Ok(Self {
            families_ordered_by_preference,
        })
    }

    pub fn families_ordered_by_preference(&self) -> &[FontFamily] {
        &self.families_ordered_by_preference[..]
    }
}

impl Default for FontFamilyStyle {
    fn default() -> Self {
        FontFamilyStyle::new(vec![FontFamily::Serif]).unwrap()
    }
}
