use essentia_rs::{physics::{Energy, TimeSpan}, reaction::{Product, Reaction}, Environment, Substance::{self, Normal}};

pub struct PyroflaxHeat {
    heat_per_unit_per_time: u16
}

impl Reaction for PyroflaxHeat {
    fn react(
        &self,
        _environment: &Environment,
        delta_time: &TimeSpan,
        substance: &Substance
    ) -> Vec::<Product> {
        if let Normal(data) = &substance {
            let heat = data.quantity.mol * self.heat_per_unit_per_time * delta_time.ticks;
            vec![
                Product::Thermal(Energy::from(heat as i16))
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