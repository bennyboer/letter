use document::style::FontFamilySource;
use unit::{Distance, DistanceUnit};

use crate::context::insets::Insets;
use crate::element::Size;

#[derive(Debug, Clone)]
pub(crate) struct LayoutStyle {
    size: Size,
    margin: Insets,
    padding: Insets,
    font_size: Distance,
    font_family: FontFamilySource,
}

impl LayoutStyle {
    pub fn new() -> Self {
        Self {
            size: Size::zero(),
            margin: Insets::zero(),
            padding: Insets::zero(),
            font_size: Distance::new(12.0, DistanceUnit::Points),
            font_family: FontFamilySource::Default,
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn margin(&self) -> &Insets {
        &self.margin
    }

    pub fn padding(&self) -> &Insets {
        &self.padding
    }

    pub fn font_size(&self) -> &Distance {
        &self.font_size
    }

    pub fn font_family(&self) -> &FontFamilySource {
        &self.font_family
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_margin(&mut self, margin: Insets) {
        self.margin = margin;
    }

    pub fn set_padding(&mut self, padding: Insets) {
        self.padding = padding;
    }

    pub fn set_font_size(&mut self, size: Distance) {
        self.font_size = size;
    }

    pub fn set_font_family(&mut self, family: FontFamilySource) {
        self.font_family = family;
    }
}
