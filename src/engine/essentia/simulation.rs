use crate::{abstractions::{
    physics::{
        get_delta_temp, TimeSpan
    },
    reaction::Product
}, physics::Quantity, Substance};

impl super::Essentia {
    fn run_reactions(&self) -> Vec<Product> {
        self.reactions
            .iter()
            .flat_map(|reaction| 
                reaction.react(self)
            )
            .collect()
    }

    pub fn simulate(&mut self, delta_time: TimeSpan) {
        self.delta_time = delta_time;

        self
            .run_reactions()
            .drain(..)
            .for_each(|p| {
                match p {
                    Product::Thermal(power) => {
                        let delta_e = power * delta_time;
                        let delta_temp = get_delta_temp(self, delta_e);
                        self.environment.temperature += delta_temp;
                    },
                    Product::Produce(substance) => {
                        self.substances.push(substance)
                    },
                    Product::Consume(substance_id, quantity) => {
                        self.consume_substance(substance_id, quantity);
                    }
                }
            });

        self.environment.time += self.delta_time;
    }

    fn consume_substance(&mut self, substance_id: u16, quantity: Quantity) {
        let found = self.substances.iter_mut()
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