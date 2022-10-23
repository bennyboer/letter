mod family;

pub use family::{FontFamily, FontFamilyName};

use crate::color::{Color, Colors};

#[derive(Debug)]
pub struct FontStyle {
    family: FontFamily,
    color: Box<dyn Color>,
}

impl FontStyle {
    pub fn new(family: FontFamily, color: Box<dyn Color>) -> Self {
        Self { family, color }
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            color: Box::new(Colors::BLACK),
            ..Default::default()
        }
    }
}
