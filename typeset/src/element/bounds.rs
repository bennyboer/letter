use super::{Position, Size};

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub position: Position,
    pub size: Size,
}

impl Bounds {
    pub fn empty() -> Self {
        Self {
            position: Position::zero(),
            size: Size::zero(),
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
