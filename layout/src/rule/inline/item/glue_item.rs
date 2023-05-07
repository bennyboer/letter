use unit::Distance;

#[derive(Debug)]
pub(crate) struct GlueItem {
    width: Distance,
    stretch: Distance,
    shrink: Distance,
}

impl GlueItem {
    pub fn new(width: Distance, stretch: Distance, shrink: Distance) -> Self {
        Self {
            width,
            stretch,
            shrink,
        }
    }

    pub fn width(&self) -> Distance {
        self.width
    }

    pub fn stretch(&self) -> Distance {
        self.stretch
    }

    pub fn shrink(&self) -> Distance {
        self.shrink
    }
}
