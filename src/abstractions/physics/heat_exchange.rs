use crate::engine::Essentia;
use crate::Substance;

use super::HeatCapacity;

fn get_substance_heat_capacity(substance: &Substance, engine: &Essentia) -> HeatCapacity {
    let essense = engine.get_essence(substance.get_essence()).unwrap();
    HeatCapacity::from_specific(substance.get_quantity(), essense.heat_capacity)
}

pub fn get_heat_capacity(engine: &Essentia) -> HeatCapacity {
    engine
        .iter_all()
        .map(|substance| get_substance_heat_capacity(substance, engine))
        .sum::<HeatCapacity>()
}
