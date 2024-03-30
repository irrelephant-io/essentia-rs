use essentia_rs::{
    engine::Essentia, physics::{Power, Quantity}, reaction::{Product, Reaction},
};

use crate::data::essence::Essences;

pub struct CryodustChill {
    chill_per_mol: Power
}

impl Reaction for CryodustChill {
    fn react(
        &self,
        engine: &Essentia
    ) -> Vec::<Product> {
        let total_cryo = engine
            .get_of_essense(Essences::Cryodust.into())
            .map(|pyro| pyro.quantity)
            .sum::<Quantity>();

        if total_cryo.mol > 0 {
            vec![ Product::ThermalPower(-self.chill_per_mol * total_cryo)]
        } else {
            vec![]
        }
    }
}
impl Default for CryodustChill {
    fn default() -> Self {
        CryodustChill { chill_per_mol: Power::from(-2) }
    }
}