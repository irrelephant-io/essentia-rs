use crate::{
    abstractions::{
        physics::{
            get_heat_capacity, Quantity, TimeSpan
        },
        reaction::Product, SubstanceId
    }, engine::ReactionContext, Either, EssenceId, FormId, Substance, SubstanceBuilder, SubstanceData
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

        self
            .run_reactions()
            .pending_products
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
                    _ => todo!("This reaction type is not supported!")
                }
            });

        self.environment.time += self.delta_time;
    }

    fn dissolve_substance(&mut self, essence_id: EssenceId, form_id: FormId, substance_id: SubstanceId, qty_to_dissolve: Quantity) {
        let pos_or_solvent = self.substances
            .iter_mut()
            .enumerate()
            .find_map(|(idx, s)| {
                match s {
                    Substance::Normal(substance_data) => {
                        if substance_data.substance_id == substance_id {
                            return Some(Either::Left(idx));
                        }
                    },
                    Substance::Solution(substance_data, _) => {
                        if substance_data.substance_id == substance_id {
                            return Some(Either::Right(s));
                        }
                    }
                }
                return None;
            });

        if let Some(solvent_or_pos) = pos_or_solvent {
            match solvent_or_pos {
                Either::Left(idx) => {
                    let solvent = self.substances.remove(idx);
                    if let Substance::Normal(solvent_data) = solvent {
                        self.substances.push(
                            Substance::Solution(
                                solvent_data,
                                SubstanceData::new(essence_id, form_id, qty_to_dissolve)
                            )
                        );
                    }
                },
                Either::Right(Substance::Solution(_, solute)) => {
                    solute.quantity += qty_to_dissolve;
                },
                _ => {}
            }
        }

        let solute_position = self.substances
            .iter()
            .position(|s| {
                if let Substance::Normal(subtance_data) = s {
                    if subtance_data.essence_id == essence_id && subtance_data.form_id == form_id {
                        return true;
                    }
                }
                return false;
            });
        if let Some(solute_position) = solute_position {
            let solute = self.substances.get_mut(solute_position).unwrap();
            if let Substance::Normal(solute) = solute {
                if solute.quantity > qty_to_dissolve {
                    solute.quantity -= qty_to_dissolve;
                } else {
                    self.substances.remove(solute_position);
                }
            }
        }
    }

    fn produce_substance(&mut self, essence_id: EssenceId, form_id: FormId, quantity: Quantity) {
        if let Some(substance_data) = self.substances
            .iter_mut()
            .find_map(|s| {
                if let Substance::Normal(subtance_data) = s {
                    if subtance_data.essence_id == essence_id && subtance_data.form_id == form_id {
                        return Some(subtance_data);
                    }
                }
                return None;
            }) {
                substance_data.quantity += quantity;
            } else {
                self.substances.push(
                    SubstanceBuilder::new(&self)
                        .with_essence(essence_id)
                        .with_form(form_id)
                        .with_quantity(quantity)
                        .build()
                );
            }
    }
    
    fn consume_substance(&mut self, essence_id: EssenceId, form_id: FormId, quantity: Quantity) {
        let found = self.substances
            .iter_mut()
            .enumerate()
            .find(|(_, s)| {
                match s {
                    Substance::Normal(n) =>
                        n.essence_id == essence_id && n.form_id == form_id,
                    Substance::Solution(n, s) =>
                        n.essence_id == essence_id && n.form_id == form_id
                        ||
                        s.essence_id == essence_id && s.form_id == form_id
                }
            });

            if let Some((found_idx, found)) = found {
                match found {
                    Substance::Normal(normal) => {
                        if normal.quantity > quantity {
                            normal.quantity -= quantity;
                        } else {
                            self.substances.remove(found_idx);
                            
                        }
                    }
                    Substance::Solution(solvent, solution) => {
                        if solvent.essence_id == essence_id && solvent.form_id == form_id {
                            if solvent.quantity > quantity {
                                solvent.quantity -= quantity;
                            } else {
                                if let Substance::Solution(_, solution) = self.substances.remove(found_idx) {
                                    self.substances.push(Substance::Normal(solution));
                                }
                            }
                        } else {
                            if solution.quantity > quantity {
                                solution.quantity -= quantity;
                            } else {
                                if let Substance::Solution(solvent, _) = self.substances.remove(found_idx) {
                                    self.substances.push(Substance::Normal(solvent));
                                }
                            }
                        }
                    }
                }
            }
    }
}