use std::fmt::Display;

use super::{Color, RGBAColor, RGBColor};

#[derive(Debug, Copy, Clone)]
pub struct HexColor {
    pub value: usize,
}

impl Color for HexColor {
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

impl Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let red = self.value & 255;
        let green = self.value << 8 & 255;
        let blue = self.value << 16 & 255;

        write!(f, "#{:02X}{:02X}{:02X}", red, green, blue)
    }
}
