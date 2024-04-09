use essentia_rs::{
    engine::ReactionContext,
    physics::{Power, Quantity},
    reaction::{Product, Reaction},
};

use crate::data::essence::Essences;

pub struct PyroflaxHeat {
    power_per_mmol: Power,
}

impl Reaction for PyroflaxHeat {
    fn react(&self, context: &ReactionContext) -> Vec<Product> {
        let total_pyro = context
            .engine
            .iter_all()
            .filter_map(|s| {
                if s.get_essence() == Essences::Pyroflux.into() {
                    Some(s.get_quantity())
                } else {
                    None
                }
            })
            .sum::<Quantity>();

        if total_pyro.mmol > 0 {
            vec![Product::Thermal(self.power_per_mmol * total_pyro)]
        } else {
            vec![]
        }
    }

    fn get_priority(&self) -> u8 {
        100
    }
}

impl PyroflaxHeat {
    pub fn default() -> Self {
        PyroflaxHeat {
            power_per_mmol: Power::from(1),
        }
    }
}

impl From<u16> for PyroflaxHeat {
    fn from(value: u16) -> Self {
        PyroflaxHeat {
            power_per_mmol: Power::from(value as i32),
        }
    }
}
