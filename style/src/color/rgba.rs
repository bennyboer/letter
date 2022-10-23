use std::fmt::Display;

use colorsys::{Cmyk, Rgb};

use super::{CMYKColor, Color, HexColor, RGBColor};

#[derive(Debug, Copy, Clone)]
pub struct RGBAColor {
    red: usize,
    green: usize,
    blue: usize,
    alpha: f64,
}

impl RGBAColor {
    pub fn new(
        red: usize,
        green: usize,
        blue: usize,
        alpha: f64,
    ) -> Result<Self, RGBAColorCreateError> {
        assert_color_value_valid(red)?;
        assert_color_value_valid(green)?;
        assert_color_value_valid(blue)?;
        if alpha < 0.0 || alpha > 1.0 {
            return Err(RGBAColorCreateError::ColorValueInvalid);
        }

        Ok(Self {
            red,
            green,
            blue,
            alpha,
        })
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

    pub fn alpha(&self) -> f64 {
        self.alpha
    }
}

fn assert_color_value_valid(value: usize) -> Result<(), RGBAColorCreateError> {
    if value > 255 {
        return Err(RGBAColorCreateError::ColorValueInvalid);
    }

    Ok(())
}

#[derive(Debug)]
pub enum RGBAColorCreateError {
    ColorValueInvalid,
}

impl Color for RGBAColor {
    fn as_rgb(self) -> RGBColor {
        let red = self.red();
        let green = self.green();
        let blue = self.blue();

        RGBColor::new(red, green, blue).unwrap()
    }

    fn as_rgba(self) -> RGBAColor {
        self
    }

    fn as_hex(self) -> HexColor {
        let red = self.red().clamp(self.red(), 255);
        let green = self.green().clamp(self.green(), 255);
        let blue = self.blue().clamp(self.blue(), 255);
        let alpha = (self.alpha() * 255.0) as usize;

        let mut hex_value = red;
        hex_value = hex_value << 8;
        hex_value = hex_value | green;
        hex_value = hex_value << 8;
        hex_value = hex_value | blue;
        hex_value = hex_value << 8;
        hex_value = hex_value | alpha;

        HexColor::new(hex_value)
    }

    fn as_cmyk(self) -> CMYKColor {
        let colorsys_rgb = Rgb::new(
            self.red() as f64,
            self.green() as f64,
            self.blue() as f64,
            Some(self.alpha() * 255.0),
        );
        let colorsys_cmyk: Cmyk = colorsys_rgb.into();

        let cyan = colorsys_cmyk.cyan() / 100.0;
        let magenta = colorsys_cmyk.magenta() / 100.0;
        let yellow = colorsys_cmyk.yellow() / 100.0;
        let key = colorsys_cmyk.key() / 100.0;

        CMYKColor::new(cyan, magenta, yellow, key).unwrap()
    }
}

impl Display for RGBAColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {})",
            self.red(),
            self.green(),
            self.blue(),
            self.alpha()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_color_to_rgb() {
        // Given: a rgba color
        let color = RGBAColor::new(255, 100, 50, 0.4).unwrap();

        // When: the color is converted to rgb
        let rgb_color = color.as_rgb();

        // Then: the received color is equal to the original color
        assert_eq!(rgb_color.red(), 255);
        assert_eq!(rgb_color.green(), 100);
        assert_eq!(rgb_color.blue(), 50);
    }

    #[test]
    fn should_convert_color_to_rgba() {
        // Given: a rgba color
        let color = RGBAColor::new(255, 100, 50, 0.4).unwrap();

        // When: the color is converted to rgba
        let rgba_color = color.as_rgba();

        // Then: the received color is equal to the original color
        assert_eq!(rgba_color.red(), 255);
        assert_eq!(rgba_color.green(), 100);
        assert_eq!(rgba_color.blue(), 50);
        assert_eq!(rgba_color.alpha(), 0.4);
    }

    #[test]
    fn should_convert_color_to_hex() {
        // Given: an RGBA color
        let color = RGBAColor::new(255, 100, 50, 0.4).unwrap();

        // When: the color is converted to hex
        let hex_color = color.as_hex();

        // Then: the received color is equal to the original color
        assert_eq!(hex_color.to_string(), "#FF643266".to_string());
    }

    #[test]
    fn should_convert_color_to_cmyk() {
        // Given: an RGBA color
        let color = RGBAColor::new(255, 100, 50, 0.4).unwrap();

        // When: the color is converted to CMYK
        let cmyk_color = color.as_cmyk();

        // Then: the received color is equal to the original color
        assert_eq!(cmyk_color.cyan(), 0.0);
        assert_eq!(cmyk_color.magenta(), 0.607843137254902);
        assert_eq!(cmyk_color.yellow(), 0.803921568627451);
        assert_eq!(cmyk_color.key(), 0.0);
    }
}
