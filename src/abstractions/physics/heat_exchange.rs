use substance::SubstanceData;

use crate::abstractions::substance;
use crate::engine::Essentia;

use super::HeatCapacity;

fn get_substance_heat_capacity(substance: &SubstanceData, engine: &Essentia) -> HeatCapacity {
    let essense = engine.get_essence(substance.essence_id).unwrap();
    HeatCapacity::from_specific(substance.quantity, essense.heat_capacity)
}

pub fn get_heat_capacity(engine: &Essentia) -> HeatCapacity {
    engine
        .get_all()
        .map(|substance| {
            get_substance_heat_capacity(&substance, engine)
        })
        .sum::<HeatCapacity>()
}
