use unit::Distance;

use crate::context::insets::Insets;
use crate::element::Size;

#[derive(Debug, Clone)]
pub(crate) struct LayoutStyle {
    size: Size,
    margin: Insets,
    padding: Insets,
}

impl LayoutStyle {
    pub fn new() -> Self {
        Self {
            size: Size::zero(),
            margin: Insets::zero(),
            padding: Insets::zero(),
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

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_margin(&mut self, margin: Insets) {
        self.margin = margin;
    }

    pub fn set_padding(&mut self, padding: Insets) {
        self.padding = padding;
    }
}
