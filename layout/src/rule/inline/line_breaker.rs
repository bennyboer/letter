use paragraph_breaker::Breakpoint;

use document::structure::NodeId;
use unit::{Distance, DistanceUnit};

use crate::context::LayoutStyle;
use crate::result::LayoutResult;
use crate::rule::inline::item::{BoxContent, Item};

#[derive(Debug)]
pub(crate) struct LineItem {
    pub(crate) parts: Vec<LineItemContent>,
}

impl LineItem {
    pub(crate) fn width(&self) -> Distance {
        self.parts.iter().map(|p| p.width).sum()
    }
}

#[derive(Debug)]
pub(crate) struct LineItemContent {
    pub(crate) kind: LineItemContentKind,
    pub(crate) width: Distance,
    pub(crate) node: NodeId,
    pub(crate) style: LayoutStyle,
}

#[derive(Debug)]
pub(crate) enum LineItemContentKind {
    Text(String),
}

pub(crate) type Line = Vec<LineItem>;
pub(crate) type Lines = Vec<Line>;

pub(crate) trait LineUtils {
    fn white_spaces(&self) -> usize;
    fn min_width(&self) -> Distance;
}

impl LineUtils for Line {
    fn white_spaces(&self) -> usize {
        self.len() - 1
    }

    fn min_width(&self) -> Distance {
        let mut width = Distance::zero();

        for item in self {
            width += item.width();
        }

        width
    }
}

pub(crate) fn break_into_lines(items: Vec<Item>, line_width: Distance) -> LayoutResult<Lines> {
    let break_points = find_break_points(&items, line_width)?;

    let mut lines = vec![Vec::new()];
    let mut break_point_idx = 0;
    let mut next_break_point = break_points.first();
    for (idx, item) in items.into_iter().enumerate() {
        let current_line = lines.last_mut().unwrap();
        current_line.push(item);

        if let Some(break_point) = next_break_point {
            if idx == break_point.index {
                lines.push(Vec::new());
                break_point_idx += 1;
                next_break_point = break_points.get(break_point_idx);
            }
        }
    }

    let mut result = Vec::new();
    for line in lines {
        let mut line_items = Vec::new();
        let item_count = line.len();
        let mut last_line_item_without_glue_between = None;
        for (item_index, item) in line.into_iter().enumerate() {
            let is_last_item = item_index == item_count - 1;

            match item {
                Item::Box(box_item) => {
                    let content = {
                        let content = match box_item.content() {
                            BoxContent::Text(text) => LineItemContentKind::Text(text.to_owned()),
                        };

                        LineItemContent {
                            kind: content,
                            width: box_item.width(),
                            node: box_item.node(),
                            style: box_item.style().clone(),
                        }
                    };

                    if let Some(last_box_item_index) = last_line_item_without_glue_between {
                        let last_item: &mut LineItem = &mut line_items[last_box_item_index];
                        last_item.parts.push(content);
                    } else {
                        let line_item = LineItem {
                            parts: vec![content],
                        };
                        line_items.push(line_item);
                        last_line_item_without_glue_between = Some(line_items.len() - 1);
                    };
                }
                Item::Penalty(item) => {
                    if is_last_item && item.width() > Distance::zero() {
                        let last_item = line_items.last_mut().unwrap();
                        let last_part = last_item.parts.last().unwrap();
                        last_item.parts.push(LineItemContent {
                            kind: LineItemContentKind::Text("-".to_owned()),
                            width: item.width(),
                            node: last_part.node,
                            style: last_part.style.clone(),
                        });
                    }
                }
                _ => last_line_item_without_glue_between = None,
            }
        }

        if !line_items.is_empty() {
            result.push(line_items);
        }
    }

    Ok(result)
}

fn find_break_points(items: &Vec<Item>, line_width: Distance) -> LayoutResult<Vec<Breakpoint>> {
    let internal_items = items
        .iter()
        .map(|item| match item {
            Item::Box(box_item) => paragraph_breaker::Item::Box {
                width: to_line_breaking_width(box_item.width()),
                data: (),
            },
            Item::Glue(glue_item) => paragraph_breaker::Item::Glue {
                width: to_line_breaking_width(glue_item.width()),
                stretch: to_line_breaking_width(glue_item.stretch()),
                shrink: to_line_breaking_width(glue_item.shrink()),
            },
            Item::Penalty(penalty_item) => paragraph_breaker::Item::Penalty {
                width: to_line_breaking_width(penalty_item.width()),
                penalty: penalty_item.penalty(),
                flagged: penalty_item.flagged(),
            },
        })
        .collect::<Vec<_>>();

    let break_points = paragraph_breaker::total_fit(
        &internal_items,
        &[to_line_breaking_width(line_width)],
        1.0,
        0,
    );

    if break_points.is_empty() {
        // Fallback to standard fit instead
        // TODO Should we log here (or somewhere else) that we had to fallback?
        return Ok(paragraph_breaker::standard_fit(
            &internal_items,
            &[to_line_breaking_width(line_width)],
            1.0,
        ));
    }

    Ok(break_points)
}

fn to_line_breaking_width(distance: Distance) -> i32 {
    distance.value(DistanceUnit::FontUnits {
        units_per_em: 1000,
        font_size: 10.0,
    }) as i32
}
