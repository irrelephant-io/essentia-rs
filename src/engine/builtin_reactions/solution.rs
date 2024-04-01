use crate::{engine::ReactionContext, physics::{Quantity, Rate, Solubility}, reaction::{Product, Reaction}, SubstanceData};

pub struct Solution {
    pub optimal_dissolution_speed_percent: u8
}

impl Reaction for Solution {
    fn react(&self, context: &ReactionContext) -> Vec::<Product> {
        let (solvents, solutes): (Vec<_>, Vec<_>) = context.engine
            .get_with_solubility()
            .partition(|(_, solubility) | {
                if let Solubility::Solvent(_) = solubility {
                    true
                } else {
                    false
                }
            });

        let total_weight = solutes
            .iter()
            .fold(
                Quantity::from(0),
                |acc, (_, solubility)| {
                    if let &Solubility::Solute(weight) = solubility {
                        acc + weight
                    } else {
                        acc
                    }
                });

        let total_saturation_limit = solvents
            .iter()
            .fold(
            Quantity::from(0),
            |acc, (_, solubility)| {
                if let &Solubility::Solvent(saturation_limit) = solubility {
                    acc + saturation_limit
                } else {
                    acc
                }
            });

        let mut products: Vec<Product> = vec![];

        for (solute, solute_solubility) in solutes {
            for (solvent, solvent_solubility) in &solvents {
                if let &Solubility::Solvent(solvent_solubility) = solvent_solubility {
                    if let Solubility::Solute(solute_solubility) = solute_solubility {
                        let relative_solubility = total_saturation_limit.mol as f32 / solvent_solubility.mol as f32;
                        let relative_weight = total_weight.mol as f32 / solute_solubility.mol as f32;
                        let absolute_solubility = relative_solubility / relative_weight;
                        let maximum_dissolve = solvent.quantity * self.optimal_dissolution_speed_percent / 100 * absolute_solubility;
                        products.push(Product::Dissolve(solute.essence_id, solute.form_id, solvent.substance_id, maximum_dissolve));
                    }
                }
            }
        }

        products
    }

    // Solubility is applied before the form transitions
    fn get_priority(&self) -> u8 { u8::MAX - 1 }
}