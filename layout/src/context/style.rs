use document::style::{
    FontFamilySource, FontStretch, FontStyle, FontVariationSettings, FontWeight, TextAlignment,
};
use unit::{Distance, DistanceUnit};

use crate::context::insets::Insets;
use crate::element::Size;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LayoutStyle {
    size: Size,
    margin: Insets,
    padding: Insets,
    font_size: Distance,
    font_family: FontFamilySource,
    font_variation_settings: FontVariationSettings,
    font_weight: FontWeight,
    font_stretch: FontStretch,
    font_style: FontStyle,
    line_height: f64,
    text_alignment: TextAlignment,
    first_line_indent: Distance,
}

impl LayoutStyle {
    pub fn new() -> Self {
        Self {
            size: Size::zero(),
            margin: Insets::zero(),
            padding: Insets::zero(),
            font_size: Distance::new(12.0, DistanceUnit::Points),
            font_family: FontFamilySource::Default,
            font_variation_settings: FontVariationSettings { variations: vec![] },
            font_weight: 400.0,
            font_stretch: 1.0,
            font_style: FontStyle::Normal,
            line_height: 1.25,
            text_alignment: TextAlignment::Justify,
            first_line_indent: Distance::zero(),
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

    pub fn font_variation_settings(&self) -> FontVariationSettings {
        self.font_variation_settings.clone()
    }

    pub fn font_weight(&self) -> FontWeight {
        self.font_weight
    }

    pub fn font_stretch(&self) -> FontStretch {
        self.font_stretch
    }

    pub fn font_style(&self) -> FontStyle {
        self.font_style
    }

    pub fn line_height(&self) -> f64 {
        self.line_height
    }

    pub fn text_alignment(&self) -> TextAlignment {
        self.text_alignment
    }

    pub fn first_line_indent(&self) -> &Distance {
        &self.first_line_indent
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

    pub fn set_font_variation_settings(&mut self, settings: FontVariationSettings) {
        // Merge with existing settings
        let mut result = self.font_variation_settings.variations.clone();
        for variation in settings.variations {
            if let Some(existing_variation) = result.iter_mut().find(|v| v.name == variation.name) {
                existing_variation.value = variation.value;
            } else {
                result.push(variation);
            }
        }

        self.font_variation_settings = FontVariationSettings { variations: result };
    }

    pub fn set_font_weight(&mut self, weight: FontWeight) {
        self.font_weight = weight;
    }

    pub fn set_font_stretch(&mut self, stretch: FontStretch) {
        self.font_stretch = stretch;
    }

    pub fn set_font_style(&mut self, style: FontStyle) {
        self.font_style = style;
    }

    pub fn set_line_height(&mut self, line_height: f64) {
        self.line_height = line_height;
    }

    pub fn set_text_alignment(&mut self, text_alignment: TextAlignment) {
        self.text_alignment = text_alignment;
    }

    pub fn set_first_line_indent(&mut self, indent: Distance) {
        self.first_line_indent = indent;
    }
}
