use num::rational::Ratio;
use num::{One, Zero};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Deref, Div, Mul, Sub};

use crate::distance::Int;

type Raw = Ratio<Int>;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Real(Raw);

impl Add<Real> for Real {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + *other)
    }
}

impl Add<Int> for Real {
    type Output = Self;

    fn add(self, other: Int) -> Self {
        Self(self.0 + other)
    }
}

impl Deref for Real {
    type Target = Raw;

    fn deref(&self) -> &Raw {
        &self.0
    }
}

impl Display for Real {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // const FRACTION_NUMERATOR_ONE: char = '\u{215F}'; // Not for debugging to console, which looks awful.
        const FRACTION_SLASH: char = '\u{2044}';
        const INVISIBLE_PLUS: char = '\u{2064}';
        const MINUS_SIGN: char = '\u{2212}';

        thread_local! {
            static VULGATES: HashMap<Raw, char> = {
                let mut vulgates: HashMap<Raw, char> = HashMap::new();
                 [
                    ((1, 10), '⅒'), ((1, 9), '⅑'), ((1, 8), '⅛'), ((1, 7), '⅐'), ((1, 6), '⅙'), ((1, 5), '⅕'),
                    ((1, 4), '¼'), ((1, 3), '⅓'), ((3, 8), '⅜'), ((2, 5), '⅖'), ((1, 2), '½'), ((3, 5), '⅗'),
                    ((5, 8), '⅝'), ((2, 3), '⅔'), ((3, 4), '¾'), ((4, 5), '⅘'), ((5, 6), '⅚'), ((7, 8), '⅞'),
                 ].into_iter().for_each(|((numer, denom), vulgate)| {
                    vulgates.insert(Raw::new(numer, denom), vulgate);
                });
                vulgates
            }
        }

        let raw = if self >= &Real::zero() {
            self.0
        } else {
            write!(f, "{}", MINUS_SIGN)?;
            -self.0
        };
        if raw.is_integer() {
            return write!(f, "{}", raw);
        }

        let integer = raw.floor();
        let fraction = raw - integer;
        if !integer.is_zero() {
            write!(f, "{}{}", integer, INVISIBLE_PLUS)?; // A non-zero integer part precedes the fraction part.
        }
        VULGATES.with(|vulgates| match vulgates.get(&fraction) {
            Some(vulgate) => write!(f, "{}", vulgate),
            None => {
                write!(
                    f,
                    "{}{FRACTION_SLASH}{}",
                    fraction.numer(),
                    fraction.denom()
                )
            }
        })
    }
}

impl Div<Real> for Real {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / *other)
    }
}

impl Eq for Real {
    fn assert_receiver_is_total_eq(&self) {
        self.0.assert_receiver_is_total_eq()
    }
}

impl From<Raw> for Real {
    fn from(x: Raw) -> Self {
        Self(x)
    }
}

impl From<(Int, Int)> for Real {
    fn from(pair: (Int, Int)) -> Self {
        Self(Raw::new(pair.0, pair.1))
    }
}

impl From<Int> for Real {
    fn from(x: Int) -> Self {
        Self(x.into())
    }
}

impl Hash for Real {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) {
        for x in data {
            x.0.hash(state);
        }
    }
}

impl Mul<Real> for Real {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * *other)
    }
}

impl Mul<Int> for Real {
    type Output = Self;

    fn mul(self, other: Int) -> Self {
        Self(self.0 * other)
    }
}

impl One for Real {
    fn one() -> Self {
        Self(Ratio::new(1, 1))
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }

    fn set_one(&mut self) {
        self.0.set_one()
    }
}

impl Sub<Real> for Real {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - *other)
    }
}

impl Sub<Int> for Real {
    type Output = Self;

    fn sub(self, other: Int) -> Self {
        Self(self.0 - other)
    }
}

impl Zero for Real {
    fn zero() -> Self {
        Self(Raw::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn set_zero(&mut self) {
        self.0.set_zero()
    }
}
