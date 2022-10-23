use colorsys::{Cmyk, Rgb};
use std::fmt::Display;

use super::{Color, HexColor, RGBAColor, RGBColor};

#[derive(Debug, Copy, Clone)]
pub struct CMYKColor {
    cyan: f64,
    magenta: f64,
    yellow: f64,
    key: f64,
}

impl CMYKColor {
    pub fn new(
        cyan: f64,
        magenta: f64,
        yellow: f64,
        key: f64,
    ) -> Result<Self, CMYKColorCreateError> {
        assert_value_is_percentage(cyan)?;
        assert_value_is_percentage(magenta)?;
        assert_value_is_percentage(yellow)?;
        assert_value_is_percentage(key)?;

        Ok(Self {
            cyan,
            magenta,
            yellow,
            key,
        })
    }

    pub fn cyan(&self) -> f64 {
        self.cyan
    }

    pub fn magenta(self) -> f64 {
        self.magenta
    }

    pub fn yellow(self) -> f64 {
        self.yellow
    }

    pub fn key(self) -> f64 {
        self.key
    }
}

fn assert_value_is_percentage(value: f64) -> Result<(), CMYKColorCreateError> {
    if value < 0.0 || value > 1.0 {
        return Err(CMYKColorCreateError::ValuesNotGivenAsPercentage);
    }

    Ok(())
}

#[derive(Debug)]
pub enum CMYKColorCreateError {
    ValuesNotGivenAsPercentage,
}

impl Color for CMYKColor {
    fn as_rgb(self) -> RGBColor {
        let colorsys_cmyk = Cmyk::new(
            self.cyan() * 100.0,
            self.magenta() * 100.0,
            self.yellow() * 100.0,
            self.key() * 100.0,
            None,
        );
        let colorsys_rgb: Rgb = colorsys_cmyk.into();

        RGBColor::new(
            colorsys_rgb.red() as usize,
            colorsys_rgb.green() as usize,
            colorsys_rgb.blue() as usize,
        )
        .unwrap()
    }

    fn as_rgba(self) -> RGBAColor {
        self.as_rgb().as_rgba()
    }

    fn as_hex(self) -> HexColor {
        self.as_rgb().as_hex()
    }

    fn as_cmyk(self) -> CMYKColor {
        self
    }
}

impl Display for CMYKColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cmyk({}%, {}%, {}%, {}%)",
            self.cyan, self.magenta, self.yellow, self.key
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_color_to_rgb() {
        // Given: an CMYK color
        let color = CMYKColor::new(0.0, 0.8, 0.7, 0.61).unwrap();

        // When: the color is converted to rgb
        let rgb_color = color.as_rgb();

        // Then: the received color is equal to the original color
        assert_eq!(rgb_color.red(), 99);
        assert_eq!(rgb_color.green(), 19);
        assert_eq!(rgb_color.blue(), 29);
    }

    #[test]
    fn should_convert_color_to_rgba() {
        // Given: an CMYK color
        let color = CMYKColor::new(0.0, 0.8, 0.7, 0.61).unwrap();

        // When: the color is converted to rgba
        let rgba_color = color.as_rgba();

        // Then: the received color is equal to the original color
        assert_eq!(rgba_color.red(), 99);
        assert_eq!(rgba_color.green(), 19);
        assert_eq!(rgba_color.blue(), 29);
        assert_eq!(rgba_color.alpha(), 1.0);
    }

    #[test]
    fn should_convert_color_to_hex() {
        // Given: an CMYK color
        let color = CMYKColor::new(0.0, 0.8, 0.7, 0.61).unwrap();

        // When: the color is converted to hex
        let hex_color = color.as_hex();

        // Then: the received color is equal to the original color
        assert_eq!(hex_color.to_string(), "#63131DFF".to_string());
    }

    #[test]
    fn should_convert_color_to_cmyk() {
        // Given: an CMYK color
        let color = CMYKColor::new(0.2, 0.4, 0.6, 0.8).unwrap();

        // When: the color is converted to CMYK
        let cmyk_color = color.as_cmyk();

        // Then: the received color is equal to the original color
        assert_eq!(cmyk_color.cyan(), 0.2);
        assert_eq!(cmyk_color.magenta(), 0.4);
        assert_eq!(cmyk_color.yellow(), 0.6);
        assert_eq!(cmyk_color.key(), 0.8);
    }
}
