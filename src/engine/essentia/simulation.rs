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
                        self.substances.push(
                            SubstanceBuilder::new(&self)
                                .with_essence(essence_id)
                                .with_form(form_id)
                                .with_quantity(quantity)
                                .build()
                        );
                    },
                    Product::Consume(substance_id, quantity) => {
                        self.consume_substance(substance_id, quantity);
                    },
                    _ => todo!("This reaction type is not supported!")
                }
            });

        self.environment.time += self.delta_time;
    }

    fn consume_substance(&mut self, substance_id: u16, quantity: Quantity) {
        let found = self.substances
            .iter_mut()
            .enumerate()
            .find(|(_, s)| {
                match s {
                    Substance::Normal(n) =>
                        n.substance_id == substance_id,
                    Substance::Solution(n, s) =>
                        n.substance_id == substance_id || s.substance_id == substance_id
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
                        if solvent.substance_id == substance_id {
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