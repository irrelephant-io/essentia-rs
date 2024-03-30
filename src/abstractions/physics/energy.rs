use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Energy {
    pub joules: i16
}

impl Energy {
    pub fn from(joules: i16) -> Self {
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