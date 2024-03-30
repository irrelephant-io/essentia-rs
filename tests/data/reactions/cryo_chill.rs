use std::iter::once;
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
        let all_cryo = engine
            .get_of_essense(Essences::Cryodust.into())
            .collect::<Vec<_>>();

        let total_cryo = all_cryo.iter()
            .map(|pyro| pyro.quantity)
            .sum::<Quantity>();

        if total_cryo.mol > 0 {
            once(Product::Thermal(-self.chill_per_mol * total_cryo))
                .chain(
                    all_cryo.iter()
                        .map(|c| Product::Consume(c.substance_id, Quantity::from(engine.delta_time.ticks)))
                )
                .collect::<Vec<Product>>()
                
        } else {
            vec![]
        }
    }
}
impl Default for CryodustChill {
    fn default() -> Self {
        CryodustChill { chill_per_mol: Power::from(2) }
    }
}