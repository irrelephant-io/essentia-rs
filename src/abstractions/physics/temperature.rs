use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct Temperature {
    pub degrees: i32
}

impl Default for Temperature {
    fn default() -> Self {
        Self { degrees: 20 }
    }
}

impl From<i32> for Temperature {
    fn from(value: i32) -> Self {
        Temperature { degrees: value }
    }
}

impl Add for Temperature {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            degrees: self.degrees + rhs.degrees
        }
    }
}

impl AddAssign for Temperature {
    fn add_assign(&mut self, rhs: Self) {
        self.degrees += rhs.degrees;
    }
}

impl Sub for Temperature {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            degrees: self.degrees - rhs.degrees
        }
    }
}