use std::collections::HashMap;

use crate::{
    abstractions::{Environment, Essence, Form, Substance, SubstanceId},
    physics::{HeatCapacity, TimeSpan},
    EssenceId, FormId,
};

pub struct Essentia {
    _private_ctor: (),

    pub environment: Environment,
    pub heat_capacity: HeatCapacity,
    pub delta_time: TimeSpan,
    pub is_in_equilibrium: bool,

    substances: HashMap<SubstanceId, Substance>,
    essence_lookup: HashMap<EssenceId, Essence>,
    form_lookup: HashMap<FormId, Form>,
    reactions: ReactionLookup,
}

unsafe impl Send for Essentia {}
unsafe impl Sync for Essentia {}

impl Essentia {
    pub fn get_essence(&self, id: EssenceId) -> Option<&Essence> {
        self.essence_lookup.get(&id)
    }

    pub fn get_form(&self, id: FormId) -> Option<&Form> {
        self.form_lookup.get(&id)
    }

    pub fn add_substance(&mut self, to_add: Substance) {
        let to_merge = self.extract_matching(to_add.get_essence(), to_add.get_form());
        if let Some(to_merge) = to_merge {
            match (to_merge, to_add) {
                (Substance::Free(id, mut to_merge_data), Substance::Free(_, to_add)) => {
                    to_merge_data.quantity += to_add.quantity;
                    self.substances
                        .insert(id, Substance::Free(id, to_merge_data));
                }
                (Substance::Solution(id, mut to_merge_data, _), Substance::Free(_, to_add)) => {
                    to_merge_data.quantity += to_add.quantity;
                    self.substances
                        .insert(id, Substance::Free(id, to_merge_data));
                }
                (
                    Substance::Free(id, mut data_to_merge),
                    Substance::Solution(_, data_to_add, to_add_solutes),
                ) => {
                    data_to_merge.quantity += data_to_add.quantity;
                    let new_substance = Substance::Solution(id, data_to_merge, to_add_solutes);
                    self.substances.insert(id, new_substance);
                }
                (
                    Substance::Solution(id, mut data_to_merge, mut to_merge_solutes),
                    Substance::Solution(_, data_to_add, to_add_solutes),
                ) => {
                    data_to_merge.quantity += data_to_add.quantity;
                    for (essence, quantity) in to_add_solutes {
                        to_merge_solutes
                            .entry(essence)
                            .and_modify(|q| {
                                *q += quantity;
                            })
                            .or_insert(quantity);
                    }
                    let new_substance = Substance::Solution(id, data_to_merge, to_merge_solutes);
                    self.substances.insert(id, new_substance);
                }
            }
        } else {
            let id = to_add.get_substance();
            self.substances.insert(id, to_add);
        }
    }
}

mod reactions;

// Contains engine simulation methods.
mod simulation;

// Contains code for querying system's contents
mod querying;

// Contains code to construct an instance of an engine
mod builder;
pub use builder::EssentiaBuilder;

use self::reactions::ReactionLookup;
