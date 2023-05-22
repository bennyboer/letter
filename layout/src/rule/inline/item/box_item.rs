use document::structure::NodeId;
use unit::Distance;

use crate::context::LayoutStyle;
use crate::rule::inline::item::box_content::BoxContent;

#[derive(Debug)]
pub(crate) struct BoxItem {
    width: Distance,
    content: BoxContent,
    node: NodeId,
    style: LayoutStyle,
}

impl BoxItem {
    pub fn new(width: Distance, content: BoxContent, node: NodeId, style: LayoutStyle) -> Self {
        Self {
            width,
            content,
            node,
            style,
        }
    }

    pub fn width(&self) -> Distance {
        self.width
    }

    pub fn content(&self) -> &BoxContent {
        &self.content
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn style(&self) -> &LayoutStyle {
        &self.style
    }
}
