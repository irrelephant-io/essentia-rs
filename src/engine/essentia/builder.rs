use std::collections::HashMap;

use crate::{
    abstractions::{
        reaction::Reaction,
        Essence,
        Form
    },
    engine::builtin_reactions::{
        FormTransition,
        Precipitation,
        Solution
    },
    physics::{HeatCapacity, TimeSpan},
    Environment,
    EssenceId, FormId
};

use super::{reactions::ReactionLookup, Essentia};

pub struct EssentiaBuilder {
    essence_lookup: HashMap::<EssenceId, Essence>,
    form_lookup: HashMap::<FormId, Form>,
    reactions: ReactionLookup,
    starting_environment: Option<Environment>
}

impl EssentiaBuilder {
    pub fn new() -> Self {
        EssentiaBuilder {
            starting_environment: Option::default(),
            essence_lookup: HashMap::new(),
            form_lookup: HashMap::new(),
            reactions: ReactionLookup::new()
        }
    }

    pub fn build(self) -> Essentia {
        Essentia {
            _private_ctor: (),
            heat_capacity: HeatCapacity::from(0),
            environment: self.starting_environment.unwrap_or(Environment::new()),
            delta_time: TimeSpan::from(0),
            substances: HashMap::new(),
            essence_lookup: self.essence_lookup,
            form_lookup: self.form_lookup,
            reactions: self.reactions
        }
    }

    pub fn register_essence(&mut self, essence: Essence) {
        self.essence_lookup.insert(essence.id, essence);
    }

    pub fn register_form(&mut self, form: Form) {
        self.form_lookup.insert(form.id, form);
    }

    pub fn register_reaction(&mut self, reaction: Box<dyn Reaction>) {
        self.reactions.insert(reaction);
    }
}

impl Default for EssentiaBuilder {
    fn default() -> Self {
        let mut builder = Self::new();
        builder.register_reaction(Box::new(FormTransition {}));
        builder.register_reaction(Box::new(Solution { optimal_dissolution_speed_percent: 20 }));
        builder.register_reaction(Box::new(Precipitation {}));
        builder
    }
}