use std::iter::Sum;
use std::{fmt::Display, ops};

use crate::UnitValue;

use super::unit::DistanceUnit;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Distance {
    value: UnitValue,
    unit: DistanceUnit,
}

impl Distance {
    pub fn new(value: UnitValue, unit: DistanceUnit) -> Self {
        Self { value, unit }
    }

    pub fn zero() -> Self {
        Self {
            value: 0.0,
            unit: DistanceUnit::Millimeter,
        }
    }

    pub fn max() -> Distance {
        Distance {
            value: f64::MAX,
            unit: DistanceUnit::Millimeter,
        }
    }

    pub fn value(&self, unit: DistanceUnit) -> UnitValue {
        let is_in_correct_unit = self.unit == unit;
        if is_in_correct_unit {
            return self.value;
        }

        let base_value = self.unit.to_base(self.value);
        return unit.from_base(base_value);
    }

    pub fn formatted(&self, unit: DistanceUnit) -> String {
        let value = self.value(unit);
        let unit_shortform = unit.shortform();

        format!("{value} {unit_shortform}")
    }
}

impl ops::Add<Distance> for Distance {
    type Output = Distance;

    fn add(self, to_add: Distance) -> Distance {
        let base_value_left = self.value(DistanceUnit::Millimeter);
        let base_value_right = to_add.value(DistanceUnit::Millimeter);

        Distance::new(base_value_left + base_value_right, DistanceUnit::Millimeter)
    }
}

impl ops::AddAssign for Distance {
    fn add_assign(&mut self, to_add: Distance) {
        let base_value_left = self.value(self.unit);
        let base_value_right = to_add.value(self.unit);

        self.value = base_value_left + base_value_right;
    }
}

impl ops::Sub<Distance> for Distance {
    type Output = Distance;

    fn sub(self, to_subtract: Distance) -> Distance {
        let base_value_left = self.value(DistanceUnit::Millimeter);
        let base_value_right = to_subtract.value(DistanceUnit::Millimeter);

        Distance::new(base_value_left - base_value_right, DistanceUnit::Millimeter)
    }
}

impl ops::Neg for Distance {
    type Output = Distance;

    fn neg(self) -> Distance {
        Distance::zero() - self
    }
}

impl ops::SubAssign for Distance {
    fn sub_assign(&mut self, to_subtract: Distance) {
        let base_value_left = self.value(self.unit);
        let base_value_right = to_subtract.value(self.unit);

        self.value = base_value_left + base_value_right;
    }
}

impl ops::Mul<Distance> for Distance {
    type Output = Distance;

    fn mul(self, to_multiply: Distance) -> Distance {
        let base_value_left = self.value(DistanceUnit::Millimeter);
        let base_value_right = to_multiply.value(DistanceUnit::Millimeter);

        Distance::new(base_value_left * base_value_right, DistanceUnit::Millimeter)
    }
}

impl ops::Mul<f64> for Distance {
    type Output = Distance;

    fn mul(self, factor: f64) -> Distance {
        let base_value_left = self.value(DistanceUnit::Millimeter);

        Distance::new(base_value_left * factor, DistanceUnit::Millimeter)
    }
}

impl ops::Div<Distance> for Distance {
    type Output = Distance;

    fn div(self, to_divide: Distance) -> Distance {
        let base_value_left = self.value(DistanceUnit::Millimeter);
        let base_value_right = to_divide.value(DistanceUnit::Millimeter);

        Distance::new(base_value_left / base_value_right, DistanceUnit::Millimeter)
    }
}

impl ops::Div<f64> for Distance {
    type Output = Distance;

    fn div(self, divisor: f64) -> Distance {
        let base_value_left = self.value(DistanceUnit::Millimeter);

        Distance::new(base_value_left / divisor, DistanceUnit::Millimeter)
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left_base_value = self.value(DistanceUnit::Millimeter);
        let right_base_value = other.value(DistanceUnit::Millimeter);

        if left_base_value == right_base_value {
            return Some(std::cmp::Ordering::Equal);
        } else if left_base_value < right_base_value {
            return Some(std::cmp::Ordering::Less);
        } else {
            return Some(std::cmp::Ordering::Greater);
        }
    }
}

impl Sum for Distance {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Distance::zero();

        for item in iter {
            sum += item;
        }

        sum
    }
}

impl Display for Distance {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.formatted(self.unit))
    }
}
