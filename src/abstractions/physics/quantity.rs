use std::{iter::Sum, ops::{Add, AddAssign, Mul, Sub, SubAssign}};

use super::TimeSpan;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity {
    pub mol: u32
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rate {
    pub mol_per_tick: u32
}

impl Default for Rate {
    fn default() -> Self {
        Self { mol_per_tick: 1 }
    }
}

impl From<u32> for Rate {
    fn from(value: u32) -> Self {
        Rate { mol_per_tick: value }
    }
}

impl Mul<TimeSpan> for Rate {
    type Output = Quantity;

    fn mul(self, rhs: TimeSpan) -> Self::Output {
        Quantity { mol: self.mol_per_tick * rhs.ticks }
    }
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

impl From<u32> for Quantity {
    fn from(value: u32) -> Self {
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
        Quantity { mol: (self.mol as f32 * rhs) as u32 }
    }
}