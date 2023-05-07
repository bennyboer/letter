//! Module responsible for transforming a Letter document node and its children into a box-glue-model
//! that can be used by the Knuth-Plass line breaking algorithm.

use document::structure::{DocumentNode, DocumentNodeValue};
use document::style::FontVariationSettings;
use document::Document;
use font::{FontId, FontVariationId, LetterFont, LetterFontVariation};
use typeset::glyph_shaping::{shape_text, GlyphDetails};
use unit::Distance;
use DocumentNodeValue::{Bold, Italic, Text};

use crate::context::LayoutContext;
use crate::result::LayoutResult;
use crate::rule::inline::item::{BoxContent, BoxItem, Item};

pub(crate) fn to_box_glue_model(
    node: &DocumentNode,
    document: &Document,
    ctx: &mut LayoutContext,
) -> LayoutResult<Vec<Item>> {
    let mut items = Vec::new();

    for child in node.children() {
        if let Some(child_node) = document.structure.get_node(*child) {
            process_node(child_node, document, ctx, &mut items)?;
        }
    }

    Ok(items)
}

fn process_node(
    node: &DocumentNode,
    document: &Document,
    ctx: &mut LayoutContext,
    result: &mut Vec<Item>,
) -> LayoutResult<()> {
    ctx.push_node_styles(node, document)?;
    {
        let is_consumed = map_node_to_item(node, ctx, result)?;
        if !is_consumed {
            for child in node.children() {
                if let Some(child_node) = document.structure.get_node(*child) {
                    process_node(child_node, document, ctx, result)?;
                }
            }
        }
    }
    ctx.pop_node_styles(node)?;

    Ok(())
}

fn map_node_to_item(
    node: &DocumentNode,
    ctx: &mut LayoutContext,
    result: &mut Vec<Item>,
) -> LayoutResult<bool> {
    match node.value() {
        Text(content) => map_text_node_to_item(content, node, ctx, result),
        Bold | Italic => Ok(false),
        // TODO Image, math, link, etc.
        _ => {
            return Err(format!(
                "Node '{}' is not a supported inline node",
                node.name().unwrap_or("unknown")
            )
            .into())
        }
    }
}

fn map_text_node_to_item(
    text: &str,
    node: &DocumentNode,
    ctx: &mut LayoutContext,
    result: &mut Vec<Item>,
) -> LayoutResult<bool> {
    let font_ctx = setup_font(ctx)?;

    let parts = split_text_into_parts(text);

    for part in parts {
        let width = calculate_text_width(&part, &font_ctx, ctx)?;
        let text_content = BoxContent::Text(part);
        let item = Item::Box(BoxItem::new(width, text_content, node.id));
        result.push(item);
    }

    Ok(true)
}

fn setup_font(ctx: &mut LayoutContext) -> LayoutResult<FontContext> {
    let style = ctx.current_style();
    let font_size = *style.font_size();
    let font_family = style.font_family().clone();
    let font_variation_settings = style.font_variation_settings().clone();
    let font_id = ctx.find_font(&font_family).ok_or(format!(
        "Could not find font for font-family: {:?}",
        font_family
    ))?;
    let font = ctx.get_font_mut(&font_id);
    let font_variation_id = initialize_font_variations(font, &font_variation_settings);

    Ok(FontContext::new(font_id, font_variation_id, font_size))
}

fn split_text_into_parts(text: &str) -> Vec<String> {
    // TODO Support hyphenation
    text.split(' ').map(|part| part.to_string()).collect()
}

fn calculate_text_width(
    text: &str,
    font_ctx: &FontContext,
    ctx: &mut LayoutContext,
) -> LayoutResult<Distance> {
    let font = ctx.get_font_mut(&font_ctx.font_id);
    let result = shape_text(text, font_ctx.font_size, font)?;

    mark_codepoints_as_used(font, &result.glyphs); // TODO This might be better suited in the real layout step. We are in a transformer here.

    Ok(result.width)
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

fn mark_codepoints_as_used(font: &mut LetterFont, glyphs: &Vec<GlyphDetails>) {
    for glyph in glyphs {
        let codepoint = glyph.codepoint;
        font.mark_codepoint_as_used(codepoint);
    }
}

struct FontContext {
    font_id: FontId,
    _font_variation_id: FontVariationId,
    font_size: Distance,
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
