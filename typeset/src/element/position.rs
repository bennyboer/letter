use super::ElementId;

#[derive(Debug, Copy, Clone)]
pub enum Position {
    Absolute {
        x: f64,
        y: f64,
    },
    Relative {
        elementId: ElementId,
        x: f64,
        y: f64,
    },
}

impl Position {
    pub fn zero() -> Self {
        Position::Absolute { x: 0.0, y: 0.0 }
    }

    pub fn absolute(x: f64, y: f64) -> Self {
        Position::Absolute { x, y }
    }

    pub fn relativeTo(elementId: ElementId, x: f64, y: f64) -> Self {
        Position::Relative { elementId, x, y }
    }

    pub fn x(&self) -> f64 {
        match self {
            Position::Absolute { x, y: _ } => *x,
            Position::Relative {
                elementId: _,
                x,
                y: _,
            } => *x,
        }
    }

    pub fn y(&self) -> f64 {
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
