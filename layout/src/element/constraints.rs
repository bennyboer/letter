use unit::Distance;

use crate::element::Size;

#[derive(Debug)]
pub struct LayoutConstraints {
    size: Size,
    top: Distance,
    bottom: Distance,
    left: Distance,
    right: Distance,
}

impl LayoutConstraints {
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn top(&self) -> Distance {
        self.top
    }

    pub fn bottom(&self) -> Distance {
        self.bottom
    }

    pub fn left(&self) -> Distance {
        self.left
    }

    pub fn right(&self) -> Distance {
        self.right
    }
}
