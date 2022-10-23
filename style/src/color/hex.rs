use colorsys::{Cmyk, Rgb};
use std::fmt::Display;

use super::{CMYKColor, Color, RGBAColor, RGBColor};

#[derive(Debug, Copy, Clone)]
pub struct HexColor {
    value: usize,
}

impl HexColor {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

impl Color for HexColor {
    fn as_rgb(self) -> RGBColor {
        let mut value = self.value();

        value = value >> 8;
        let blue = value & 255;
        value = value >> 8;
        let green = value & 255;
        value = value >> 8;
        let red = value & 255;

        RGBColor::new(red, green, blue).unwrap()
    }

    fn as_rgba(self) -> RGBAColor {
        let mut value = self.value();

        let alpha = (value & 255) as f64 / 255.0;
        value = value >> 8;
        let blue = value & 255;
        value = value >> 8;
        let green = value & 255;
        value = value >> 8;
        let red = value & 255;

        RGBAColor::new(red, green, blue, alpha).unwrap()
    }

    fn as_hex(self) -> HexColor {
        self
    }

    fn as_cmyk(self) -> CMYKColor {
        let rgba = self.as_rgba();
        let colorsys_rgb = Rgb::new(
            rgba.red() as f64,
            rgba.green() as f64,
            rgba.blue() as f64,
            Some(rgba.alpha() * 255.0),
        );
        let colorsys_cmyk: Cmyk = colorsys_rgb.into();

        let cyan = colorsys_cmyk.cyan() / 100.0;
        let magenta = colorsys_cmyk.magenta() / 100.0;
        let yellow = colorsys_cmyk.yellow() / 100.0;
        let key = colorsys_cmyk.key() / 100.0;

        CMYKColor::new(cyan, magenta, yellow, key).unwrap()
    }
}

impl Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgba = self.as_rgba();
        let colorsys_rgb = Rgb::new(
            rgba.red() as f64,
            rgba.green() as f64,
            rgba.blue() as f64,
            Some(rgba.alpha() * 255.0),
        );

        let hex_without_alpha = colorsys_rgb.to_hex_string().to_uppercase();
        let hex_alpha_part = format!("{:02X}", (rgba.alpha() * 255.0) as usize).to_string();

        write!(f, "{}{}", hex_without_alpha, hex_alpha_part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_color_to_rgb() {
        // Given: a hex color
        let color = HexColor::new(0x6699CCFF);

        // When: the color is converted to rgb
        let rgb_color = color.as_rgb();

        // Then: the received color is equal to the original color
        assert_eq!(rgb_color.red(), 102);
        assert_eq!(rgb_color.green(), 153);
        assert_eq!(rgb_color.blue(), 204);
    }

    #[test]
    fn should_convert_color_to_rgba() {
        // Given: a hex color
        let color = HexColor::new(0x6699CC66);

        // When: the color is converted to rgba
        let rgba_color = color.as_rgba();

        // Then: the received color is equal to the original color
        assert_eq!(rgba_color.red(), 102);
        assert_eq!(rgba_color.green(), 153);
        assert_eq!(rgba_color.blue(), 204);
        assert_eq!(rgba_color.alpha(), 0.4);
    }

    #[test]
    fn should_convert_color_to_hex() {
        // Given: a hex color
        let color = HexColor::new(0x6699CC66);

        // When: the color is converted to hex
        let hex_color = color.as_hex();

        // Then: the received color is equal to the original color
        assert_eq!(hex_color.to_string(), "#6699CC66");
    }

    #[test]
    fn should_convert_color_to_cmyk() {
        // Given: an hex color
        let color = HexColor::new(0xFF643266);

        // When: the color is converted to CMYK
        let cmyk_color = color.as_cmyk();

        // Then: the received color is equal to the original color
        assert_eq!(cmyk_color.cyan(), 0.0);
        assert_eq!(cmyk_color.magenta(), 0.607843137254902);
        assert_eq!(cmyk_color.yellow(), 0.803921568627451);
        assert_eq!(cmyk_color.key(), 0.0);
    }
}
