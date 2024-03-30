use substance::SubstanceData;

use crate::abstractions::{substance, Substance};
use crate::engine::Essentia;
use crate::abstractions::physics::{Energy, Temperature};

use super::HeatCapacity;

fn get_substance_heat_capacity(substance: &SubstanceData, engine: &Essentia) -> HeatCapacity {
    let essense = engine.get_essence(substance.essence_id).unwrap();
    HeatCapacity::from_specific(substance.quantity, essense.heat_capacity)
}

pub fn get_delta_temp(engine: &Essentia, e: Energy) -> Temperature {
    let total_cap = engine.substances
        .iter()
        .map(|substance| {
            match substance {
                Substance::Normal(substance) => {
                    get_substance_heat_capacity(&substance, engine)
                }
                Substance::Solution(solvent, solution) => {
                    get_substance_heat_capacity(&solvent, engine)
                    +
                    get_substance_heat_capacity(&solution, engine)
                }
            }
        })
        .sum::<HeatCapacity>();

    total_cap.get_delta_temp(&e)
}

