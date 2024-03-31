use crate::reaction::Reaction;

pub struct FormTransition;

impl Reaction for FormTransition {
    fn react(&self, _engine: &crate::engine::Essentia) -> Vec::<crate::reaction::Product> {
        vec![]
    }
}