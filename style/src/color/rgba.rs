use std::fmt::Display;

use super::{Color, RGBColor, HexColor};

#[derive(Debug, Copy, Clone)]
pub struct RGBAColor {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
    pub alpha: f64,
}

impl Color for RGBAColor {
    fn as_rgb(self) -> RGBColor {
        todo!()
    }

    fn as_rgba(self) -> RGBAColor {
        todo!()
    }

    fn as_hex(self) -> HexColor {
        todo!()
    }
}

impl Display for RGBAColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}
