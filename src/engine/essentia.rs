use std::collections::HashMap;

use crate::{abstractions::{Environment, Essence, Form, Substance, SubstanceId}, physics::{HeatCapacity, TimeSpan}, EssenceId, FormId};

pub struct Essentia {
    _private_ctor: (),
    
    pub environment: Environment,
    pub heat_capacity: HeatCapacity,
    pub delta_time: TimeSpan,

    substances: HashMap::<SubstanceId, Substance>,
    essence_lookup: HashMap::<EssenceId, Essence>,
    form_lookup: HashMap::<FormId, Form>,
    reactions: ReactionLookup
}

impl Essentia {
    pub fn get_essence(&self, id: EssenceId) -> Option<&Essence> {
        self.essence_lookup.get(&id)
    }

    pub fn get_form(&self, id: FormId) -> Option<&Form> {
        self.form_lookup.get(&id)
    }

    pub fn add_substance(&mut self, substance: Substance) {
        let id = match substance {
            Substance::Free(id, _) => id,
            Substance::Solution(id, _, _) => id
        };
        self.substances.insert(id, substance);
    }
}

mod reactions;

// Contains engine simulation methods.
mod simulation;

// Contains code for querying system's contents
mod querying;

// Contains code to construct an instance of an engine
mod builder;
pub use builder::EssentiaBuilder;

use self::reactions::ReactionLookup;