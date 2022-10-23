mod cmyk;
mod hex;
mod rgb;
mod rgba;

use core::fmt;

pub use cmyk::CMYKColor;
pub use hex::HexColor;
pub use rgb::RGBColor;
pub use rgba::RGBAColor;

pub trait Color: fmt::Display {
    fn as_rgb(self) -> RGBColor;
    fn as_rgba(self) -> RGBAColor;
    fn as_hex(self) -> HexColor;
    fn as_cmyk(self) -> CMYKColor;
}
