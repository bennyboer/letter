use unit::Distance;

pub(crate) use crate::rule::inline::item::box_content::BoxContent;
pub(crate) use crate::rule::inline::item::box_item::BoxItem;
pub(crate) use crate::rule::inline::item::glue_item::GlueItem;
pub(crate) use crate::rule::inline::item::penalty_item::PenaltyItem;

mod box_content;
mod box_item;
mod glue_item;
mod penalty_item;

#[derive(Debug)]
pub(crate) enum Item {
    Box(BoxItem),
    Glue(GlueItem),
    Penalty(PenaltyItem),
}

impl Item {
    pub fn width(&self) -> Distance {
        match self {
            Item::Box(box_item) => box_item.width(),
            Item::Glue(glue_item) => glue_item.width(),
            Item::Penalty(penalty_item) => penalty_item.width(),
        }
    }
}
