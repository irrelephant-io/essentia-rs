use crate::abstractions::{
    physics::{
        TimeSpan,
        get_delta_temp
    },
    reaction::Product
};

impl super::Essentia {
    fn run_reactions(&self, delta_time: &TimeSpan) -> Vec<Product> {
        self.reaction_lookup
            .iter()
            .flat_map(|(_, reaction)| 
                self
                    .substances
                    .iter()
                    .flat_map(|substance| 
                        reaction.react(&self.environment, delta_time, substance)
                    )
            )
            .collect()
    }

    pub fn simulate(&mut self, delta_time: TimeSpan) {
        self
            .run_reactions(&delta_time)
            .drain(..)
            .for_each(|p| {
                match p {
                    Product::Thermal(ex) => {
                        let delta_temp = get_delta_temp(self, ex);
                        self.environment.temperature += delta_temp;
                    },
                    _ => { }
                }
            });

        self.environment.time += delta_time;
    }
}