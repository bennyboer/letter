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
