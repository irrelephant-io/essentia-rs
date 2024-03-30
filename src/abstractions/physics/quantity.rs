use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity {
    pub mol: u16
}

impl Quantity {
    pub fn default() -> Self {
        Quantity { mol: 1 }
    }
}

impl From<u16> for Quantity {
    fn from(value: u16) -> Self {
        Quantity { mol: value }
    }
}

impl Add for Quantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Quantity {
            mol: self.mol + rhs.mol
        }
    }
}

impl Sub for Quantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Quantity {
            mol: self.mol - rhs.mol
        }
    }
}