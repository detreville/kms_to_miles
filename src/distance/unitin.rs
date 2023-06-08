use num::One;
use std::fmt::{Display, Formatter, Result};

use crate::distance::{Real, Unit};

#[derive(Clone, Copy, Debug)]
pub enum InputUnit {
    Kms(),
    TenthKms(),
}

impl InputUnit {
    pub fn unit(&self) -> Unit {
        let one_km = &Real::one();
        match *self {
            Self::Kms() => Unit::new("kms", &Real::one(), one_km),
            Self::TenthKms() => Unit::new("tenth-km", &(1, 10).into(), one_km),
        }
    }
}

impl Display for InputUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.unit().name())
    }
}
