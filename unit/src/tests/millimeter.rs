use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_millimeter_to_centimeters() {
    struct Sample {
        millimeter: f64,
        centimeter: f64,
    }

    let samples = [
        Sample {
            millimeter: 40.0,
            centimeter: 4.0,
        },
        Sample {
            millimeter: 0.0043,
            centimeter: 0.00043,
        },
        Sample {
            millimeter: -123.34,
            centimeter: -12.334,
        },
        Sample {
            millimeter: 0.0,
            centimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.millimeter, DistanceUnit::Millimeter);
        let result = distance.value(DistanceUnit::Centimeter);
        assert_eq!(sample.centimeter, result);
    }
}

#[test]
fn should_convert_millimeter_to_decimeter() {
    struct Sample {
        millimeter: f64,
        decimeter: f64,
    }

    let samples = [
        Sample {
            millimeter: 40.0,
            decimeter: 0.4,
        },
        Sample {
            millimeter: 0.0043,
            decimeter: 0.000043,
        },
        Sample {
            millimeter: -123.34,
            decimeter: -1.2334,
        },
        Sample {
            millimeter: 0.0,
            decimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.millimeter, DistanceUnit::Millimeter);
        let result = distance.value(DistanceUnit::Decimeter);
        assert_eq!(sample.decimeter, result);
    }
}

#[test]
fn should_convert_millimeter_to_meter() {
    struct Sample {
        millimeter: f64,
        meter: f64,
    }

    let samples = [
        Sample {
            millimeter: 40.0,
            meter: 0.04,
        },
        Sample {
            millimeter: 0.0043,
            meter: 0.0000043,
        },
        Sample {
            millimeter: -123.34,
            meter: -0.12334,
        },
        Sample {
            millimeter: 0.0,
            meter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.millimeter, DistanceUnit::Millimeter);
        let result = distance.value(DistanceUnit::Meter);
        assert_eq!(sample.meter, result);
    }
}

#[test]
fn should_convert_millimeter_to_inches() {
    struct Sample {
        millimeter: f64,
        inches: f64,
    }

    let samples = [
        Sample {
            millimeter: 40.0,
            inches: 1.5748031496062993,
        },
        Sample {
            millimeter: 0.0043,
            inches: 0.00016929133858267718,
        },
        Sample {
            millimeter: -123.34,
            inches: -4.855905511811024,
        },
        Sample {
            millimeter: 0.0,
            inches: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.millimeter, DistanceUnit::Millimeter);
        let result = distance.value(DistanceUnit::Inch);
        assert_eq!(sample.inches, result);
    }
}

#[test]
fn should_convert_millimeter_to_points() {
    struct Sample {
        millimeter: f64,
        points: f64,
    }

    let samples = [
        Sample {
            millimeter: 40.0,
            points: 113.38582677165356,
        },
        Sample {
            millimeter: 0.0043,
            points: 0.012188976377952757,
        },
        Sample {
            millimeter: -123.34,
            points: -349.62519685039376,
        },
        Sample {
            millimeter: 0.0,
            points: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.millimeter, DistanceUnit::Millimeter);
        let result = distance.value(DistanceUnit::Points);
        assert_eq!(sample.points, result);
    }
}

#[test]
fn should_convert_millimeter_to_pixel() {
    struct Sample {
        millimeter: f64,
        pixel_rounded: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            millimeter: 40.0,
            pixel_rounded: 113.0,
            dots_per_inch: 72,
        },
        Sample {
            millimeter: 0.0043,
            pixel_rounded: 0.0,
            dots_per_inch: 144,
        },
        Sample {
            millimeter: -123.34,
            pixel_rounded: -1457.0,
            dots_per_inch: 300,
        },
        Sample {
            millimeter: 0.0,
            pixel_rounded: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.millimeter, DistanceUnit::Millimeter);
        let result = distance.value(DistanceUnit::Pixel {
            dots_per_inch: sample.dots_per_inch,
        });
        assert_eq!(sample.pixel_rounded, result.round());
    }
}

#[test]
fn should_convert_millimeter_to_font_units() {
    struct Sample {
        millimeter: f64,
        font_units_rounded: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            millimeter: 40.0,
            font_units_rounded: 5714.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
        Sample {
            millimeter: 0.0043,
            font_units_rounded: 1.0,
            units_per_em: 2048,
            font_size: 9.0,
        },
        Sample {
            millimeter: -123.34,
            font_units_rounded: -18975.0,
            units_per_em: 1000,
            font_size: 6.5,
        },
        Sample {
            millimeter: 0.0,
            font_units_rounded: 0.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.millimeter, DistanceUnit::Millimeter);
        let result = distance.value(DistanceUnit::FontUnits {
            units_per_em: sample.units_per_em,
            font_size: sample.font_size,
        });
        assert_eq!(sample.font_units_rounded, result.round());
    }
}
