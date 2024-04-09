use std::ops::{Add, Mul, Neg, Sub};

use super::{energy::Energy, Quantity, TimeSpan};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Power {
    pub mwatts: i32,
}

impl From<i32> for Power {
    fn from(value: i32) -> Self {
        Power { mwatts: value }
    }
}

impl Mul<Quantity> for Power {
    type Output = Power;

    fn mul(self, rhs: Quantity) -> Self::Output {
        Self::Output {
            mwatts: self.mwatts * rhs.mmol as i32,
        }
    }
}

impl Add for Power {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            mwatts: self.mwatts + rhs.mwatts,
        }
    }
}

impl Sub for Power {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            mwatts: self.mwatts - rhs.mwatts,
        }
    }
}

impl Neg for Power {
    type Output = Power;

    fn neg(self) -> Self::Output {
        Power {
            mwatts: -self.mwatts,
        }
    }
}

impl Mul<TimeSpan> for Power {
    type Output = Energy;

    fn mul(self, rhs: TimeSpan) -> Self::Output {
        Energy {
            joules: self.mwatts * rhs.ticks as i32,
        }
    }
}
