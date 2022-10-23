use colorsys::{Cmyk, Rgb};
use std::fmt::Display;

use super::{CMYKColor, Color, HexColor, RGBAColor};

#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    red: usize,
    green: usize,
    blue: usize,
}

impl RGBColor {
    pub fn new(red: usize, green: usize, blue: usize) -> Result<Self, RGBColorCreateError> {
        assert_color_value_valid(red)?;
        assert_color_value_valid(green)?;
        assert_color_value_valid(blue)?;

        Ok(Self { red, green, blue })
    }

    pub fn red(&self) -> usize {
        self.red
    }

    pub fn green(&self) -> usize {
        self.green
    }

    pub fn blue(&self) -> usize {
        self.blue
    }
}

fn assert_color_value_valid(value: usize) -> Result<(), RGBColorCreateError> {
    if value > 255 {
        return Err(RGBColorCreateError::ColorValueInvalid);
    }

    Ok(())
}

#[derive(Debug)]
pub enum RGBColorCreateError {
    ColorValueInvalid,
}

impl Color for RGBColor {
    fn as_rgb(self) -> RGBColor {
        self
    }

    fn as_rgba(self) -> RGBAColor {
        RGBAColor::new(self.red(), self.green(), self.blue(), 1.0).unwrap()
    }

    fn as_hex(self) -> HexColor {
        let red = self.red.clamp(self.red(), 255);
        let green = self.green.clamp(self.green(), 255);
        let blue = self.blue.clamp(self.blue(), 255);

        let mut hex_value = red;
        hex_value = hex_value << 8;
        hex_value = hex_value | green;
        hex_value = hex_value << 8;
        hex_value = hex_value | blue;
        hex_value = hex_value << 8;
        hex_value = hex_value | 255;

        HexColor::new(hex_value)
    }

    fn as_cmyk(self) -> CMYKColor {
        let colorsys_rgb = Rgb::new(
            self.red() as f64,
            self.green() as f64,
            self.blue() as f64,
            None,
        );
        let colorsys_cmyk: Cmyk = colorsys_rgb.into();

        let cyan = colorsys_cmyk.cyan() / 100.0;
        let magenta = colorsys_cmyk.magenta() / 100.0;
        let yellow = colorsys_cmyk.yellow() / 100.0;
        let key = colorsys_cmyk.key() / 100.0;

        CMYKColor::new(cyan, magenta, yellow, key).unwrap()
    }
}

impl Display for RGBColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.red(), self.green(), self.blue())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_color_to_rgba() {
        // Given: an RGB color
        let rgb_color = RGBColor::new(100, 20, 30).unwrap();

        // When: the color is converted to RGBA
        let rgba_color = rgb_color.as_rgba();

        // Then: the received color is opaque
        assert_eq!(rgba_color.alpha(), 1.0);

        // And: the remaining values are copied from the original RGB color
        assert_eq!(rgba_color.red(), 100);
        assert_eq!(rgba_color.green(), 20);
        assert_eq!(rgba_color.blue(), 30);
    }

    #[test]
    fn should_convert_color_to_rgb() {
        // Given: an RGB color
        let color = RGBColor::new(100, 20, 30).unwrap();

        // When: the color is converted to RGB
        let result = color.as_rgba();

        // Then: the received color contains the same values
        assert_eq!(result.red(), 100);
        assert_eq!(result.green(), 20);
        assert_eq!(result.blue(), 30);
    }

    #[test]
    fn should_convert_color_to_hex() {
        // Given: an RGB color
        let color = RGBColor::new(255, 100, 50).unwrap();

        // When: the color is converted to hex
        let hex_color = color.as_hex();

        // Then: the received color is equal to the original color
        assert_eq!(hex_color.to_string(), "#FF6432FF".to_string());
    }

    #[test]
    fn should_convert_color_to_cmyk() {
        // Given: an RGB color
        let rgb_color = RGBColor::new(100, 20, 30).unwrap();

        // When: the color is converted to CMYK
        let cmyk_color = rgb_color.as_cmyk();

        // Then: the remaining values are correct CMYK
        assert_eq!(cmyk_color.cyan(), 0.0);
        assert_eq!(cmyk_color.magenta(), 0.8);
        assert_eq!(cmyk_color.yellow(), 0.6999999999999998);
        assert_eq!(cmyk_color.key(), 0.607843137254902);
    }
}
