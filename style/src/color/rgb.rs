use std::fmt::Display;

use super::{Color, HexColor, RGBAColor};

#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Color for RGBColor {
    fn as_rgb(self) -> RGBColor {
        self
    }

    fn as_rgba(self) -> RGBAColor {
        RGBAColor {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: 1.0,
        }
    }

    fn as_hex(self) -> HexColor {
        let red = self.red.clamp(self.red, 255);
        let green = self.green.clamp(self.green, 255);
        let blue = self.blue.clamp(self.blue, 255);

        let mut hex_value = blue;
        hex_value = hex_value << 8;
        hex_value = hex_value | green;
        hex_value = hex_value << 8;
        hex_value = hex_value | red;

        HexColor { value: hex_value }
    }
}

impl Display for RGBColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_color_to_rgba() {
        // Given: an RGB color
        let rgb_color = RGBColor {
            red: 100,
            green: 20,
            blue: 30,
        };

        // When: the color is converted to RGBA
        let rgba_color = rgb_color.as_rgba();

        // Then: the received color is opaque
        assert_eq!(rgba_color.alpha, 1.0);

        // And: the remaining values are copied from the original RGB color
        assert_eq!(rgba_color.red, 100);
        assert_eq!(rgba_color.green, 20);
        assert_eq!(rgba_color.blue, 30);
    }

    #[test]
    fn should_convert_color_to_rgb() {
        // Given: an RGB color
        let color = RGBColor {
            red: 100,
            green: 20,
            blue: 30,
        };

        // When: the color is converted to RGB
        let result = color.as_rgba();

        // Then: the received color contains the same values
        assert_eq!(result.red, 100);
        assert_eq!(result.green, 20);
        assert_eq!(result.blue, 30);
    }

    #[test]
    fn should_convert_color_to_hex() {
        // Given: an RGB color
        let color = RGBColor {
            red: 255,
            green: 100,
            blue: 50,
        };

        // When: the color is converted to hex
        let hex_color = color.as_hex();

        // Then: the received color is equal to the original color
        assert_eq!(hex_color.to_string(), "#FF0000".to_string());
    }
}
