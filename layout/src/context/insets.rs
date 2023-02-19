use unit::Distance;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Insets {
    top: Distance,
    right: Distance,
    bottom: Distance,
    left: Distance,
}

impl Insets {
    pub(crate) fn zero() -> Self {
        Self {
            top: Distance::zero(),
            right: Distance::zero(),
            bottom: Distance::zero(),
            left: Distance::zero(),
        }
    }

    pub fn top(&self) -> Distance {
        self.top
    }

    pub fn right(&self) -> Distance {
        self.right
    }

    pub fn bottom(&self) -> Distance {
        self.bottom
    }

    pub fn left(&self) -> Distance {
        self.left
    }

    pub(crate) fn with_top(self, top: Distance) -> Self {
        Self { top, ..self }
    }

    pub(crate) fn with_right(self, right: Distance) -> Self {
        Self { right, ..self }
    }

    pub(crate) fn with_bottom(self, bottom: Distance) -> Self {
        Self { bottom, ..self }
    }

    pub(crate) fn with_left(self, left: Distance) -> Self {
        Self { left, ..self }
    }
}
