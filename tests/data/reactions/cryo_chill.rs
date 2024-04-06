use essentia_rs::{
    engine::ReactionContext, physics::{Power, Quantity, Rate}, reaction::{Product, Reaction},
};

use crate::data::essence::Essences;

pub struct CryodustChill {
    chill_per_mol: Power,
    consumption_rate: Rate
}

impl Reaction for CryodustChill {
    fn react(
        &self,
        context: &ReactionContext
    ) -> Vec::<Product> {
        let all_cryo = context
            .engine
            .iter_all()
            .filter(|s| 
                s.get_essence() == Essences::Cryodust.into()
            )
            .collect::<Vec<_>>();

        let total_cryo = all_cryo
            .iter()
            .map(|c| c.get_quantity())
            .sum::<Quantity>();

        if total_cryo.mmol > 0 {
            let mut products = vec![
                Product::Thermal(-self.chill_per_mol * total_cryo)
            ];

            all_cryo
                .iter()
                .for_each(|c| {
                    products.push(Product::Consume(
                        c.get_essence(),
                        c.get_form(),
                        self.consumption_rate * context.engine.delta_time
                    ));
                });
            
            products
        } else {
            vec![]
        }
    }
    
    fn get_priority(&self) -> u8 { 100 }
}
impl Default for CryodustChill {
    fn default() -> Self {
        CryodustChill { chill_per_mol: Power::from(2), consumption_rate: Rate::default() }
    }
}

impl CryodustChill {
    pub fn new(power: Power, consumption_rate: Rate) -> Self {
        CryodustChill { chill_per_mol: power, consumption_rate }
    }
}