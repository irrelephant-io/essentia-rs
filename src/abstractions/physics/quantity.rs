use std::{iter::Sum, ops::{Add, AddAssign, Div, Mul, Sub, SubAssign}};

use super::TimeSpan;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity {
    pub mmol: u32
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rate {
    pub mmol_per_tick: u32
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PerMol {
    pub mmol_per: u32
}

impl Default for PerMol {
    fn default() -> Self {
        Self { mmol_per: 1 }
    }
}

impl Mul<Quantity> for PerMol {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Self::Output {
        Self::Output::from(self.mmol_per * rhs.mmol)
    }
}

impl Mul<PerMol> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: PerMol) -> Self::Output {
        Self::Output::from(self.mmol * rhs.mmol_per)
    }
}

impl Default for Rate {
    fn default() -> Self {
        Self { mmol_per_tick: 1000 }
    }
}

impl From<u32> for Rate {
    fn from(value: u32) -> Self {
        Rate { mmol_per_tick: value }
    }
}

impl Mul<TimeSpan> for Rate {
    type Output = Quantity;

    fn mul(self, rhs: TimeSpan) -> Self::Output {
        Quantity { mmol: self.mmol_per_tick * rhs.ticks }
    }
}

impl Mul<u32> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: u32) -> Self::Output {
        Quantity { mmol: self.mmol * rhs as u32 }
    }
}

impl Div<u8> for Quantity {
    type Output = Quantity;

    fn div(self, rhs: u8) -> Self::Output {
        Quantity { mmol: self.mmol / rhs as u32 }
    }
}

impl SubAssign for Quantity {
    fn sub_assign(&mut self, rhs: Self) {
        self.mmol -= rhs.mmol;
    }
}

impl AddAssign for Quantity {
    fn add_assign(&mut self, rhs: Self) {
        self.mmol += rhs.mmol;
    }
}

impl Div<Quantity> for Quantity {
    type Output = PerMol;

    fn div(self, rhs: Quantity) -> Self::Output {
        Self::Output {
            mmol_per: (self.mmol as f32 / rhs.mmol as f32) as u32
        }
    }
}


impl Quantity {
    pub fn none() -> Self {
        Quantity { mmol: 0 }
    }
}

impl Default for Quantity {
    fn default() -> Self {
        Quantity { mmol: 1000 }
    }
}

impl From<u32> for Quantity {
    fn from(value: u32) -> Self {
        Quantity { mmol: value }
    }
}

impl Add for Quantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Quantity {
            mmol: self.mmol + rhs.mmol
        }
    }
}

impl Sub for Quantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Quantity {
            mmol: self.mmol - rhs.mmol
        }
    }
}

impl Sum for Quantity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Quantity { mmol: iter.map(|q| q.mmol).sum() }
    }
}

impl Mul<f32> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: f32) -> Self::Output {
        Quantity { mmol: (self.mmol as f32 * rhs) as u32 }
    }
}

impl Mul<Quantity> for f32 {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Self::Output {
        Quantity { mmol: (self * rhs.mmol as f32) as u32 }
    }
}