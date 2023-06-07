use num::Zero;
use std::fmt::{Display, Formatter, Result};
use std::ops::RangeInclusive;

use crate::distance::{Int, Real, RealInterval, Rounding};

#[derive(Clone, Debug)]
pub struct Unit {
    name: String,
    many: Real,
    of: Real,
}

impl Unit {
    pub fn new(name: &str, many: &Real, of: &Real) -> Self {
        Self {
            name: name.to_owned(),
            many: many.to_owned(),
            of: of.to_owned(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn many(&self) -> &Real {
        &self.many
    }

    pub fn of(&self) -> &Real {
        &self.of
    }

    pub fn size(&self) -> Real {
        self.many * self.of
    }

    // Given a half-open interval over the reals, return a range of possible counts for this unit.
    pub fn count_range(&self, interval: &RealInterval, rounding: Rounding) -> RangeInclusive<Int> {
        assert!(interval.to_owned().size() > Real::zero());
        let reals = interval.to_owned() / self.size() - rounding.offsets();
        reals.0.ceil().to_integer()..=reals.1.floor().to_integer()
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}
