use std::fmt::{Display, Formatter, Result};

use crate::distance::{Int, Real, RealInterval, Rounding, Unit};

// An Approximation represents some multiple of a fixed unit, approximating a given interval.
#[derive(Clone, Debug)]
pub struct Approximation {
    count: Int, // No need to reference-count since always computed fresh.
    unit: Unit,
    rounding: Rounding,
}

impl Approximation {
    pub fn new(count: Int, unit: &Unit, rounding: Rounding) -> Self {
        Self {
            count,
            unit: unit.to_owned(),
            rounding,
        }
    }

    pub fn many(&self) -> Real {
        self.unit.count().to_owned() * self.count
    }

    pub fn distance(&self) -> Real {
        self.many() * *self.unit().base()
    }

    pub fn unit(&self) -> Unit {
        self.unit.to_owned()
    }

    pub fn interval(&self) -> RealInterval {
        (self.rounding.offsets() + self.count) * self.unit.distance()
    }
}

impl Display for Approximation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Approximation {{ count: {}, unit: {}, interval: {}, {} }}",
            self.count,
            self.unit,
            self.interval(),
            self.rounding
        )
    }
}
