//! Module responsible for transforming a Letter document node and its children into a box-glue-model
//! that can be used by the Knuth-Plass line breaking algorithm.

use hypher::{hyphenate, Lang};

use document::structure::{DocumentNode, DocumentNodeValue, NodeId};
use document::style::FontVariationSettings;
use document::Document;
use font::{FontId, FontVariationId, LetterFont, LetterFontVariation};
use typeset::glyph_shaping::shape_text;
use unit::{Distance, DistanceUnit};
use DocumentNodeValue::{Bold, Italic, Text};

use crate::context::LayoutContext;
use crate::result::LayoutResult;
use crate::rule::inline::item::{BoxContent, BoxItem, GlueItem, Item, PenaltyItem};

const HYPHEN_PENALTY: i32 = 50;
const INFINITE_PENALTY: i32 = 10000;

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

    finalize_paragraph(&mut items);

    Ok(items)
}

fn finalize_paragraph(result: &mut Vec<Item>) {
    result.push(Item::Glue(GlueItem::new(
        Distance::zero(),
        Distance::new(INFINITE_PENALTY as f64, DistanceUnit::Meter),
        Distance::zero(),
    )));
    result.push(Item::Penalty(PenaltyItem::new(
        Distance::zero(),
        -INFINITE_PENALTY,
        true,
    )));
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

    split_text_into_parts_and_map_to_items(text, node.id, ctx, font_ctx, result)?;

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

fn glue_after(
    _node_id: NodeId,
    _ctx: &mut LayoutContext,
    font_ctx: &FontContext,
    _last_char: char,
    white_space_width: Distance,
) -> Item {
    let font_size = font_ctx.font_size;

    Item::Glue(GlueItem::new(
        white_space_width,
        font_size / 6.0,
        font_size / 9.0,
    ))
}

fn split_text_into_parts_and_map_to_items(
    text: &str,
    node_id: NodeId,
    ctx: &mut LayoutContext,
    font_ctx: FontContext,
    result: &mut Vec<Item>,
) -> LayoutResult<()> {
    let mut buf = String::new();
    let mut last_char = '*';
    let white_space_width = calculate_text_width(" ", &font_ctx, ctx)?;
    let minus_char_width = calculate_text_width("-", &font_ctx, ctx)?;
    let style = ctx.current_style().clone();

    for c in text.chars() {
        if "- \n".find(c).is_some() {
            if !buf.is_empty() {
                split_word_into_syllables_and_map_to_items(
                    &buf.trim(),
                    node_id,
                    ctx,
                    &font_ctx,
                    minus_char_width,
                    result,
                )?;
                buf.clear();
            }
        }

        match c {
            '\r' => {}
            ' ' | '\n' => {
                if let Some(item) = result.last() {
                    if let Item::Glue(_) = item {
                        continue;
                    }
                }

                if result.len() == 0 {
                    continue;
                }

                result.push(glue_after(
                    node_id,
                    ctx,
                    &font_ctx,
                    last_char,
                    white_space_width,
                ))
            }
            '-' => {
                result.push(Item::Box(BoxItem::new(
                    minus_char_width,
                    BoxContent::Text(c.to_string()),
                    node_id,
                    style.clone(),
                )));
                result.push(Item::Penalty(PenaltyItem::new(
                    Distance::zero(),
                    HYPHEN_PENALTY,
                    true,
                )));
            }
            _ => {
                buf.push(c);
            }
        }

        last_char = c;
    }

    if !buf.is_empty() {
        split_word_into_syllables_and_map_to_items(
            &buf.trim(),
            node_id,
            ctx,
            &font_ctx,
            minus_char_width,
            result,
        )?;
    }

    Ok(())
}

fn split_word_into_syllables_and_map_to_items(
    word: &str,
    node_id: NodeId,
    ctx: &mut LayoutContext,
    font_ctx: &FontContext,
    minus_char_width: Distance,
    result: &mut Vec<Item>,
) -> LayoutResult<()> {
    let style = ctx.current_style().clone();
    let lang = Lang::English;
    let syllables: Vec<&str> = hyphenate(word, lang).collect();
    let syllable_count = syllables.len();
    for (idx, syllable) in syllables.into_iter().enumerate() {
        let width = calculate_text_width(&syllable, font_ctx, ctx)?;
        result.push(Item::Box(BoxItem::new(
            width,
            BoxContent::Text(syllable.to_owned()),
            node_id,
            style.clone(),
        )));

        if idx < syllable_count - 1 {
            result.push(Item::Penalty(PenaltyItem::new(
                minus_char_width,
                HYPHEN_PENALTY,
                true,
            )));
        }
    }

    Ok(())
}

fn calculate_text_width(
    text: &str,
    font_ctx: &FontContext,
    ctx: &mut LayoutContext,
) -> LayoutResult<Distance> {
    let font = ctx.get_font_mut(&font_ctx.font_id);
    let result = shape_text(text, font_ctx.font_size, font)?;

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
