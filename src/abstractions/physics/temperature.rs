use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct Temperature {
    pub mkelvin: i32
}

impl Default for Temperature {
    fn default() -> Self {
        Self { mkelvin: 293_000 }
    }
}

impl From<i32> for Temperature {
    fn from(value: i32) -> Self {
        Temperature { mkelvin: value }
    }
}

impl Add for Temperature {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            mkelvin: self.mkelvin + rhs.mkelvin
        }
    }
}

impl AddAssign for Temperature {
    fn add_assign(&mut self, rhs: Self) {
        self.mkelvin += rhs.mkelvin;
    }
}

impl Sub for Temperature {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            mkelvin: self.mkelvin - rhs.mkelvin
        }
    }
}