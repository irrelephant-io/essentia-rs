use std::collections::HashMap;

use crate::{
    abstractions::{
        env::{Environment, Time},
        substance::Substance
    },
    Essence,
    Form,
    Product,
    Reaction
};

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

    pub fn register_essence(&mut self, essence: Essence) {
        self.essence_lookup.insert(essence.id, essence);
    }

    pub fn register_form(&mut self, form: Form) {
        self.form_lookup.insert(form.id, form);
    }

    pub fn register_reaction(&mut self, reaction: impl Reaction + 'static) {
        self.reaction_lookup.insert(reaction.get_id(), Box::new(reaction));
    }

    fn run_reactions(&self) -> Vec<Product> {
        self.reaction_lookup
            .iter()
            .flat_map(|(_, reaction)| 
                self
                    .substances
                    .iter()
                    .flat_map(|substance| 
                        reaction.react(&self.environment, substance)
                    )
            )
            .collect()
    }

    pub fn simulate(&mut self, time: &Time) {
        self
            .run_reactions()
            .drain(0..)
            .for_each(|p| {
                if let Product::Exotherm(delta) = p {
                    self.environment.temperature.add(delta)
                }
            });

        self.environment.time.advance(&time)
    }
    
    pub fn get_essence<'a>(&'a self, id: u16) -> Option<&'a Essence> {
        self.essence_lookup.get(&id)
    }

    pub fn get_form(&self, id: u16) -> Option<&Form> {
        self.form_lookup.get(&id)
    }

    pub fn add_substance(&mut self, substance: Substance) {
        self.substances.push(substance);
    }
}