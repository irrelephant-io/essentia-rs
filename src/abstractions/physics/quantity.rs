use std::{iter::Sum, ops::{Add, AddAssign, Mul, Sub, SubAssign}};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity {
    pub mol: u16
}

impl SubAssign for Quantity {
    fn sub_assign(&mut self, rhs: Self) {
        self.mol -= rhs.mol;
    }
}

impl AddAssign for Quantity {
    fn add_assign(&mut self, rhs: Self) {
        self.mol += rhs.mol;
    }
}


impl Quantity {
    pub fn none() -> Self {
        Quantity { mol: 0 }
    }
}

impl Default for Quantity {
    fn default() -> Self {
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

impl Sum for Quantity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Quantity { mol: iter.map(|q| q.mol).sum() }
    }
}

impl Mul<f32> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: f32) -> Self::Output {
        Quantity { mol: (self.mol as f32 * rhs) as u16 }
    }
}