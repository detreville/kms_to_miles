use num::Zero;
use std::fmt::{Display, Formatter, Result};
use std::ops::RangeInclusive;

use crate::distance::{Int, Real, RealInterval, Rounding};

#[derive(Clone, Debug)]
pub struct Unit {
    name: String,
    count: Real,
    base: Real,
}

impl Unit {
    pub fn new(name: &str, count: &Real, base: &Real) -> Self {
        Self {
            name: name.to_owned(),
            count: count.to_owned(),
            base: base.to_owned(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn count(&self) -> &Real {
        &self.count
    }

    pub fn base(&self) -> &Real {
        &self.base
    }

    pub fn distance(&self) -> Real {
        self.count * self.base
    }

    // Given a half-open interval over the reals, return a range of possible counts for this unit.
    pub fn count_range(&self, interval: &RealInterval, rounding: Rounding) -> RangeInclusive<Int> {
        assert!(interval.size() > Real::zero());
        let reals = interval.to_owned() / self.distance() - rounding.offsets();
        reals.0.ceil().to_integer()..=reals.1.floor().to_integer()
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}
