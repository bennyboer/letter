use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_meter_to_millimeter() {
    struct Sample {
        meter: f64,
        millimeter: f64,
    }

    let samples = [
        Sample {
            meter: 4.0,
            millimeter: 4000.0,
        },
        Sample {
            meter: 0.00043,
            millimeter: 0.43,
        },
        Sample {
            meter: -12.334,
            millimeter: -12334.0,
        },
        Sample {
            meter: 0.0,
            millimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.meter, DistanceUnit::Meter);
        let result = distance.value(DistanceUnit::Millimeter);
        assert_eq!(sample.millimeter, result);
    }
}

#[test]
fn should_convert_meter_to_centimeter() {
    struct Sample {
        meter: f64,
        centimeter: f64,
    }

    let samples = [
        Sample {
            meter: 40.0,
            centimeter: 4000.0,
        },
        Sample {
            meter: 0.0043,
            centimeter: 0.43,
        },
        Sample {
            meter: -123.34,
            centimeter: -12334.0,
        },
        Sample {
            meter: 0.0,
            centimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.meter, DistanceUnit::Meter);
        let result = distance.value(DistanceUnit::Centimeter);
        assert_eq!(sample.centimeter, result);
    }
}

#[test]
fn should_convert_meter_to_decimeter() {
    struct Sample {
        meter: f64,
        decimeter: f64,
    }

    let samples = [
        Sample {
            meter: 4.0,
            decimeter: 40.0,
        },
        Sample {
            meter: 0.043,
            decimeter: 0.43,
        },
        Sample {
            meter: -12.334,
            decimeter: -123.34,
        },
        Sample {
            meter: 0.0,
            decimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.meter, DistanceUnit::Meter);
        let result = distance.value(DistanceUnit::Decimeter);
        assert_eq!(sample.decimeter, result);
    }
}

#[test]
fn should_convert_meter_to_inches() {
    struct Sample {
        meter: f64,
        inches: f64,
    }

    let samples = [
        Sample {
            meter: 40.0,
            inches: 1574.8031496062993,
        },
        Sample {
            meter: 0.0043,
            inches: 0.16929133858267717,
        },
        Sample {
            meter: -123.34,
            inches: -4855.905511811024,
        },
        Sample {
            meter: 0.0,
            inches: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.meter, DistanceUnit::Meter);
        let result = distance.value(DistanceUnit::Inch);
        assert_eq!(sample.inches, result);
    }
}

#[test]
fn should_convert_meter_to_points() {
    struct Sample {
        meter: f64,
        points: f64,
    }

    let samples = [
        Sample {
            meter: 40.0,
            points: 113385.82677165355,
        },
        Sample {
            meter: 0.0043,
            points: 12.188976377952757,
        },
        Sample {
            meter: -123.34,
            points: -349625.1968503937,
        },
        Sample {
            meter: 0.0,
            points: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.meter, DistanceUnit::Meter);
        let result = distance.value(DistanceUnit::Points);
        assert_eq!(sample.points, result);
    }
}

#[test]
fn should_convert_meter_to_pixel() {
    struct Sample {
        meter: f64,
        pixel_rounded: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            meter: 40.0,
            pixel_rounded: 113386.0,
            dots_per_inch: 72,
        },
        Sample {
            meter: 0.0043,
            pixel_rounded: 24.0,
            dots_per_inch: 144,
        },
        Sample {
            meter: -123.34,
            pixel_rounded: -1456772.0,
            dots_per_inch: 300,
        },
        Sample {
            meter: 0.0,
            pixel_rounded: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.meter, DistanceUnit::Meter);
        let result = distance.value(DistanceUnit::Pixel {
            dots_per_inch: sample.dots_per_inch,
        });
        assert_eq!(sample.pixel_rounded, result.round());
    }
}

#[test]
fn should_convert_meter_to_font_units() {
    struct Sample {
        meter: f64,
        font_units_rounded: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            meter: 40.0,
            font_units_rounded: 5714286.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
        Sample {
            meter: 0.0043,
            font_units_rounded: 978.0,
            units_per_em: 2048,
            font_size: 9.0,
        },
        Sample {
            meter: -123.34,
            font_units_rounded: -18975385.0,
            units_per_em: 1000,
            font_size: 6.5,
        },
        Sample {
            meter: 0.0,
            font_units_rounded: 0.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.meter, DistanceUnit::Meter);
        let result = distance.value(DistanceUnit::FontUnits {
            units_per_em: sample.units_per_em,
            font_size: sample.font_size,
        });
        assert_eq!(sample.font_units_rounded, result.round());
    }
}
