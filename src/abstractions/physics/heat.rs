use std::{iter::Sum, ops::Add};

use super::{Energy, Quantity, Temperature};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct SpecificHeatCapacity {
    pub joule_mol_per_kelvin: u32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HeatCapacity {
    pub joule_per_mkelvin: u32,
}

impl From<u32> for HeatCapacity {
    fn from(value: u32) -> Self {
        Self {
            joule_per_mkelvin: value,
        }
    }
}

impl From<u32> for SpecificHeatCapacity {
    fn from(value: u32) -> Self {
        Self {
            joule_mol_per_kelvin: value,
        }
    }
}

impl Default for SpecificHeatCapacity {
    fn default() -> Self {
        Self {
            joule_mol_per_kelvin: 1,
        }
    }
}

impl HeatCapacity {
    pub fn from_specific(quantity: Quantity, heat_capacity: SpecificHeatCapacity) -> Self {
        Self {
            joule_per_mkelvin: heat_capacity.joule_mol_per_kelvin * quantity.mmol,
        }
    }

    pub fn get_delta_temp(&self, e: Energy) -> Temperature {
        Temperature::from(e.joules / (self.joule_per_mkelvin as i32))
    }
}

impl Add for HeatCapacity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            joule_per_mkelvin: self.joule_per_mkelvin + rhs.joule_per_mkelvin,
        }
    }
}

impl Sum for HeatCapacity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        HeatCapacity {
            joule_per_mkelvin: iter.map(|c| c.joule_per_mkelvin).sum(),
        }
    }
}
