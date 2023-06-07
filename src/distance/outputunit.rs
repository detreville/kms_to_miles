use enum_iterator::Sequence;
use num::One;
use std::fmt::{Display, Formatter, Result};

use crate::distance::{Real, Unit};

#[derive(Clone, Copy, Debug, Sequence)]
pub enum OutputUnit {
    Mile(),
    HalfMile(),
    QuarterMile(),
    EighthMile(),
}

impl OutputUnit {
    pub fn unit(&self) -> Unit {
        let one_mile = &(1609344, 1000000).into(); // One mile, in kms.
        match *self {
            Self::Mile() => Unit::new("mile", &Real::one(), one_mile),
            Self::HalfMile() => Unit::new("half-mile", &(1, 2).into(), one_mile),
            Self::QuarterMile() => Unit::new("quarter-mile", &(1, 4).into(), one_mile),
            Self::EighthMile() => Unit::new("eighth-mile", &(1, 8).into(), one_mile),
        }
    }
}

impl Display for OutputUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.unit().name())
    }
}
