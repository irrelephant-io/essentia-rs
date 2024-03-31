use crate::reaction::Reaction;

pub struct Precipitation;

impl Reaction for Precipitation {
    fn react(&self, _engine: &crate::engine::Essentia) -> Vec::<crate::reaction::Product> {
        vec![]
    }
}