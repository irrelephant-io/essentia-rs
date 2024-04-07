use std::{iter::Sum, ops::{Add, AddAssign, Div, Mul, Sub, SubAssign}};

use super::{Power, TimeSpan};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Energy {
    pub joules: i32
}

impl Sum for Energy {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Energy { joules: iter.map(|e| e.joules).sum() }
    }
}

impl SubAssign for Energy {
    fn sub_assign(&mut self, rhs: Self) {
        self.joules -= rhs.joules;
    }
}

impl AddAssign for Energy {
    fn add_assign(&mut self, rhs: Self) {
        self.joules += rhs.joules;
    }
}

impl Energy {
    pub fn from(joules: i32) -> Self {
        Energy { joules }
    }
}

impl Add for Energy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { joules: self.joules + rhs.joules }
    }
}

impl Sub for Energy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { joules: self.joules - rhs.joules }
    }
}

impl Mul<i32> for Energy {
    type Output = Energy;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output { joules: self.joules * rhs }
    }
}

impl Mul<u32> for Energy {
    type Output = Energy;

    fn mul(self, rhs: u32) -> Self::Output {
        Self::Output { joules: self.joules * rhs as i32 }
    }
}

impl Div<TimeSpan> for Energy {
    type Output = Power;
    
    fn div(self, rhs: TimeSpan) -> Self::Output {
        Self::Output { mwatts: self.joules / rhs.ticks as i32 }
    }
}