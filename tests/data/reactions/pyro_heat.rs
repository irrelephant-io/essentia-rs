use essentia_rs::{engine::ReactionContext, physics::{Power, Quantity}, reaction::{Product, Reaction}};

use crate::data::essence::Essences;


pub struct PyroflaxHeat {
    power_per_mol: Power
}

impl Reaction for PyroflaxHeat {
    fn react(
        &self,
        context: &ReactionContext
    ) -> Vec::<Product> {
        let total_pyro = context
            .engine
            .get_of_essense(Essences::Pyroflux.into())
            .map(|pyro| pyro.quantity)
            .sum::<Quantity>();

        if total_pyro.mol > 0 {
            vec![ Product::Thermal(self.power_per_mol * total_pyro) ]
        } else {
            vec![]
        }
    }
    
    fn get_priority(&self) -> u8 { 100 }
}

impl PyroflaxHeat {
    pub fn default() -> Self {
        PyroflaxHeat { power_per_mol: Power::from(1) }
    }
}

impl From<u16> for PyroflaxHeat {
    fn from(value: u16) -> Self {
        PyroflaxHeat { power_per_mol: Power::from(value as i32) }
    }
}