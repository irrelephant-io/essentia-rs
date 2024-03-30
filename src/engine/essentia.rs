use std::collections::HashMap;

use crate::{abstractions::{reaction::Reaction, Environment, Essence, Form, Substance}, physics::TimeSpan};

pub struct Essentia {
    pub environment: Environment,
    pub delta_time: TimeSpan,

    substances: Vec::<Substance>,
    essence_lookup: HashMap::<u16, Essence>,
    form_lookup: HashMap::<u16, Form>,
    reactions: Vec<Box<dyn Reaction>>
}

impl Essentia {
    pub fn new(environment: Environment) -> Self {
        Essentia {
            environment,
            delta_time: TimeSpan::from(0),
            substances: Vec::<Substance>::new(),

            essence_lookup: HashMap::new(),
            form_lookup: HashMap::new(),
            reactions: vec![]
        }
    }
}

// Contains methods related to manipulating registrations
// such as forms, essences and reactions.
mod registrations;

// Contains engine simulation methods.
mod simulation;

// Contains code for querying system's contents
mod querying;