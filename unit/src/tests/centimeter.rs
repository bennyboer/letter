use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_centimeter_to_millimeter() {
    struct Sample {
        centimeter: f64,
        millimeter: f64,
    }

    let samples = [
        Sample {
            centimeter: 4.0,
            millimeter: 40.0,
        },
        Sample {
            centimeter: 0.00043,
            millimeter: 0.0043,
        },
        Sample {
            centimeter: -12.334,
            millimeter: -123.34,
        },
        Sample {
            centimeter: 0.0,
            millimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.centimeter, DistanceUnit::Centimeter);
        let result = distance.value(DistanceUnit::Millimeter);
        assert_eq!(sample.millimeter, result);
    }
}

#[test]
fn should_convert_centimeter_to_decimeter() {
    struct Sample {
        centimeter: f64,
        decimeter: f64,
    }

    let samples = [
        Sample {
            centimeter: 40.0,
            decimeter: 4.0,
        },
        Sample {
            centimeter: 0.0043,
            decimeter: 0.00043,
        },
        Sample {
            centimeter: -123.34,
            decimeter: -12.334000000000001,
        },
        Sample {
            centimeter: 0.0,
            decimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.centimeter, DistanceUnit::Centimeter);
        let result = distance.value(DistanceUnit::Decimeter);
        assert_eq!(sample.decimeter, result);
    }
}

#[test]
fn should_convert_centimeter_to_meter() {
    struct Sample {
        centimeter: f64,
        meter: f64,
    }

    let samples = [
        Sample {
            centimeter: 40.0,
            meter: 0.4,
        },
        Sample {
            centimeter: 0.0043,
            meter: 4.2999999999999995e-5,
        },
        Sample {
            centimeter: -123.34,
            meter: -1.2334,
        },
        Sample {
            centimeter: 0.0,
            meter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.centimeter, DistanceUnit::Centimeter);
        let result = distance.value(DistanceUnit::Meter);
        assert_eq!(sample.meter, result);
    }
}

#[test]
fn should_convert_centimeter_to_inches() {
    struct Sample {
        centimeter: f64,
        inches: f64,
    }

    let samples = [
        Sample {
            centimeter: 40.0,
            inches: 15.748031496062993,
        },
        Sample {
            centimeter: 0.0043,
            inches: 0.0016929133858267717,
        },
        Sample {
            centimeter: -123.34,
            inches: -48.559055118110244,
        },
        Sample {
            centimeter: 0.0,
            inches: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.centimeter, DistanceUnit::Centimeter);
        let result = distance.value(DistanceUnit::Inch);
        assert_eq!(sample.inches, result);
    }
}

#[test]
fn should_convert_centimeter_to_points() {
    struct Sample {
        centimeter: f64,
        points: f64,
    }

    let samples = [
        Sample {
            centimeter: 40.0,
            points: 1133.8582677165355,
        },
        Sample {
            centimeter: 0.0043,
            points: 0.12188976377952757,
        },
        Sample {
            centimeter: -123.34,
            points: -3496.2519685039374,
        },
        Sample {
            centimeter: 0.0,
            points: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.centimeter, DistanceUnit::Centimeter);
        let result = distance.value(DistanceUnit::Points);
        assert_eq!(sample.points, result);
    }
}

#[test]
fn should_convert_centimeter_to_pixel() {
    struct Sample {
        centimeter: f64,
        pixel_rounded: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            centimeter: 40.0,
            pixel_rounded: 1134.0,
            dots_per_inch: 72,
        },
        Sample {
            centimeter: 0.0043,
            pixel_rounded: 0.0,
            dots_per_inch: 144,
        },
        Sample {
            centimeter: -123.34,
            pixel_rounded: -14568.0,
            dots_per_inch: 300,
        },
        Sample {
            centimeter: 0.0,
            pixel_rounded: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.centimeter, DistanceUnit::Centimeter);
        let result = distance.value(DistanceUnit::Pixel {
            dots_per_inch: sample.dots_per_inch,
        });
        assert_eq!(sample.pixel_rounded, result.round());
    }
}

#[test]
fn should_convert_centimeter_to_font_units() {
    struct Sample {
        centimeter: f64,
        font_units_rounded: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            centimeter: 40.0,
            font_units_rounded: 57143.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
        Sample {
            centimeter: 0.0043,
            font_units_rounded: 10.0,
            units_per_em: 2048,
            font_size: 9.0,
        },
        Sample {
            centimeter: -123.34,
            font_units_rounded: -189754.0,
            units_per_em: 1000,
            font_size: 6.5,
        },
        Sample {
            centimeter: 0.0,
            font_units_rounded: 0.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.centimeter, DistanceUnit::Centimeter);
        let result = distance.value(DistanceUnit::FontUnits {
            units_per_em: sample.units_per_em,
            font_size: sample.font_size,
        });
        assert_eq!(sample.font_units_rounded, result.round());
    }
}
