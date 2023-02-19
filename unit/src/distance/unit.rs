use crate::UnitValue;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistanceUnit {
    /// The base unit for the library is Millimeters.
    Millimeter,
    Centimeter,
    Decimeter,
    Meter,
    Inch,
    /// Printer points (pt)
    Points,
    Pixel {
        /// Dots per inch of the monitor.
        dots_per_inch: usize,
    },
    FontUnits {
        /// Units per em define the font-internal coordinate system.
        /// This value is given for each font file.
        units_per_em: usize,
        /// Font size given in Millimeters.
        font_size: f64,
    },
}

impl DistanceUnit {
    fn base_factor(&self) -> f64 {
        match self {
            DistanceUnit::Millimeter => 1.0,
            DistanceUnit::Centimeter => 10.0,
            DistanceUnit::Decimeter => 100.0,
            DistanceUnit::Meter => 1000.0,
            DistanceUnit::Inch => 25.4,
            DistanceUnit::Points => 25.4 / 72.0,
            DistanceUnit::Pixel { dots_per_inch } => 25.4 / (*dots_per_inch as f64),
            DistanceUnit::FontUnits {
                units_per_em,
                font_size,
            } => font_size / (*units_per_em as f64),
        }
    }

    pub fn to_base(&self, value: UnitValue) -> UnitValue {
        value * self.base_factor()
    }

    pub fn from_base(&self, base_value: UnitValue) -> UnitValue {
        base_value / self.base_factor()
    }

    pub fn shortform(&self) -> &str {
        match self {
            DistanceUnit::Millimeter => "mm",
            DistanceUnit::Centimeter => "cm",
            DistanceUnit::Decimeter => "dm",
            DistanceUnit::Meter => "m",
            DistanceUnit::Inch => "in",
            DistanceUnit::Points => "pt",
            DistanceUnit::Pixel { dots_per_inch: _ } => "px",
            DistanceUnit::FontUnits {
                units_per_em: _,
                font_size: _,
            } => "FUnits",
        }
    }

    pub fn from_shortform(shortform: &str) -> Option<Self> {
        match shortform {
            "mm" => Some(DistanceUnit::Millimeter),
            "cm" => Some(DistanceUnit::Centimeter),
            "dm" => Some(DistanceUnit::Decimeter),
            "m" => Some(DistanceUnit::Meter),
            "in" => Some(DistanceUnit::Inch),
            "pt" => Some(DistanceUnit::Points),
            _ => None,
        }
    }
}
