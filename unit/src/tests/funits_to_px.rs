use crate::{Distance, DistanceUnit};

#[test]
fn should_convert_font_units_to_px() {
    struct ConversionSample {
        /// Units per em is usually given by the font you are working with
        /// and defines the font-internal coordinate system.
        units_per_em: usize,
        /// Font size in millimeters.
        font_size_mm: f64,
        /// Dots per inch of the monitor.
        dots_per_inch: usize,
        font_units_to_convert: f64,
        expected_px_rounded: f64,
    }

    // Given: some conversion samples from font units to px
    let samples = [
        ConversionSample {
            units_per_em: 1000,
            font_size_mm: 7.0,
            dots_per_inch: 72,
            font_units_to_convert: 800.0,
            expected_px_rounded: 16.0,
        },
        ConversionSample {
            units_per_em: 2048,
            font_size_mm: 9.0,
            dots_per_inch: 300,
            font_units_to_convert: 2450.0,
            expected_px_rounded: 127.0,
        },
    ];

    for sample in samples {
        let font_units = Distance::new(
            sample.font_units_to_convert,
            DistanceUnit::FontUnits {
                units_per_em: sample.units_per_em,
                font_size: sample.font_size_mm,
            },
        );
        let pixel = font_units.value(DistanceUnit::Pixel {
            dots_per_inch: sample.dots_per_inch,
        });

        assert_eq!(sample.expected_px_rounded, pixel.round());
    }
}
