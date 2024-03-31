use std::ops::{Add, Mul, Neg, Sub};

use super::{Energy, Quantity, TimeSpan};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Power {
    pub watts: i16
}

impl From<i16> for Power {
    fn from(value: i16) -> Self {
        Power { watts: value }
    }
}

impl Mul<Quantity> for Power {
    type Output = Power;

    fn mul(self, rhs: Quantity) -> Self::Output {
        Self::Output { watts: self.watts * rhs.mol as i16 }
    }
}

impl Add for Power {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            watts: self.watts + rhs.watts
        }
    }
}

impl Sub for Power {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            watts: self.watts - rhs.watts
        }
    }
}

impl Neg for Power {
    type Output = Power;

    fn neg(self) -> Self::Output {
        Power { watts: -self.watts }
    }
}

impl Mul<TimeSpan> for Power {
    type Output = Energy;

    fn mul(self, rhs: TimeSpan) -> Self::Output {
        Energy { joules: self.watts * rhs.ticks as i16 }
    }
}