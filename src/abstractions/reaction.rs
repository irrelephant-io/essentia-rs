use crate::{
    abstractions::{
        physics::Power,
        Substance
    },
    engine::Essentia, physics::Quantity
};

pub enum Product {
    Produce(Substance),
    Consume(u16, Quantity),
    Thermal(Power),
}

pub trait Reaction {
    fn react(&self, engine: &Essentia) -> Vec::<Product>;
}