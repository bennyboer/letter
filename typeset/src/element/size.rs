use unit::Distance;

#[derive(Debug, Copy, Clone)]
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

    pub fn new(width: Distance, height: Distance) -> Self {
        Self { width, height }
    }
}
