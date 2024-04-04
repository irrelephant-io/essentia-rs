use crate::{engine::ReactionContext, physics::{Quantity, Solubility}, reaction::{Product, Reaction}};

pub struct Solution {
    pub optimal_dissolution_speed_percent: u8
}

impl Reaction for Solution {
    fn react(&self, context: &ReactionContext) -> Vec::<Product> {
        
        let solvents = context.engine.iter_solvents();
        let solutes = context.engine.iter_solutes();

        let total_weight = solutes
            .map(|(_, solubility)| {
                match solubility {
                    Solubility::Solute(_, weight) => weight,
                    _ => Quantity::none()
                }
            })
            .sum::<Quantity>();

        let total_saturation_limit = solvents
            .map(|(_, solubility)| {
                match solubility {
                    Solubility::Solvent(_, limit) => limit,
                    _ => Quantity::none()
                }
            })
            .sum::<Quantity>();

        let mut products: Vec<Product> = vec![];

        for (solute, solute_solubility) in context.engine.iter_solutes() {
            for (solvent, solvent_solubility) in context.engine.iter_solvents() {
                if let Solubility::Solvent(_, solvent_solubility) = solvent_solubility {
                    if let Solubility::Solute(_, solute_solubility) = solute_solubility {
                        let relative_solubility = total_saturation_limit.mol as f32 / solvent_solubility.mol as f32;
                        let relative_weight = total_weight.mol as f32 / solute_solubility.mol as f32;
                        let absolute_solubility = relative_solubility / relative_weight;
                        let maximum_dissolve = solvent.get_quantity() * self.optimal_dissolution_speed_percent / 100 * absolute_solubility;
                        products.push(Product::Dissolve(solute.get_essence(), solute.get_form(), solvent.get_substance(), maximum_dissolve));
                    }
                }
            }
        }

        products
    }

    // Solubility is applied before the form transitions
    fn get_priority(&self) -> u8 { u8::MAX - 1 }
}