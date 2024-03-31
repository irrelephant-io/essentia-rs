use crate::reaction::{Reaction, Product};
use crate::engine::ReactionContext;

pub struct Precipitation;

impl Reaction for Precipitation {
    fn react(
        &self,
        _context: &ReactionContext
    ) -> Vec::<Product> {
        vec![]
    }
    
        // Solubility is applied before the form transitions
    fn get_priority(&self) -> u8 { u8::MAX - 1 }
}