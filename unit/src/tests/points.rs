use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_points_to_millimeter() {
    struct Sample {
        points: f64,
        millimeter: f64,
    }

    let samples = [
        Sample {
            points: 4.0,
            millimeter: 1.411111111111111,
        },
        Sample {
            points: 0.00043,
            millimeter: 0.00015169444444444442,
        },
        Sample {
            points: -12.334,
            millimeter: -4.351161111111111,
        },
        Sample {
            points: 0.0,
            millimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.points, DistanceUnit::Points);
        let result = distance.value(DistanceUnit::Millimeter);
        assert_eq!(sample.millimeter, result);
    }
}

#[test]
fn should_convert_points_to_centimeter() {
    struct Sample {
        points: f64,
        centimeter: f64,
    }

    let samples = [
        Sample {
            points: 40.0,
            centimeter: 1.411111111111111,
        },
        Sample {
            points: 0.0043,
            centimeter: 0.00015169444444444442,
        },
        Sample {
            points: -123.34,
            centimeter: -4.351161111111111,
        },
        Sample {
            points: 0.0,
            centimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.points, DistanceUnit::Points);
        let result = distance.value(DistanceUnit::Centimeter);
        assert_eq!(sample.centimeter, result);
    }
}

#[test]
fn should_convert_points_to_decimeter() {
    struct Sample {
        points: f64,
        decimeter: f64,
    }

    let samples = [
        Sample {
            points: 4.0,
            decimeter: 0.014111111111111109,
        },
        Sample {
            points: 0.043,
            decimeter: 0.00015169444444444442,
        },
        Sample {
            points: -12.334,
            decimeter: -0.043511611111111105,
        },
        Sample {
            points: 0.0,
            decimeter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.points, DistanceUnit::Points);
        let result = distance.value(DistanceUnit::Decimeter);
        assert_eq!(sample.decimeter, result);
    }
}

#[test]
fn should_convert_points_to_meter() {
    struct Sample {
        points: f64,
        meter: f64,
    }

    let samples = [
        Sample {
            points: 1574.8031496062993,
            meter: 0.5555555555555556,
        },
        Sample {
            points: 0.16929133858267717,
            meter: 5.972222222222222e-5,
        },
        Sample {
            points: -4855.905511811024,
            meter: -1.7130555555555553,
        },
        Sample {
            points: 0.0,
            meter: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.points, DistanceUnit::Points);
        let result = distance.value(DistanceUnit::Meter);
        assert_eq!(sample.meter, result);
    }
}

#[test]
fn should_convert_points_to_inch() {
    struct Sample {
        points: f64,
        inch: f64,
    }

    let samples = [
        Sample {
            points: 2880.0000000000005,
            inch: 40.00000000000001,
        },
        Sample {
            points: 0.30960000000000004,
            inch: 0.004300000000000001,
        },
        Sample {
            points: -8880.48,
            inch: -123.34,
        },
        Sample {
            points: 0.0,
            inch: 0.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.points, DistanceUnit::Points);
        let result = distance.value(DistanceUnit::Inch);
        assert_eq!(sample.inch, result);
    }
}

#[test]
fn should_convert_points_to_pixel() {
    struct Sample {
        points: f64,
        pixel_rounded: f64,
        dots_per_inch: usize,
    }

    let samples = [
        Sample {
            points: 40.0,
            pixel_rounded: 40.0,
            dots_per_inch: 72,
        },
        Sample {
            points: 0.0043,
            pixel_rounded: 0.0,
            dots_per_inch: 144,
        },
        Sample {
            points: -123.34,
            pixel_rounded: -514.0,
            dots_per_inch: 300,
        },
        Sample {
            points: 0.0,
            pixel_rounded: 0.0,
            dots_per_inch: 72,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.points, DistanceUnit::Points);
        let result = distance.value(DistanceUnit::Pixel {
            dots_per_inch: sample.dots_per_inch,
        });
        assert_eq!(sample.pixel_rounded, result.round());
    }
}

#[test]
fn should_convert_points_to_font_units() {
    struct Sample {
        points: f64,
        font_units_rounded: f64,
        units_per_em: usize,
        font_size: f64,
    }

    let samples = [
        Sample {
            points: 40.0,
            font_units_rounded: 2016.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
        Sample {
            points: 0.0043,
            font_units_rounded: 0.0,
            units_per_em: 2048,
            font_size: 9.0,
        },
        Sample {
            points: -123.34,
            font_units_rounded: -6694.0,
            units_per_em: 1000,
            font_size: 6.5,
        },
        Sample {
            points: 0.0,
            font_units_rounded: 0.0,
            units_per_em: 1000,
            font_size: 7.0,
        },
    ];

    for sample in samples {
        let distance = Distance::new(sample.points, DistanceUnit::Points);
        let result = distance.value(DistanceUnit::FontUnits {
            units_per_em: sample.units_per_em,
            font_size: sample.font_size,
        });
        assert_eq!(sample.font_units_rounded, result.round());
    }
}
