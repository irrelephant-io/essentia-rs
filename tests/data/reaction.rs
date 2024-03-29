use essentia_rs::{Energy, Environment, Product, Reaction, Substance::{self, Normal}};

pub struct PyroflaxHeat {
    heat_per_unit_per_time: u16
}

impl Reaction for PyroflaxHeat {
    fn react(
        &self,
        _environment: &Environment,
        substance: &Substance
    ) -> Vec::<Product> {
        if let Normal(data) = &substance {
            let heat = data.quantity.mol * self.heat_per_unit_per_time;
            vec![
                Product::Exotherm(Energy::from(heat))
            ]
        } else {
            vec![]
        }
    }
    
    fn get_id(&self) -> u16 { 1 }
}

impl PyroflaxHeat {
    pub fn default() -> Self {
        PyroflaxHeat { heat_per_unit_per_time: 1 }
    }
}