mod font;

pub use font::FontStyle;

use crate::color::Color;

pub enum Style {
    Font(FontStyle),
    Color(Box<dyn Color>),
    Padding,
    Margin,
}
