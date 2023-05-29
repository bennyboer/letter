use document::style::FontVariationSettings;
use font::{FontId, FontVariationId, LetterFont, LetterFontVariation};
use unit::Distance;

use crate::context::{LayoutContext, LayoutStyle};
use crate::result::LayoutResult;

pub(crate) struct FontContext {
    pub(crate) font_id: FontId,
    pub(crate) _font_variation_id: FontVariationId,
    pub(crate) font_size: Distance,
}

impl FontContext {
    pub fn new(font_id: FontId, font_variation_id: FontVariationId, font_size: Distance) -> Self {
        Self {
            font_id,
            _font_variation_id: font_variation_id,
            font_size,
        }
    }
}

pub(crate) fn setup_font(
    ctx: &mut LayoutContext,
    style: Option<&LayoutStyle>,
) -> LayoutResult<FontContext> {
    let style = style.unwrap_or_else(|| ctx.current_style()).clone();
    let font_size = *style.font_size();
    let font_family = style.font_family().clone();
    let font_variation_settings = style.font_variation_settings();
    let font_id = ctx.find_font(&font_family, Some(&style)).ok_or(format!(
        "Could not find font for font-family: {:?}",
        font_family
    ))?;
    let font = ctx.get_font_mut(&font_id);
    let font_variation_id = initialize_font_variations(font, &font_variation_settings);

    Ok(FontContext::new(font_id, font_variation_id, font_size))
}

fn initialize_font_variations(
    font: &mut LetterFont,
    font_variation_settings: &FontVariationSettings,
) -> FontVariationId {
    let variations: Vec<LetterFontVariation> = font_variation_settings
        .variations
        .iter()
        .map(|v| LetterFontVariation::new(v.name.to_owned(), v.value))
        .collect();
    return font.set_variations(&variations);
}
