use crate::{
    abstractions::{
        physics::{
            get_heat_capacity, Quantity, TimeSpan
        },
        reaction::Product
    },
    engine::ReactionContext,
    Substance, SubstanceBuilder
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
                        if let Some(substance_data) = self.substances
                            .iter_mut()
                            .find_map(|s| {
                                if let Substance::Normal(subtance_data) = s {
                                    if subtance_data.essence_id == essence_id && subtance_data.form_id == form_id {
                                        Some(subtance_data)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
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
                    },
                    Product::Consume(essence_id, form_id, quantity) => {
                        self.consume_substance(essence_id, form_id, quantity);
                    },
                    _ => todo!("This reaction type is not supported!")
                }
            });

        self.environment.time += self.delta_time;
    }

    fn consume_substance(&mut self, essence_id: u16, form_id: u16, quantity: Quantity) {
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