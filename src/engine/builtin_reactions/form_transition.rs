use crate::reaction::{Reaction, Product};
use crate::engine::ReactionContext;

pub struct FormTransition;

impl Reaction for FormTransition {
    fn react(
        &self,
        _context: &ReactionContext
    ) -> Vec::<Product> {
        vec![]
    }
    
    // We want the form transitions to occur at the very end of the
    // simulation pipeline
    fn get_priority(&self) -> u8 { u8::MAX }
}