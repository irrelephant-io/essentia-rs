use std::sync::atomic::{AtomicU16, Ordering};

use super::physics::SpecificHeatCapacity;
use crate::physics::{PhaseGraph, PhaseGraphBuilder, Solubility, SolubilityBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EssenceId {
    id: u16,
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

#[non_exhaustive]
pub struct Essence {
    pub id: EssenceId,
    pub name: String,
    pub heat_capacity: SpecificHeatCapacity,
    pub phase_graph: Option<PhaseGraph>,
    pub solubility: Option<Solubility>,
}

#[derive(Default)]
enum IdGenerationStrategy {
    #[default]
    Auto,
    Specific(EssenceId),
}

static ESSENCE_COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Default)]
pub struct EssenceBuilder {
    name: String,
    heat_capacity: SpecificHeatCapacity,
    id_generation: IdGenerationStrategy,
    phase_graph: Option<PhaseGraph>,
    solubility: Option<Solubility>,
}

impl EssenceBuilder {
    pub fn build(self) -> Essence {
        Essence {
            name: self.name,
            id: match self.id_generation {
                IdGenerationStrategy::Auto => ESSENCE_COUNTER.fetch_add(1, Ordering::SeqCst).into(),
                IdGenerationStrategy::Specific(id) => {
                    ESSENCE_COUNTER
                        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
                            Some(u16::max(current, id.into()))
                        })
                        .unwrap();

                    id
                }
            },
            phase_graph: self.phase_graph,
            heat_capacity: self.heat_capacity,
            solubility: self.solubility,
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

    pub fn with_phase_transitions(mut self, builder_fn: impl Fn(&mut PhaseGraphBuilder)) -> Self {
        let mut builder = PhaseGraphBuilder::default();
        builder_fn(&mut builder);
        self.phase_graph = Some(builder.build());
        self
    }

    pub fn with_solubility(
        mut self,
        builder_fn: impl FnOnce(SolubilityBuilder) -> Solubility,
    ) -> Self {
        let builder = SolubilityBuilder;
        self.solubility = Some(builder_fn(builder));
        self
    }
}
