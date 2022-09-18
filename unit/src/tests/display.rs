use crate::{Distance, DistanceUnit};

#[test]
fn should_display_with_default_unit() {
    let distance_a = Distance::new(12.5, DistanceUnit::Millimeter);
    let distance_b = Distance::new(12.5, DistanceUnit::Points);

    assert_eq!("12.5 mm", distance_a.to_string());
    assert_eq!("12.5 pt", distance_b.to_string());
}

#[test]
fn should_display_millimeter() {
    let distance = Distance::new(6.4632, DistanceUnit::Meter);

    assert_eq!("6463.2 mm", distance.formatted(DistanceUnit::Millimeter));
}

#[test]
fn should_display_centimeter() {
    let distance = Distance::new(6.46, DistanceUnit::Meter);

    assert_eq!("646 cm", distance.formatted(DistanceUnit::Centimeter));
}

#[test]
fn should_display_decimeter() {
    let distance = Distance::new(6.46, DistanceUnit::Meter);

    assert_eq!("64.6 dm", distance.formatted(DistanceUnit::Decimeter));
}

#[test]
fn should_display_meter() {
    let distance = Distance::new(273.2, DistanceUnit::Centimeter);

    assert_eq!("2.732 m", distance.formatted(DistanceUnit::Meter));
}

#[test]
fn should_display_inches() {
    let distance = Distance::new(273.2, DistanceUnit::Millimeter);

    assert_eq!(
        "10.755905511811024 in",
        distance.formatted(DistanceUnit::Inch)
    );
}

#[test]
fn should_display_points() {
    let distance = Distance::new(273.2, DistanceUnit::Millimeter);

    assert_eq!(
        "774.4251968503937 pt",
        distance.formatted(DistanceUnit::Points)
    );
}

#[test]
fn should_display_pixel() {
    let distance = Distance::new(5.23, DistanceUnit::Millimeter);

    assert_eq!(
        "14.825196850393704 px",
        distance.formatted(DistanceUnit::Pixel { dots_per_inch: 72 })
    );
}

#[test]
fn should_display_font_units() {
    let distance = Distance::new(273.2, DistanceUnit::Millimeter);

    assert_eq!(
        "68300 FUnits",
        distance.formatted(DistanceUnit::FontUnits {
            units_per_em: 1000,
            font_size: 4.0
        })
    );
}
