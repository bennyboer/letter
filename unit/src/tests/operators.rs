use crate::{Distance, DistanceUnit};

#[test]
fn should_add_two_distances() {
    let a = Distance::new(16.0, DistanceUnit::Points);
    let b = Distance::new(2.0, DistanceUnit::Centimeter);

    assert_eq!(25.644444444444446, (a + b).value(DistanceUnit::Millimeter));
}

#[test]
fn should_add_two_distances_via_assign() {
    let mut a = Distance::new(16.0, DistanceUnit::Points);
    let b = Distance::new(2.0, DistanceUnit::Centimeter);

    a += b;

    assert_eq!(25.644444444444446, a.value(DistanceUnit::Millimeter));
}

#[test]
fn should_subtract_two_distances() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(16.0, DistanceUnit::Points);

    assert_eq!(14.355555555555556, (a - b).value(DistanceUnit::Millimeter));
}

#[test]
fn should_subtract_two_distances_via_assign() {
    let mut a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(16.0, DistanceUnit::Points);

    a -= b;

    assert_eq!(25.644444444444442, a.value(DistanceUnit::Millimeter));
}

#[test]
fn should_multiply_two_distances() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(10.0, DistanceUnit::Millimeter);

    assert_eq!(20.0, (a * b).value(DistanceUnit::Centimeter));
}

#[test]
fn should_multiply_by_float() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);

    assert_eq!(4.0, (a * 2.0).value(DistanceUnit::Centimeter));
}

#[test]
fn should_divide_two_distances() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(5.0, DistanceUnit::Millimeter);

    assert_eq!(4.0, (a / b).value(DistanceUnit::Millimeter));
}

#[test]
fn should_divide_by_float() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);

    assert_eq!(1.0, (a / 2.0).value(DistanceUnit::Centimeter));
}

#[test]
fn should_compare_two_distances_by_greater_than() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(5.0, DistanceUnit::Millimeter);

    assert_eq!(true, a > b);
    assert_eq!(false, b > a);
    assert_eq!(false, a > a);
    assert_eq!(false, b > b);
}

#[test]
fn should_compare_two_distances_by_greater_or_equal_than() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(5.0, DistanceUnit::Millimeter);
    let c = Distance::new(2.0, DistanceUnit::Centimeter);
    let d = Distance::new(5.0, DistanceUnit::Millimeter);

    assert_eq!(true, a >= b);
    assert_eq!(false, b >= a);
    assert_eq!(true, a >= c);
    assert_eq!(true, b >= d);
}

#[test]
fn should_compare_two_distances_by_lower_than() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(5.0, DistanceUnit::Millimeter);

    assert_eq!(false, a < b);
    assert_eq!(true, b < a);
    assert_eq!(false, a < a);
    assert_eq!(false, b < b);
}

#[test]
fn should_compare_two_distances_by_lower_or_equal_than() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(5.0, DistanceUnit::Millimeter);
    let c = Distance::new(2.0, DistanceUnit::Centimeter);
    let d = Distance::new(5.0, DistanceUnit::Millimeter);

    assert_eq!(false, a <= b);
    assert_eq!(true, b <= a);
    assert_eq!(true, a <= c);
    assert_eq!(true, b <= d);
}

#[test]
fn should_compare_two_distances_for_equality() {
    let a = Distance::new(2.0, DistanceUnit::Centimeter);
    let b = Distance::new(5.0, DistanceUnit::Millimeter);
    let c = Distance::new(2.0, DistanceUnit::Centimeter);
    let d = Distance::new(5.0, DistanceUnit::Millimeter);

    assert_eq!(false, a == b);
    assert_eq!(false, b == a);
    assert_eq!(true, a == c);
    assert_eq!(true, c == a);
    assert_eq!(true, b == d);
    assert_eq!(true, d == b);
    assert_eq!(true, a != b);
    assert_eq!(true, b != a);
    assert_eq!(false, a != c);
    assert_eq!(false, c != a);
    assert_eq!(false, b != d);
    assert_eq!(false, d != b);
}
