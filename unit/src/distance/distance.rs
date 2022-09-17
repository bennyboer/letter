use crate::UnitValue;

use super::unit::DistanceUnit;

pub struct Distance {
    value: UnitValue,
    unit: DistanceUnit,
}

impl Distance {
    pub fn new(value: UnitValue, unit: DistanceUnit) -> Self {
        Self { value, unit }
    }

    pub fn value(&self, unit: DistanceUnit) -> UnitValue {
        let is_in_correct_unit = self.unit == unit;
        if is_in_correct_unit {
            return self.value;
        }

        let base_value = self.unit.to_base(self.value);
        return unit.from_base(base_value);
    }
}
