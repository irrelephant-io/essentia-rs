use std::collections::HashMap;

use crate::abstractions::{reaction::Reaction, Environment, Essence, Form, Substance};

pub struct Essentia {
    pub environment: Environment,
    pub substances: Vec::<Substance>,

    essence_lookup: HashMap::<u16, Essence>,
    form_lookup: HashMap::<u16, Form>,
    reaction_lookup: HashMap<u16, Box<dyn Reaction>>
}

impl Essentia {
    pub fn new(environment: Environment) -> Self {
        Essentia {
            environment,
            substances: Vec::<Substance>::new(),

            essence_lookup: HashMap::new(),
            form_lookup: HashMap::new(),
            reaction_lookup: HashMap::new()
        }
    }
}

// Contains methods related to manipulating registrations
// such as forms, essences and reactions.
mod registrations;

// Contains engine simulation methods.
mod simulation;