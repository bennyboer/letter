#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn zero() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}
