use crate::{engine::ReactionContext, physics::{Quantity, Solubility}, reaction::{Product, Reaction}};

pub struct Solution {
    pub optimal_dissolution_speed_percent: u32
}

impl Reaction for Solution {
    fn react(&self, context: &ReactionContext) -> Vec::<Product> {
        let solvents = context.engine.iter_solvents();
        let solutes = context.engine.iter_solutes();

        let total_weight = solutes
            .map(|(solute, solubility)| {
                match solubility {
                    Solubility::Solute(_, weight) => solute.get_quantity() * weight,
                    _ => Quantity::none()
                }
            })
            .sum::<Quantity>();

        let total_saturation_limit = solvents
            .map(|(solvent, solubility)| {
                match solubility {
                    Solubility::Solvent(_, limit) => solvent.get_quantity() * limit,
                    _ => Quantity::none()
                }
            })
            .sum::<Quantity>();

        let mut products: Vec<Product> = vec![];

        for (solute, solute_solubility) in context.engine.iter_solutes() {
            for (solvent, solvent_solubility) in context.engine.iter_solvents() {
                if let Solubility::Solvent(_, saturation_limit) = solvent_solubility {
                    if let Solubility::Solute(_, weight) = solute_solubility {
                        let solvent_saturation_limit = solvent.get_quantity() * saturation_limit.mmol_per;
                        let solute_weight = solute.get_quantity() * weight.mmol_per;

                        let relative_solubility = solvent_saturation_limit.mmol as f32 / total_saturation_limit.mmol as f32;
                        let relative_weight = total_weight.mmol as f32 / solute_weight.mmol as f32;
                        let solubility_ratio = relative_solubility / relative_weight;
                        let saturation_ratio = 1.0 - solvent_solubility.get_saturation_percent(solvent).min(0.9);

                        let total_solubility = solubility_ratio * saturation_ratio;

                        let maximum_dissolve = solvent.get_quantity() * self.optimal_dissolution_speed_percent / 100 * total_solubility;
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