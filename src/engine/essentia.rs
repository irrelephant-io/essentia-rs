use std::collections::HashMap;

use crate::{abstractions::{reaction::Reaction, Environment, Essence, Form, Substance}, physics::TimeSpan};

pub struct Essentia {
    _private_ctor: (),
    pub environment: Environment,
    pub delta_time: TimeSpan,

    substances: Vec::<Substance>,
    essence_lookup: HashMap::<u16, Essence>,
    form_lookup: HashMap::<u16, Form>,
    reactions: Vec<Box<dyn Reaction>>
}

impl Essentia {
    pub fn get_essence(&self, id: u16) -> Option<&Essence> {
        self.essence_lookup.get(&id)
    }

    pub fn get_form(&self, id: u16) -> Option<&Form> {
        self.form_lookup.get(&id)
    }

    pub fn add_substance(&mut self, substance: Substance) {
        self.substances.push(substance);
    }
}

// Contains engine simulation methods.
mod simulation;

// Contains code for querying system's contents
mod querying;

// Contains code to construct an instance of an engine
mod builder;
pub use builder::EssentiaBuilder;