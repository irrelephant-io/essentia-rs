use std::{iter::Sum, ops::Add};

use super::{Energy, Quantity, Temperature};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct SpecificHeatCapacity {
    pub joule_mmol_per_degree: u32
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HeatCapacity {
    pub joule_per_degree: u32
}

impl From<u32> for HeatCapacity {
    fn from(value: u32) -> Self {
        Self { joule_per_degree: value }
    }
}

impl From<u32> for SpecificHeatCapacity {
    fn from(value: u32) -> Self {
        Self { joule_mmol_per_degree: value }
    }
}

impl Default for SpecificHeatCapacity {
    fn default() -> Self {
        Self { joule_mmol_per_degree: 1 }
    }
}

impl HeatCapacity {
    pub fn from_specific(quantity: Quantity, heat_capacity: SpecificHeatCapacity) -> Self {
        Self {
            joule_per_degree: heat_capacity.joule_mmol_per_degree * quantity.mmol
        }
    }

    pub fn get_delta_temp(&self, e: Energy) -> Temperature {
        Temperature::from(e.joules / (self.joule_per_degree as i32))
    }
}

impl Add for HeatCapacity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { joule_per_degree: self.joule_per_degree + rhs.joule_per_degree }
    }
}

impl Sum for HeatCapacity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        HeatCapacity { joule_per_degree: iter.map(|c| c.joule_per_degree ).sum() }
    }
}