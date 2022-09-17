use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_font_units_to_millimeter() {
    struct Sample {
        font_units: f64,
        millimeter: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            font_units: 4.0,
            millimeter: 0.02,
            units_per_em: 1000,
            font_size: 5.0,
        },
        Sample {
            font_units: 0.00043,
            millimeter: 6.298828125e-7,
            units_per_em: 2048,
            font_size: 3.0,
        },
        Sample {
            font_units: -12.334,
            millimeter: -0.030835,
            units_per_em: 1000,
            font_size: 2.5,
        },
        Sample {
            font_units: 0.0,
            millimeter: 0.0,
            units_per_em: 1024,
            font_size: 1.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.font_units,
            DistanceUnit::FontUnits {
                units_per_em: sample.units_per_em,
                font_size: sample.font_size,
            },
        );
        let result = distance.value(DistanceUnit::Millimeter);
        assert_eq!(sample.millimeter, result);
    }
}

#[test]
fn should_convert_font_units_to_centimeter() {
    struct Sample {
        font_units: f64,
        centimeter: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            font_units: 40.0,
            centimeter: 0.016,
            units_per_em: 1000,
            font_size: 4.0,
        },
        Sample {
            font_units: 0.0043,
            centimeter: 1.15478515625e-6,
            units_per_em: 2048,
            font_size: 5.5,
        },
        Sample {
            font_units: -123.34,
            centimeter: -0.02408984375,
            units_per_em: 1024,
            font_size: 2.0,
        },
        Sample {
            font_units: 0.0,
            centimeter: 0.0,
            units_per_em: 1000,
            font_size: 1.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.font_units,
            DistanceUnit::FontUnits {
                units_per_em: sample.units_per_em,
                font_size: sample.font_size,
            },
        );
        let result = distance.value(DistanceUnit::Centimeter);
        assert_eq!(sample.centimeter, result);
    }
}

#[test]
fn should_convert_font_units_to_decimeter() {
    struct Sample {
        font_units: f64,
        decimeter: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            font_units: 4.0,
            decimeter: 0.00021999999999999998,
            units_per_em: 1000,
            font_size: 5.5,
        },
        Sample {
            font_units: 0.043,
            decimeter: 1.5117187499999999e-5,
            units_per_em: 2048,
            font_size: 72.0,
        },
        Sample {
            font_units: -12.334,
            decimeter: -0.0008431445312499999,
            units_per_em: 1024,
            font_size: 7.0,
        },
        Sample {
            font_units: 0.0,
            decimeter: 0.0,
            units_per_em: 1000,
            font_size: 1.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.font_units,
            DistanceUnit::FontUnits {
                units_per_em: sample.units_per_em,
                font_size: sample.font_size,
            },
        );
        let result = distance.value(DistanceUnit::Decimeter);
        assert_eq!(sample.decimeter, result);
    }
}

#[test]
fn should_convert_font_units_to_meter() {
    struct Sample {
        font_units: f64,
        meter: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            font_units: 1574.8031496062993,
            meter: 0.018897637795275594,
            units_per_em: 1000,
            font_size: 12.0,
        },
        Sample {
            font_units: 0.16929133858267717,
            meter: 5.9516486220472436e-6,
            units_per_em: 2048,
            font_size: 72.0,
        },
        Sample {
            font_units: -4855.905511811024,
            meter: -0.014226285679133858,
            units_per_em: 1024,
            font_size: 3.0,
        },
        Sample {
            font_units: 0.0,
            meter: 0.0,
            units_per_em: 1000,
            font_size: 1.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.font_units,
            DistanceUnit::FontUnits {
                units_per_em: sample.units_per_em,
                font_size: sample.font_size,
            },
        );
        let result = distance.value(DistanceUnit::Meter);
        assert_eq!(sample.meter, result);
    }
}

#[test]
fn should_convert_font_units_to_inch() {
    struct Sample {
        font_units: f64,
        inch: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            font_units: 2880.0000000000005,
            inch: 1.360629921259843,
            units_per_em: 1000,
            font_size: 12.0,
        },
        Sample {
            font_units: 0.30960000000000004,
            inch: 7.141978346456693e-5,
            units_per_em: 2048,
            font_size: 12.0,
        },
        Sample {
            font_units: -8880.48,
            inch: -4.0971702755905515,
            units_per_em: 1024,
            font_size: 12.0,
        },
        Sample {
            font_units: 0.0,
            inch: 0.0,
            units_per_em: 1000,
            font_size: 1.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.font_units,
            DistanceUnit::FontUnits {
                units_per_em: sample.units_per_em,
                font_size: sample.font_size,
            },
        );
        let result = distance.value(DistanceUnit::Inch);
        assert_eq!(sample.inch, result);
    }
}

#[test]
fn should_convert_font_units_to_pixel() {
    struct Sample {
        font_units: f64,
        pixel_rounded: f64,
        units_per_em: usize,
        font_size: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            font_units: 40.0,
            pixel_rounded: 1.0,
            units_per_em: 1000,
            font_size: 12.0,
            dots_per_inch: 72,
        },
        Sample {
            font_units: 0.0043,
            pixel_rounded: 0.0,
            units_per_em: 2048,
            font_size: 72.0,
            dots_per_inch: 144,
        },
        Sample {
            font_units: -123.34,
            pixel_rounded: -9.0,
            units_per_em: 1024,
            font_size: 6.5,
            dots_per_inch: 300,
        },
        Sample {
            font_units: 0.0,
            pixel_rounded: 0.0,
            units_per_em: 1000,
            font_size: 1.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.font_units,
            DistanceUnit::FontUnits {
                units_per_em: sample.units_per_em,
                font_size: sample.font_size,
            },
        );
        let result = distance.value(DistanceUnit::Pixel {
            dots_per_inch: sample.dots_per_inch,
        });
        assert_eq!(sample.pixel_rounded, result.round());
    }
}

#[test]
fn should_convert_font_units_to_points() {
    struct Sample {
        font_units: f64,
        points: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            font_units: 2016.0,
            points: 40.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
        Sample {
            font_units: 2334.0,
            points: 29.0,
            units_per_em: 2048,
            font_size: 9.0,
        },
        Sample {
            font_units: -6694.0,
            points: -123.0,
            units_per_em: 1000,
            font_size: 6.5,
        },
        Sample {
            font_units: 0.0,
            points: 0.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.font_units,
            DistanceUnit::FontUnits {
                units_per_em: sample.units_per_em,
                font_size: sample.font_size,
            },
        );
        let result = distance.value(DistanceUnit::Points);
        assert_eq!(sample.points, result.round());
    }
}
