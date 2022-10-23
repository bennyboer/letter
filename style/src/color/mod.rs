mod cmyk;
mod hex;
mod rgb;
mod rgba;

use core::fmt::{Debug, Display};

pub use cmyk::CMYKColor;
pub use hex::HexColor;
pub use rgb::RGBColor;
pub use rgba::RGBAColor;

pub trait Color: Display {
    fn as_rgb(self) -> RGBColor;
    fn as_rgba(self) -> RGBAColor;
    fn as_hex(self) -> HexColor;
    fn as_cmyk(self) -> CMYKColor;
}

impl Debug for dyn Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct Colors;

impl Colors {
    pub const BLACK: HexColor = HexColor::new(0x000000FF);
}
