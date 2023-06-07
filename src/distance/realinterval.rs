use min_max::{max, min};
use num::Zero;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};

use crate::distance::{Int, Real};

// An interval of Reals. This is typically half-open, but we are sometimes sloppy since we usually care only about their size. Treating them
// as closed is okay since the mistake does not lead us astray with this main.rs.
#[derive(Clone, Debug)]
pub struct RealInterval(pub Real, pub Real); // Requires interval.0 <= interval.1 .

impl RealInterval {
    pub fn new(lo: &Real, hi: &Real) -> Self {
        assert!(lo <= hi);
        Self(lo.to_owned(), hi.to_owned())
    }

    pub fn size(&self) -> Real {
        self.1 - self.0
    }

    pub fn intersection_with(&self, other: &Self) -> Self {
        let (lo, hi) = (max(self.0, other.0), min(self.1, other.1));
        Self(lo, max(lo, hi)) // May be empty
    }
}

impl Add<Int> for RealInterval {
    type Output = Self;
    fn add(self, rhs: Int) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

// TODO: This creates an open interval; fix this?
impl Sub<Self> for RealInterval {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.1, self.1 - rhs.0)
    }
}

impl Mul<Real> for RealInterval {
    type Output = Self;
    fn mul(self, rhs: Real) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<Real> for RealInterval {
    type Output = Self;
    fn div(self, rhs: Real) -> Self::Output {
        assert_ne!(rhs, Real::zero());
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Display for RealInterval {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}+[0,{})", self.0, self.1 - self.0)
    }
}
