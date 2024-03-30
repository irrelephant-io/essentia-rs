use crate::abstractions::Environment;
use crate::abstractions::physics::{Energy, TimeSpan};
use crate::abstractions::Substance;

pub enum Product {
    Substance(Substance),
    Thermal(Energy),
}

pub trait Reaction {
    fn get_id(&self) -> u16;
    fn react(
        &self,
        environment: &Environment,
        delta_time: &TimeSpan,
        substance: &Substance
    ) -> Vec::<Product>;
}