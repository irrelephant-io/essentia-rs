use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Power {
    pub watts: i16
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