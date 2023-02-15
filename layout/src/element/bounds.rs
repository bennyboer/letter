use super::{Position, Size};

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    position: Position,
    size: Size,
}

impl Bounds {
    pub fn new(position: Position, size: Size) -> Self {
        Self { position, size }
    }

    pub fn empty() -> Self {
        Self {
            position: Position::zero(),
            size: Size::zero(),
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, pos: Position) {
        self.position = pos;
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
