use std::cmp::min;

use document::structure::DocumentNode;
use document::style::{FontVariationSettings, TextAlignment};
use document::Document;
use font::{FontId, FontVariationId, LetterFont, LetterFontVariation};
use typeset::glyph_shaping::{shape_text, GlyphDetails};
use unit::{Distance, DistanceUnit, UnitValue};
use DistanceUnit::Millimeter;

use crate::context::{LayoutContext, LayoutStyle};
use crate::element::content::{LayoutElementContent, TextSliceContent};
use crate::element::{Bounds, LayoutElement, Position, Size};
use crate::result::LayoutResult;
use crate::rule::inline::line_breaker::{Line, LineItem, LineItemContentKind, Lines};
use crate::rule::LayoutRule;

mod item;
mod line_breaker;
mod transformer;

pub(crate) struct InlineLayoutRule;

impl InlineLayoutRule {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LayoutRule for InlineLayoutRule {
    fn layout(
        &self,
        node: &DocumentNode,
        document: &Document,
        ctx: &mut LayoutContext,
    ) -> LayoutResult<()> {
        let bounds = ctx.bounds();
        let style = ctx.current_style();
        let size = style.size();

        let line_width = if size.width < bounds.size().width {
            size.width
        } else {
            bounds.size().width
        };

        let items = transformer::to_box_glue_model(node, document, ctx)?;
        let lines = line_breaker::break_into_lines(items, line_width)?;

        layout_lines(lines, line_width, ctx)?;

        Ok(())
    }
}

fn layout_lines(lines: Lines, line_width: Distance, ctx: &mut LayoutContext) -> LayoutResult<()> {
    let mut bounds = ctx.bounds();
    let style = ctx.current_style().clone();

    let font_size = *style.font_size();
    let line_height = style.line_height();
    let line_height = font_size * line_height;

    if line_height > bounds.size().height {
        bounds = ctx.choose_next_bounds();
    }

    let mut position_ctx = PositionContext {
        line_height,
        bounds,
        x_offset: Distance::zero(),
        y_offset: Distance::zero(),
    };

    let line_count = lines.len();
    for (line_index, line) in lines.into_iter().enumerate() {
        let is_last_line = line_index == line_count - 1;

        let alignment = align_line(line_width, &line, is_last_line, &style);

        layout_line(line, &mut position_ctx, alignment, ctx)?;

        if !is_last_line {
            position_ctx.y_offset += line_height;

            let no_more_space_on_page = position_ctx.y_offset > position_ctx.bounds.size().height;
            if no_more_space_on_page {
                position_ctx.bounds = ctx.choose_next_bounds();
                position_ctx.y_offset = Distance::zero();
            }

            position_ctx.x_offset = Distance::zero();
        }
    }

    // Update context bounds
    {
        let total_height = position_ctx.y_offset + line_height;
        let new_bounds_position = Position::relative_to(
            &position_ctx.bounds.position(),
            Distance::zero(),
            total_height,
        );
        let new_height = position_ctx.bounds.size().height - total_height;
        let new_bounds = Bounds::new(
            new_bounds_position,
            position_ctx.bounds.size().with_height(new_height),
        );
        ctx.set_bounds(new_bounds);
    }

    Ok(())
}

fn layout_line(
    line: Line,
    position_ctx: &mut PositionContext,
    alignment: Alignment,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    let item_count_in_line = line.items.len();

    position_ctx.x_offset += alignment.indent;

    for (item_index, item) in line.items.into_iter().enumerate() {
        let is_last_item_on_line = item_index == item_count_in_line - 1;
        let flags = ItemLayoutFlags {
            is_last_item_on_line,
        };

        layout_item_on_line(item, flags, position_ctx, &alignment, ctx)?;
    }

    Ok(())
}

struct ItemLayoutFlags {
    is_last_item_on_line: bool,
}

fn layout_item_on_line(
    item: LineItem,
    flags: ItemLayoutFlags,
    position_ctx: &mut PositionContext,
    alignment: &Alignment,
    ctx: &mut LayoutContext,
) -> LayoutResult<()> {
    let mut elements = Vec::new();
    for part in item.parts {
        let style = part.style;
        let font_ctx = setup_font(style, ctx)?;
        let font = ctx.get_font_mut(&font_ctx.font_id);

        match part.kind {
            LineItemContentKind::Text(text) => {
                let result = shape_text(&text, font_ctx.font_size, font)?;

                mark_codepoints_as_used(font, &result.glyphs);

                let element = {
                    let position = Position::relative_to(
                        &position_ctx.bounds.position(),
                        position_ctx.x_offset,
                        position_ctx.y_offset,
                    );
                    let size = Size::new(result.width, position_ctx.line_height);
                    let bounds = Bounds::new(position, size);

                    let content = LayoutElementContent::TextSlice(TextSliceContent {
                        font: font_ctx.font_id,
                        font_variation: font_ctx._font_variation_id,
                        font_size: font_ctx.font_size,
                        glyphs: result.glyphs,
                    });

                    LayoutElement::new(bounds, content)
                };
                elements.push(element);

                position_ctx.x_offset += result.width;
            }
        }
    }

    for element in elements {
        ctx.register_element(element);
    }

    if !flags.is_last_item_on_line {
        position_ctx.x_offset += alignment.white_space_width;
    }

    Ok(())
}

struct PositionContext {
    line_height: Distance,
    bounds: Bounds,
    x_offset: Distance,
    y_offset: Distance,
}

struct Alignment {
    indent: Distance,
    white_space_width: Distance,
}

fn align_line(
    line_width: Distance,
    line: &Line,
    is_last_line: bool,
    style: &LayoutStyle,
) -> Alignment {
    let text_alignment = style.text_alignment();

    let white_space_count_in_line = line.white_spaces();
    let min_width = line.min_width();
    let stretchable_width = line_width - min_width;

    let preferred_white_space_width = line.white_space_width;
    let white_space_width = if white_space_count_in_line > 0 {
        Distance::new(
            stretchable_width.value(Millimeter) / white_space_count_in_line as UnitValue,
            Millimeter,
        )
    } else {
        Distance::zero()
    };
    let non_justified_white_space_width = if white_space_width < preferred_white_space_width {
        white_space_width
    } else {
        preferred_white_space_width
    };
    let justified_white_space_width = if is_last_line {
        non_justified_white_space_width
    } else {
        white_space_width
    };

    let indent = {
        if let TextAlignment::Justify | TextAlignment::Left = text_alignment {
            Distance::zero()
        } else {
            let remaining_space = stretchable_width
                - non_justified_white_space_width * white_space_count_in_line as UnitValue;

            match text_alignment {
                TextAlignment::Center => remaining_space / 2.0,
                TextAlignment::Right => remaining_space,
                _ => unreachable!(),
            }
        }
    };

    let final_white_space_width = if let TextAlignment::Justify = text_alignment {
        justified_white_space_width
    } else {
        non_justified_white_space_width
    };

    return Alignment {
        indent,
        white_space_width: final_white_space_width,
    };
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

fn setup_font(style: LayoutStyle, ctx: &mut LayoutContext) -> LayoutResult<FontContext> {
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
