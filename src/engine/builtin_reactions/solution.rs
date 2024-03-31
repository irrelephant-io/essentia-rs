use crate::reaction::Reaction;

pub struct Solution;

impl Reaction for Solution {
    fn react(&self, _engine: &crate::engine::Essentia) -> Vec::<crate::reaction::Product> {
        vec![]
    }
}