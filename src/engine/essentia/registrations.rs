use crate::abstractions::{
    reaction::Reaction, Essence, Form, Substance
};

impl super::Essentia {
    pub fn register_essence(&mut self, essence: Essence) {
        self.essence_lookup.insert(essence.id, essence);
    }

    pub fn register_form(&mut self, form: Form) {
        self.form_lookup.insert(form.id, form);
    }

    pub fn register_reaction(&mut self, reaction: Box<dyn Reaction>) {
        self.reactions.push(reaction);
    }

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