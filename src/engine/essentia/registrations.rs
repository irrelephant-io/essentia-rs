use crate::abstractions::{
    Essence,
    Form,
    Substance,
    reaction::Reaction
};

impl super::Essentia {
    pub fn register_essence(&mut self, essence: Essence) {
        self.essence_lookup.insert(essence.id, essence);
    }

    pub fn register_form(&mut self, form: Form) {
        self.form_lookup.insert(form.id, form);
    }

    pub fn register_reaction(&mut self, reaction: impl Reaction + 'static) {
        self.reaction_lookup.insert(reaction.get_id(), Box::new(reaction));
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