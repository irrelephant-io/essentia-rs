use std::{sync::atomic::{AtomicU16, Ordering}};

use super::physics::SpecificHeatCapacity;

pub struct Essence {
    pub id: u16,
    pub name: String,
    pub heat_capacity: SpecificHeatCapacity,
    
    _private_ctor: ()
}

#[derive(Default)]
enum IdGenerationStrategy {
    #[default]
    Auto,
    Specific(u16)
}

static ESSENCE_COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Default)]
pub struct EssenceBuilder {
    name: String,
    heat_capacity: SpecificHeatCapacity,
    id_generation: IdGenerationStrategy
}

impl EssenceBuilder {
    pub fn build(self) -> Essence {
        Essence {
            _private_ctor: (),
            name: self.name,
            id: match self.id_generation {
                IdGenerationStrategy::Auto => ESSENCE_COUNTER.fetch_add(1, Ordering::SeqCst),
                IdGenerationStrategy::Specific(id) => {
                    ESSENCE_COUNTER.fetch_update(
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                        |current| { Some(u16::max(current, id)) }
                    ).unwrap();

                    id
                }
            },
            heat_capacity: self.heat_capacity,
        }
    }

    pub fn with_custom_id(mut self, id: u16) -> Self {
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
}