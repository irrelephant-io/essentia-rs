use std::collections::HashSet;

use crate::{
    abstractions::{
        physics::{
            get_heat_capacity, Quantity, TimeSpan
        },
        reaction::Product, SubstanceId
    }, engine::ReactionContext, physics::Solubility, EssenceId, FormId, Substance, SubstanceBuilder
};

impl super::Essentia {
    fn run_reactions(&mut self) -> ReactionContext {
        self.reactions
            .iter_groups()
            .fold(
                ReactionContext::new(self),
                |context, group| {
                    let result = group
                        .iter_reactions()
                        .flat_map(|r| r.react(&context))
                        .collect::<Vec<_>>();

                    context.apply(result)
                }
            )
    }

    pub fn simulate(&mut self, delta_time: TimeSpan) {
        self.delta_time = delta_time;
        self.heat_capacity = get_heat_capacity(&self);

        let mut products = self
            .run_reactions()
            .pending_products;

        self.is_in_equilibrium = products.len() == 0;

        products
            .drain(..)
            .for_each(|p| {
                match p {
                    Product::Thermal(power) => {
                        let delta_e = power * delta_time;
                        self.environment.temperature += self.heat_capacity.get_delta_temp(delta_e);
                    },
                    Product::Produce(essence_id, form_id, quantity) => {
                        self.produce_substance(essence_id, form_id, quantity);
                    },
                    Product::Consume(essence_id, form_id, quantity) => {
                        self.consume_substance(essence_id, form_id, quantity);
                    },
                    Product::Dissolve(essence_id, form_id, substance_id, quantity) => {
                        self.dissolve_substance(essence_id, form_id, substance_id, quantity);
                    }
                    Product::Precipitate(essence_id, form_id, substance_id, quantity) => {
                        self.precipitate_substance(essence_id, form_id, substance_id, quantity);
                    }
                }
            });

        self.environment.time += self.delta_time;
    }

    fn dissolve_substance(&mut self, essence_id: EssenceId, form_id: FormId, substance_id: SubstanceId, qty_to_dissolve: Quantity) {
        let solute_ids = self.get_matching_solute_ids(essence_id, form_id);
        let solvent = self.substances.remove(&substance_id);
        if let Some(solvent) = solvent {
            let solutes = self.substances
                .extract_if(|id, _| solute_ids.contains(id))
                .collect::<Vec<_>>();
            let mut solution_builder = SubstanceBuilder::new(&self)
                .is_solution()
                .with_base(solvent);
            
            let mut remainders = vec![];
            for (_, solute) in solutes {
                let (solute, remainder) = solute.divide(qty_to_dissolve);
                let actual_qty_to_dissolve = solute.get_quantity();
                solution_builder = solution_builder.with_solute(solute, actual_qty_to_dissolve);
                if let Some(remainder) = remainder {
                    remainders.push(remainder);
                }
            }
            let substance = solution_builder.build();
            self.add_substance(substance);
            for remainder in remainders {
                self.add_substance(remainder);
            }
        }
    }

    fn precipitate_substance(&mut self, essence_id: EssenceId, form_id: FormId, substance_id: SubstanceId, quantity: Quantity) {
        let solution = self.substances
            .get_mut(&substance_id);

        if solution.is_none() {
            return;
        }

        if let Some(Substance::Solution(_, _, solutes)) = solution {
            let mut quantity_to_precipitate = Quantity::none();
            solutes
                .retain(|&solute_essence_id, solute_quantity| {
                    if solute_essence_id == essence_id {
                        if *solute_quantity > quantity {
                            *solute_quantity -= quantity;
                            quantity_to_precipitate = quantity;
                        } else {
                            quantity_to_precipitate = *solute_quantity;
                            return false
                        }
                    }

                    return true;
                });
            
            if quantity_to_precipitate > Quantity::none() {
                self.produce_substance(essence_id, form_id, quantity_to_precipitate);
            }

        } else {
            panic!("Substance to precipitate from is not a solution!");
        }
    }

    fn get_matching_solute_ids(&mut self, essence_id: EssenceId, form_id: FormId) -> HashSet<SubstanceId> {
        self.substances.values()
            .filter_map(|substance| {
                // Only normal substances can be solutes
                if let Substance::Free(id, data) = substance {
                    if data.essence_id == essence_id && data.form_id == form_id {
                        return Some(id);
                    }
                }
    
                return None;
            })
            .copied()
            .collect::<HashSet<_>>()
    }
    
    fn produce_substance(&mut self, essence_id: EssenceId, form_id: FormId, quantity: Quantity) {
        for (_, substance) in self.substances.iter_mut() {
            match substance {
                Substance::Free(_, data) => {
                    if data.essence_id == essence_id && data.form_id == form_id {
                        data.quantity += quantity;
                        return;
                    }
                },
                Substance::Solution(_, data, _) => {
                    if data.essence_id == essence_id && data.form_id == form_id {
                        data.quantity += quantity;
                        return;
                    }
                }
            }
        }

        self.add_substance(
            SubstanceBuilder::new(self)
                .is_normal()
                .with_essence(essence_id)
                .with_form(form_id)
                .with_quantity(quantity)
                .build()
        );
    }
    
    fn consume_substance(&mut self, essence_id: EssenceId, form_id: FormId, quantity: Quantity) {
        let mut quantity_left = quantity;
        let mut solutes_to_fall_out = vec![];
        self.substances.retain(|_, substance| {
            if quantity_left == Quantity::none() {
                return true;
            }
            match substance {
                Substance::Free(_, data) => {
                    if data.essence_id == essence_id && data.form_id == form_id {
                        if data.quantity > quantity_left {
                            data.quantity -= quantity_left;
                            quantity_left = Quantity::none();
                            return true;
                        } else {
                            quantity_left -= data.quantity;
                            return false;
                        }
                    }
                    return true;
                },
                Substance::Solution(_, data, solutes) => {
                    if data.essence_id == essence_id && data.form_id == form_id {
                        if data.quantity > quantity_left {
                            data.quantity -= quantity_left;
                            quantity_left = Quantity::none();
                            return true;
                        } else {
                            quantity_left -= data.quantity;
                            for solute in solutes.clone() {
                                solutes_to_fall_out.push(solute.clone());
                            }
                            return false;
                        }
                    }
                    return true;
                }
            }
        });

        for (essence_id, quantity) in solutes_to_fall_out {
            let solute_essence = self
                .get_essence(essence_id)
                .expect("Couldn't find solute essence when precipitating the remains");
            
            if let Solubility::Solute(form_id, _) = solute_essence.solubility.expect("Solute is insoluble. Weird.") {
                self.produce_substance(essence_id, form_id, quantity);
            } else {
                panic!("Non-solute was dissolved into a solution.");
            }
        }
    }
}