mod hex;
mod rgb;
mod rgba;

use core::fmt;

pub use rgb::RGBColor;
pub use rgba::RGBAColor;
pub use hex::HexColor;

pub trait Color: fmt::Display {
    fn as_rgb(self) -> RGBColor;
    fn as_rgba(self) -> RGBAColor;
    fn as_hex(self) -> HexColor;
}
