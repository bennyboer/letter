use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_pixel_to_millimeter() {
    struct Sample {
        pixel: f64,
        millimeter: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            pixel: 230.0,
            millimeter: 81.13888888888889,
            dots_per_inch: 72,
        },
        Sample {
            pixel: 0.00043,
            millimeter: 7.584722222222221e-5,
            dots_per_inch: 144,
        },
        Sample {
            pixel: -12.334,
            millimeter: -1.0442786666666666,
            dots_per_inch: 300,
        },
        Sample {
            pixel: 0.0,
            millimeter: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.pixel,
            DistanceUnit::Pixel {
                dots_per_inch: sample.dots_per_inch,
            },
        );
        let result = distance.value(DistanceUnit::Millimeter);
        assert_eq!(sample.millimeter, result);
    }
}

#[test]
fn should_convert_pixel_to_centimeter() {
    struct Sample {
        pixel: f64,
        centimeter: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            pixel: 423.0,
            centimeter: 14.9225,
            dots_per_inch: 72,
        },
        Sample {
            pixel: 0.0043,
            centimeter: 7.584722222222221e-5,
            dots_per_inch: 144,
        },
        Sample {
            pixel: -123.34,
            centimeter: -1.0442786666666666,
            dots_per_inch: 300,
        },
        Sample {
            pixel: 0.0,
            centimeter: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.pixel,
            DistanceUnit::Pixel {
                dots_per_inch: sample.dots_per_inch,
            },
        );
        let result = distance.value(DistanceUnit::Centimeter);
        assert_eq!(sample.centimeter, result);
    }
}

#[test]
fn should_convert_pixel_to_decimeter() {
    struct Sample {
        pixel: f64,
        decimeter: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            pixel: 2348.0,
            decimeter: 8.283222222222221,
            dots_per_inch: 72,
        },
        Sample {
            pixel: 0.043,
            decimeter: 7.584722222222221e-5,
            dots_per_inch: 144,
        },
        Sample {
            pixel: -12.334,
            decimeter: -0.010442786666666665,
            dots_per_inch: 300,
        },
        Sample {
            pixel: 0.0,
            decimeter: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.pixel,
            DistanceUnit::Pixel {
                dots_per_inch: sample.dots_per_inch,
            },
        );
        let result = distance.value(DistanceUnit::Decimeter);
        assert_eq!(sample.decimeter, result);
    }
}

#[test]
fn should_convert_pixel_to_meter() {
    struct Sample {
        pixel: f64,
        meter: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            pixel: 13574.8031496062993,
            meter: 4.788888888888889,
            dots_per_inch: 72,
        },
        Sample {
            pixel: 0.16929133858267717,
            meter: 2.986111111111111e-5,
            dots_per_inch: 144,
        },
        Sample {
            pixel: -4855.905511811024,
            meter: -0.41113333333333335,
            dots_per_inch: 300,
        },
        Sample {
            pixel: 0.0,
            meter: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.pixel,
            DistanceUnit::Pixel {
                dots_per_inch: sample.dots_per_inch,
            },
        );
        let result = distance.value(DistanceUnit::Meter);
        assert_eq!(sample.meter, result);
    }
}

#[test]
fn should_convert_pixel_to_inch() {
    struct Sample {
        pixel: f64,
        inch: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            pixel: 2880.0000000000005,
            inch: 40.00000000000001,
            dots_per_inch: 72,
        },
        Sample {
            pixel: 0.30960000000000004,
            inch: 0.0021500000000000004,
            dots_per_inch: 144,
        },
        Sample {
            pixel: -8880.48,
            inch: -29.6016,
            dots_per_inch: 300,
        },
        Sample {
            pixel: 0.0,
            inch: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.pixel,
            DistanceUnit::Pixel {
                dots_per_inch: sample.dots_per_inch,
            },
        );
        let result = distance.value(DistanceUnit::Inch);
        assert_eq!(sample.inch, result);
    }
}

#[test]
fn should_convert_pixel_to_points() {
    struct Sample {
        pixel: f64,
        points: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            pixel: 40.0,
            points: 40.0,
            dots_per_inch: 72,
        },
        Sample {
            pixel: 1238.234,
            points: 619.0,
            dots_per_inch: 144,
        },
        Sample {
            pixel: -514.0,
            points: -123.0,
            dots_per_inch: 300,
        },
        Sample {
            pixel: 0.0,
            points: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.pixel,
            DistanceUnit::Pixel {
                dots_per_inch: sample.dots_per_inch,
            },
        );
        let result = distance.value(DistanceUnit::Points);
        assert_eq!(sample.points, result.round());
    }
}

#[test]
fn should_convert_pixel_to_font_units() {
    struct Sample {
        pixel: f64,
        font_units_rounded: f64,
        dots_per_inch: usize,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            pixel: 40.0,
            font_units_rounded: 2016.0,
            dots_per_inch: 72,
            units_per_em: 1000,
            font_size: 7.0,
        },
        Sample {
            pixel: 0.0043,
            font_units_rounded: 0.0,
            dots_per_inch: 144,
            units_per_em: 2048,
            font_size: 9.0,
        },
        Sample {
            pixel: -123.34,
            font_units_rounded: -1607.0,
            dots_per_inch: 300,
            units_per_em: 1000,
            font_size: 6.5,
        },
        Sample {
            pixel: 0.0,
            font_units_rounded: 0.0,
            dots_per_inch: 72,
            units_per_em: 1000,
            font_size: 7.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(
            sample.pixel,
            DistanceUnit::Pixel {
                dots_per_inch: sample.dots_per_inch,
            },
        );
        let result = distance.value(DistanceUnit::FontUnits {
            units_per_em: sample.units_per_em,
            font_size: sample.font_size,
        });
        assert_eq!(sample.font_units_rounded, result.round());
    }
}
