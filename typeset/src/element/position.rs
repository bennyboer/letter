use unit::Distance;

use super::ElementId;

#[derive(Debug, Copy, Clone)]
pub enum Position {
    Absolute {
        x: Distance,
        y: Distance,
    },
    Relative {
        elementId: ElementId,
        x: Distance,
        y: Distance,
    },
}

impl Position {
    pub fn zero() -> Self {
        Position::Absolute {
            x: Distance::zero(),
            y: Distance::zero(),
        }
    }

    pub fn absolute(x: Distance, y: Distance) -> Self {
        Position::Absolute { x, y }
    }

    pub fn relativeTo(elementId: ElementId, x: Distance, y: Distance) -> Self {
        Position::Relative { elementId, x, y }
    }

    pub fn x(&self) -> Distance {
        match self {
            Position::Absolute { x, y: _ } => *x,
            Position::Relative {
                elementId: _,
                x,
                y: _,
            } => *x,
        }
    }

    pub fn y(&self) -> Distance {
        match self {
            Position::Absolute { x: _, y } => *y,
            Position::Relative {
                elementId: _,
                x: _,
                y,
            } => *y,
        }
    }
}
