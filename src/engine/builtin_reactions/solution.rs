use crate::{engine::ReactionContext, reaction::{Reaction, Product}};

pub struct Solution;

impl Reaction for Solution {
    fn react(&self, _context: &ReactionContext) -> Vec::<Product> {
        vec![]
    }

    // Solubility is applied before the form transitions
    fn get_priority(&self) -> u8 { u8::MAX - 1 }
}