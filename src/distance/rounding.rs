use num::{One, Zero};
use std::fmt::{Display, Formatter, Result};

use crate::distance::{Real, RealInterval};

// A Rounding is a rounding mode already applied to an input, or to be applied (by a human) to an output.
#[derive(Clone, Copy, Debug)]
pub enum Rounding {
    Truncated,
    WillRound,
}

impl Rounding {
    pub fn offsets(&self) -> RealInterval {
        match *self {
            Self::Truncated => RealInterval::new(&Real::zero(), &Real::one()),
            Self::WillRound => RealInterval::new(&(-1, 2).into(), &(1, 2).into()),
        }
    }
}

impl Display for Rounding {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Truncated => "Truncated",
                Self::WillRound => "WillRound",
            }
        )
    }
}
