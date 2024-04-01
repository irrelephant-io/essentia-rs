use std::sync::atomic::{AtomicU16, Ordering};

use crate::physics::{PhaseGraph, PhaseGraphBuilder};
use super::physics::SpecificHeatCapacity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EssenceId {
    id: u16
}

impl From<u16> for EssenceId {
    fn from(value: u16) -> Self {
        EssenceId { id: value }
    }
}

impl From<EssenceId> for u16 {
    fn from(value: EssenceId) -> Self {
        value.id
    }
}

pub struct Essence {
    pub id: EssenceId,
    pub name: String,
    pub heat_capacity: SpecificHeatCapacity,
    pub phase_graph: Option<PhaseGraph>,
    
    _private_ctor: ()
}

#[derive(Default)]
enum IdGenerationStrategy {
    #[default]
    Auto,
    Specific(EssenceId)
}

static ESSENCE_COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Default)]
pub struct EssenceBuilder {
    name: String,
    heat_capacity: SpecificHeatCapacity,
    id_generation: IdGenerationStrategy,
    phase_graph: Option<PhaseGraph>
}

impl EssenceBuilder {
    pub fn build(self) -> Essence {
        Essence {
            _private_ctor: (),
            name: self.name,
            id: match self.id_generation {
                IdGenerationStrategy::Auto => ESSENCE_COUNTER.fetch_add(1, Ordering::SeqCst).into(),
                IdGenerationStrategy::Specific(id) => {
                    ESSENCE_COUNTER.fetch_update(
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                        |current| { Some(u16::max(current, id.into())) }
                    ).unwrap();

                    id
                }
            },
            phase_graph: self.phase_graph,
            heat_capacity: self.heat_capacity,
        }
    }

    pub fn with_custom_id(mut self, id: EssenceId) -> Self {
        self.id_generation = IdGenerationStrategy::Specific(id);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }

    pub fn with_specific_heat_capacity(mut self, capacity: SpecificHeatCapacity) -> Self {
        self.heat_capacity = capacity;
        self
    }
    
    pub fn with_phase_transitions(mut self, builder_fn: impl Fn(&mut PhaseGraphBuilder) -> ()) -> Self {
        let mut builder = PhaseGraphBuilder::default();
        builder_fn(&mut builder);
        self.phase_graph = Some(builder.build());
        self
    }
}