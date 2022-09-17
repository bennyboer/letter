use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_decimeter_to_millimeter() {
    struct Sample {
        decimeter: f64,
        millimeter: f64,
    }

    let samples = [
        Sample {
            decimeter: 4.0,
            millimeter: 400.0,
        },
        Sample {
            decimeter: 0.00043,
            millimeter: 0.043,
        },
        Sample {
            decimeter: -12.334,
            millimeter: -1233.3999999999999,
        },
        Sample {
            decimeter: 0.0,
            millimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.decimeter, DistanceUnit::Decimeter);
        let result = distance.value(DistanceUnit::Millimeter);
        assert_eq!(sample.millimeter, result);
    }
}

#[test]
fn should_convert_decimeter_to_centimeter() {
    struct Sample {
        decimeter: f64,
        centimeter: f64,
    }

    let samples = [
        Sample {
            decimeter: 40.0,
            centimeter: 400.0,
        },
        Sample {
            decimeter: 0.0043,
            centimeter: 0.043,
        },
        Sample {
            decimeter: -123.34,
            centimeter: -1233.4000000000001,
        },
        Sample {
            decimeter: 0.0,
            centimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.decimeter, DistanceUnit::Decimeter);
        let result = distance.value(DistanceUnit::Centimeter);
        assert_eq!(sample.centimeter, result);
    }
}

#[test]
fn should_convert_decimeter_to_meter() {
    struct Sample {
        decimeter: f64,
        meter: f64,
    }

    let samples = [
        Sample {
            decimeter: 40.0,
            meter: 4.0,
        },
        Sample {
            decimeter: 0.43,
            meter: 0.043,
        },
        Sample {
            decimeter: -123.34,
            meter: -12.334,
        },
        Sample {
            decimeter: 0.0,
            meter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.decimeter, DistanceUnit::Decimeter);
        let result = distance.value(DistanceUnit::Meter);
        assert_eq!(sample.meter, result);
    }
}

#[test]
fn should_convert_decimeter_to_inches() {
    struct Sample {
        decimeter: f64,
        inches: f64,
    }

    let samples = [
        Sample {
            decimeter: 40.0,
            inches: 157.48031496062993,
        },
        Sample {
            decimeter: 0.0043,
            inches: 0.016929133858267716,
        },
        Sample {
            decimeter: -123.34,
            inches: -485.5905511811024,
        },
        Sample {
            decimeter: 0.0,
            inches: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.decimeter, DistanceUnit::Decimeter);
        let result = distance.value(DistanceUnit::Inch);
        assert_eq!(sample.inches, result);
    }
}

#[test]
fn should_convert_decimeter_to_points() {
    struct Sample {
        decimeter: f64,
        points: f64,
    }

    let samples = [
        Sample {
            decimeter: 40.0,
            points: 11338.582677165356,
        },
        Sample {
            decimeter: 0.0043,
            points: 1.2188976377952756,
        },
        Sample {
            decimeter: -123.34,
            points: -34962.51968503938,
        },
        Sample {
            decimeter: 0.0,
            points: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.decimeter, DistanceUnit::Decimeter);
        let result = distance.value(DistanceUnit::Points);
        assert_eq!(sample.points, result);
    }
}

#[test]
fn should_convert_decimeter_to_pixel() {
    struct Sample {
        decimeter: f64,
        pixel_rounded: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            decimeter: 40.0,
            pixel_rounded: 11339.0,
            dots_per_inch: 72,
        },
        Sample {
            decimeter: 0.0043,
            pixel_rounded: 2.0,
            dots_per_inch: 144,
        },
        Sample {
            decimeter: -123.34,
            pixel_rounded: -145677.0,
            dots_per_inch: 300,
        },
        Sample {
            decimeter: 0.0,
            pixel_rounded: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.decimeter, DistanceUnit::Decimeter);
        let result = distance.value(DistanceUnit::Pixel {
            dots_per_inch: sample.dots_per_inch,
        });
        assert_eq!(sample.pixel_rounded, result.round());
    }
}

#[test]
fn should_convert_decimeter_to_font_units() {
    struct Sample {
        decimeter: f64,
        font_units_rounded: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            decimeter: 40.0,
            font_units_rounded: 571429.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
        Sample {
            decimeter: 0.0043,
            font_units_rounded: 98.0,
            units_per_em: 2048,
            font_size: 9.0,
        },
        Sample {
            decimeter: -123.34,
            font_units_rounded: -1897538.0,
            units_per_em: 1000,
            font_size: 6.5,
        },
        Sample {
            decimeter: 0.0,
            font_units_rounded: 0.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.decimeter, DistanceUnit::Decimeter);
        let result = distance.value(DistanceUnit::FontUnits {
            units_per_em: sample.units_per_em,
            font_size: sample.font_size,
        });
        assert_eq!(sample.font_units_rounded, result.round());
    }
}
