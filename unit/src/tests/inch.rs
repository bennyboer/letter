use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_inch_to_millimeter() {
    struct Sample {
        inch: f64,
        millimeter: f64,
    }

    let samples = [
        Sample {
            inch: 4.0,
            millimeter: 101.6,
        },
        Sample {
            inch: 0.00043,
            millimeter: 0.010922,
        },
        Sample {
            inch: -12.334,
            millimeter: -313.2836,
        },
        Sample {
            inch: 0.0,
            millimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.inch, DistanceUnit::Inch);
        let result = distance.value(DistanceUnit::Millimeter);
        assert_eq!(sample.millimeter, result);
    }
}

#[test]
fn should_convert_inch_to_centimeter() {
    struct Sample {
        inch: f64,
        centimeter: f64,
    }

    let samples = [
        Sample {
            inch: 40.0,
            centimeter: 101.6,
        },
        Sample {
            inch: 0.0043,
            centimeter: 0.010922,
        },
        Sample {
            inch: -123.34,
            centimeter: -313.2836,
        },
        Sample {
            inch: 0.0,
            centimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.inch, DistanceUnit::Inch);
        let result = distance.value(DistanceUnit::Centimeter);
        assert_eq!(sample.centimeter, result);
    }
}

#[test]
fn should_convert_inch_to_decimeter() {
    struct Sample {
        inch: f64,
        decimeter: f64,
    }

    let samples = [
        Sample {
            inch: 4.0,
            decimeter: 1.016,
        },
        Sample {
            inch: 0.043,
            decimeter: 0.010921999999999998,
        },
        Sample {
            inch: -12.334,
            decimeter: -3.1328359999999997,
        },
        Sample {
            inch: 0.0,
            decimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.inch, DistanceUnit::Inch);
        let result = distance.value(DistanceUnit::Decimeter);
        assert_eq!(sample.decimeter, result);
    }
}

#[test]
fn should_convert_inch_to_meter() {
    struct Sample {
        inch: f64,
        meter: f64,
    }

    let samples = [
        Sample {
            inch: 1574.8031496062993,
            meter: 40.0,
        },
        Sample {
            inch: 0.16929133858267717,
            meter: 0.0043,
        },
        Sample {
            inch: -4855.905511811024,
            meter: -123.34,
        },
        Sample {
            inch: 0.0,
            meter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.inch, DistanceUnit::Inch);
        let result = distance.value(DistanceUnit::Meter);
        assert_eq!(sample.meter, result);
    }
}

#[test]
fn should_convert_inch_to_points() {
    struct Sample {
        inch: f64,
        points: f64,
    }

    let samples = [
        Sample {
            inch: 40.0,
            points: 2880.0000000000005,
        },
        Sample {
            inch: 0.0043,
            points: 0.30960000000000004,
        },
        Sample {
            inch: -123.34,
            points: -8880.48,
        },
        Sample {
            inch: 0.0,
            points: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.inch, DistanceUnit::Inch);
        let result = distance.value(DistanceUnit::Points);
        assert_eq!(sample.points, result);
    }
}

#[test]
fn should_convert_inch_to_pixel() {
    struct Sample {
        inch: f64,
        pixel_rounded: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            inch: 40.0,
            pixel_rounded: 2880.0,
            dots_per_inch: 72,
        },
        Sample {
            inch: 0.0043,
            pixel_rounded: 1.0,
            dots_per_inch: 144,
        },
        Sample {
            inch: -123.34,
            pixel_rounded: -37002.0,
            dots_per_inch: 300,
        },
        Sample {
            inch: 0.0,
            pixel_rounded: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.inch, DistanceUnit::Inch);
        let result = distance.value(DistanceUnit::Pixel {
            dots_per_inch: sample.dots_per_inch,
        });
        assert_eq!(sample.pixel_rounded, result.round());
    }
}

#[test]
fn should_convert_inch_to_font_units() {
    struct Sample {
        inch: f64,
        font_units_rounded: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            inch: 40.0,
            font_units_rounded: 145143.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
        Sample {
            inch: 0.0043,
            font_units_rounded: 25.0,
            units_per_em: 2048,
            font_size: 9.0,
        },
        Sample {
            inch: -123.34,
            font_units_rounded: -481975.0,
            units_per_em: 1000,
            font_size: 6.5,
        },
        Sample {
            inch: 0.0,
            font_units_rounded: 0.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.inch, DistanceUnit::Inch);
        let result = distance.value(DistanceUnit::FontUnits {
            units_per_em: sample.units_per_em,
            font_size: sample.font_size,
        });
        assert_eq!(sample.font_units_rounded, result.round());
    }
}
