use crate::{Energy, Environment, Substance};

pub enum Product {
    Substance(Substance),
    Exotherm(Energy)
}

pub trait Reaction {
    fn get_id(&self) -> u16;
    fn react(&self, environment: &Environment, substance: &Substance) -> Vec::<Product>;
}