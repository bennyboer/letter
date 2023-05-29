use document::style::{FontStretch, FontStyle, FontWeight};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct HashableFloat {
    encoded_value: i64,
}

impl HashableFloat {
    pub fn new(value: f32) -> Self {
        Self {
            encoded_value: (value as f64 * 1000.0) as i64,
        }
    }

    pub fn value(&self) -> f32 {
        ((self.encoded_value as f64) / 1000.0) as f32
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct FontStyleSettings {
    style: FontStyle,
    weight: HashableFloat,
    stretch: HashableFloat,
}

impl FontStyleSettings {
    pub fn new(style: FontStyle, weight: FontWeight, stretch: FontStretch) -> Self {
        Self {
            style,
            weight: HashableFloat::new(weight),
            stretch: HashableFloat::new(stretch),
        }
    }

    pub fn style(&self) -> FontStyle {
        self.style
    }

    pub fn weight(&self) -> FontWeight {
        self.weight.value()
    }

    pub fn stretch(&self) -> FontStretch {
        self.stretch.value()
    }
}
