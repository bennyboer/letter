use unit::Distance;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub width: Distance,
    pub height: Distance,
}

impl Size {
    pub fn zero() -> Self {
        Self {
            width: Distance::zero(),
            height: Distance::zero(),
        }
    }

    pub fn max() -> Size {
        Self {
            width: Distance::max(),
            height: Distance::max(),
        }
    }

    pub fn new(width: Distance, height: Distance) -> Self {
        Self { width, height }
    }

    pub fn with_width(&self, distance: Distance) -> Size {
        Size {
            width: distance,
            ..*self
        }
    }

    pub fn with_height(&self, distance: Distance) -> Size {
        Size {
            height: distance,
            ..*self
        }
    }

    pub fn is_negative(&self) -> bool {
        self.width < Distance::zero() || self.height < Distance::zero()
    }
}
