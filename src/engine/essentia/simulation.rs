use crate::abstractions::{
    physics::{
        TimeSpan,
        get_delta_temp
    },
    reaction::Product
};

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
                    Product::ThermalPower(power) => {
                        let delta_e = power * delta_time;
                        let delta_temp = get_delta_temp(self, delta_e);
                        self.environment.temperature += delta_temp;
                    },
                    _ => { }
                }
            });

        self.environment.time += self.delta_time;
    }
}