use unit::Distance;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    x: Distance,
    y: Distance,
}

impl Position {
    pub fn zero() -> Self {
        Self {
            x: Distance::zero(),
            y: Distance::zero(),
        }
    }

    pub fn absolute(x: Distance, y: Distance) -> Self {
        Self { x, y }
    }

    pub fn relative_to(position: &Position, x: Distance, y: Distance) -> Self {
        Self::absolute(position.x() + x, position.y() + y)
    }

    pub fn x(&self) -> Distance {
        self.x
    }

    pub fn y(&self) -> Distance {
        self.y
    }
}
