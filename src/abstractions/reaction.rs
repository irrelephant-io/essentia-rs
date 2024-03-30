use crate::{
    abstractions::{
        physics::Power,
        Substance
    },
    engine::Essentia
};

pub enum Product {
    Produce(Substance),
    ThermalPower(Power),
}

pub trait Reaction {
    fn react(&self, engine: &Essentia) -> Vec::<Product>;
}